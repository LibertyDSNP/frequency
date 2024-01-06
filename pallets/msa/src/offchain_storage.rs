use crate::{pallet::MSAEventCount, Config, Event, Pallet, PublicKeyToMsaId};
pub use common_primitives::msa::MessageSourceId;
/// Offchain Storage for MSA
use common_primitives::offchain::{
	self as offchain_common, get_msa_account_lock_name, get_msa_account_storage_key_name,
	LockStatus, MSA_ACCOUNT_LOCK_TIMEOUT_EXPIRATION,
};
use frame_support::RuntimeDebugNoBound;
use frame_system::pallet_prelude::BlockNumberFor;
use parity_scale_codec::{Decode, Encode};
use sp_core::serde::Deserialize;
extern crate alloc;
use alloc::string::String;
use sp_runtime::{
	offchain::{
		storage::StorageValueRef,
		storage_lock::{BlockAndTime, StorageLock, Time},
		Duration,
	},
	traits::One,
	Saturating,
};
use sp_std::{collections::btree_map::BTreeMap, fmt::Debug, vec, vec::Vec};

/// Block event storage prefix
const BLOCK_EVENT_KEY: &[u8] = b"frequency::block_event::msa::";

/// Offchain index for MSA events count
const BLOCK_EVENT_COUNT_KEY: &[u8] = b"frequency::block_event::msa::count::";

/// Lock expiration timeout in in milli-seconds for initial data import msa pallet
const MSA_INITIAL_LOCK_TIMEOUT_EXPIRATION: u64 = 3000;

/// Lock expiration block for initial data import msa pallet
const MSA_INITIAL_LOCK_BLOCK_EXPIRATION: u32 = 20;

/// Lock name for initial data index for msa pallet
const MSA_INITIAL_LOCK_NAME: &[u8; 28] = b"Msa::ofw::initial-index-lock";

/// storage name for initial data import storage
const MSA_INITIAL_INDEXED_STORAGE_NAME: &[u8; 25] = b"Msa::ofw::initial-indexed";

/// Lock name for last processed block number events
const LAST_PROCESSED_BLOCK_LOCK_NAME: &[u8; 35] = b"Msa::ofw::last-processed-block-lock";

/// lst processed block storage name
const LAST_PROCESSED_BLOCK_STORAGE_NAME: &[u8; 30] = b"Msa::ofw::last-processed-block";

/// Lock expiration timeout in in milli-seconds for last processed block
const LAST_PROCESSED_BLOCK_LOCK_TIMEOUT_EXPIRATION: u64 = 5000;

/// Lock expiration block for initial data import msa pallet
const LAST_PROCESSED_BLOCK_LOCK_BLOCK_EXPIRATION: u32 = 2;

/// number of previous blocks to check to mitigate offchain worker skips processing any block
const NUMBER_OF_PREVIOUS_BLOCKS_TO_CHECK: u32 = 5u32;

/// number of blocks to explore when trying to find the block number from block hash
const NUMBER_OF_BLOCKS_TO_EXPLORE: u32 = 1000;

/// HTTP request deadline in milliseconds
const HTTP_REQUEST_DEADLINE_MILLISECONDS: u64 = 2000;

/// LOCAL RPC URL and port
/// warning: this should be updated if rpc port is set to anything different from 9944
const RPC_REQUEST_URL: &'static str = "http://localhost:9944";

/// stores the event into offchain DB using offchain indexing
pub fn offchain_index_event<T: Config>(event: &Event<T>, msa_id: MessageSourceId) {
	if let Some(event) = IndexedEvent::map(event, msa_id) {
		let block_number: u32 =
			<frame_system::Pallet<T>>::block_number().try_into().unwrap_or_default();
		let current_event_count: u16 = <MSAEventCount<T>>::get().saturating_add(1);
		<MSAEventCount<T>>::put(current_event_count);
		let event_key = [
			BLOCK_EVENT_KEY,
			block_number.encode().as_slice(),
			current_event_count.encode().as_slice(),
		]
		.concat();
		// set the event in offchain storage
		set_offchain_index(event_key.encode().as_slice(), event);

		let count_key = [BLOCK_EVENT_COUNT_KEY, block_number.encode().as_slice()].concat();
		// Set the latest count of event in current block
		set_offchain_index(count_key.encode().as_slice(), current_event_count);
	}
}

/// Offchain indexes all existing data in chain state
/// returns the LockStatus
pub fn offchain_index_initial_state<T: Config>(block_number: BlockNumberFor<T>) -> LockStatus {
	let mut lock = StorageLock::<BlockAndTime<Pallet<T>>>::with_block_and_time_deadline(
		MSA_INITIAL_LOCK_NAME,
		MSA_INITIAL_LOCK_BLOCK_EXPIRATION,
		Duration::from_millis(MSA_INITIAL_LOCK_TIMEOUT_EXPIRATION),
	);
	if let Ok(mut guard) = lock.try_lock() {
		let processed_storage = StorageValueRef::persistent(MSA_INITIAL_INDEXED_STORAGE_NAME);
		let is_initial_indexed = processed_storage.get::<bool>().unwrap_or(None);

		if !is_initial_indexed.unwrap_or_default() {
			log::info!("Msa::ofw::initial-indexed is {:?}", is_initial_indexed);

			// setting last processed block so we can start indexing from that block after
			// initial index is done
			init_last_processed_block::<T>(block_number);

			let mut counter = 0u64;
			for (account_id, msa_id) in PublicKeyToMsaId::<T>::iter() {
				add_key_to_offchain_msa::<T>(account_id, msa_id);

				// extend the initial index lock
				counter += 1;
				if counter % 1000 == 0 {
					log::info!("Added {} more keys!", counter);
					if guard.extend_lock().is_err() {
						log::warn!("lock is expired in block {:?}", block_number);
						return LockStatus::Released
					}
				}
			}

			processed_storage.set(&true);
			log::info!("Finished adding {} keys!", counter);
		}
	} else {
		return LockStatus::Locked
	};
	LockStatus::Released
}

/// apply offchain event into offchain DB
pub fn apply_offchain_events<T: Config>(block_number: BlockNumberFor<T>) {
	let mut lock = StorageLock::<BlockAndTime<Pallet<T>>>::with_block_and_time_deadline(
		LAST_PROCESSED_BLOCK_LOCK_NAME,
		LAST_PROCESSED_BLOCK_LOCK_BLOCK_EXPIRATION,
		Duration::from_millis(LAST_PROCESSED_BLOCK_LOCK_TIMEOUT_EXPIRATION),
	);

	if let Ok(mut guard) = lock.try_lock() {
		log::info!("processing events in {:?}", block_number);

		let last_processed_block_storage =
			StorageValueRef::persistent(LAST_PROCESSED_BLOCK_STORAGE_NAME);
		let mut start_block_number = last_processed_block_storage
			.get::<BlockNumberFor<T>>()
			.unwrap_or(Some(
				block_number
					.saturating_sub(BlockNumberFor::<T>::from(NUMBER_OF_PREVIOUS_BLOCKS_TO_CHECK)),
			))
			.unwrap_or_default();

		// since this is the last processed block number we already processed it and starting from the next one
		start_block_number += BlockNumberFor::<T>::one();
		while start_block_number <= block_number {
			if reverse_map_msa_keys::<T>(start_block_number) {
				if guard.extend_lock().is_err() {
					log::warn!("last processed block lock is expired in block {:?}", block_number);
					break
				}
			}
			last_processed_block_storage.set(&start_block_number);
			start_block_number += BlockNumberFor::<T>::one();
		}
	} else {
		log::info!("skipping processing events on {:?} due to existing lock!", block_number);
	};
}

/// Set offchain index value, used to store MSA Events to be process by offchain worker
fn set_offchain_index<V>(key: &[u8], value: V)
where
	V: Encode + Clone + Decode + Eq + Debug,
{
	offchain_common::set_offchain_index_value(key, value.encode().as_slice());
}

/// Get offchain index value, used to store MSA Events to be process by offchain worker
fn get_offchain_index<V>(key: &[u8]) -> Option<V>
where
	V: Encode + Clone + Decode + Eq + Debug,
{
	let value = offchain_common::get_index_value::<V>(key);
	value.unwrap_or_else(|e| {
		log::error!("Error getting offchain index value: {:?}", e);
		None
	})
}

/// Offchain indexed compatible Event type
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebugNoBound)]
enum IndexedEvent<T: Config> {
	/// A new Message Service Account was created with a new MessageSourceId
	IndexedMsaCreated {
		/// The MSA for the Event
		msa_id: MessageSourceId,

		/// The key added to the MSA
		key: T::AccountId,
	},
	/// An AccountId has been associated with a MessageSourceId
	IndexedPublicKeyAdded {
		/// The MSA for the Event
		msa_id: MessageSourceId,

		/// The key added to the MSA
		key: T::AccountId,
	},
	/// An AccountId had all permissions revoked from its MessageSourceId
	IndexedPublicKeyDeleted {
		/// The MSA for the Event
		msa_id: MessageSourceId,
		/// The key no longer approved for the associated MSA
		key: T::AccountId,
	},
}

impl<T: Config> IndexedEvent<T> {
	/// maps a pallet event to indexed event type
	pub fn map(event: &Event<T>, msa_id: MessageSourceId) -> Option<Self> {
		match event {
			Event::MsaCreated { msa_id, key } =>
				Some(Self::IndexedMsaCreated { msa_id: *msa_id, key: key.clone() }),
			Event::PublicKeyAdded { msa_id, key } =>
				Some(Self::IndexedPublicKeyAdded { msa_id: *msa_id, key: key.clone() }),
			Event::PublicKeyDeleted { key } =>
				Some(Self::IndexedPublicKeyDeleted { msa_id, key: key.clone() }),
			_ => None,
		}
	}
}

/// Initializes the last_process_block value in offchain DB
fn init_last_processed_block<T: Config>(current_block_number: BlockNumberFor<T>) {
	let mut last_processed_block_lock = StorageLock::<'_, Time>::with_deadline(
		LAST_PROCESSED_BLOCK_LOCK_NAME,
		Duration::from_millis(LAST_PROCESSED_BLOCK_LOCK_TIMEOUT_EXPIRATION),
	);
	let _ = last_processed_block_lock.lock();
	let last_processed_block_storage =
		StorageValueRef::persistent(LAST_PROCESSED_BLOCK_STORAGE_NAME);

	// setting current_block-1 as the last processed so that we start indexing from current_block
	last_processed_block_storage
		.set(&current_block_number.saturating_sub(BlockNumberFor::<T>::one()));
}

/// Add a key into MSA offchain DB
fn add_key_to_offchain_msa<T: Config>(key: T::AccountId, msa_id: MessageSourceId) {
	let msa_lock_name = get_msa_account_lock_name(msa_id);
	let mut msa_lock = StorageLock::<'_, Time>::with_deadline(
		&msa_lock_name,
		Duration::from_millis(MSA_ACCOUNT_LOCK_TIMEOUT_EXPIRATION),
	);
	let _ = msa_lock.lock();
	let msa_storage_name = get_msa_account_storage_key_name(msa_id);
	let msa_storage = StorageValueRef::persistent(&msa_storage_name);

	let mut msa_keys =
		msa_storage.get::<Vec<T::AccountId>>().unwrap_or(None).unwrap_or(Vec::default());

	if !msa_keys.contains(&key) {
		msa_keys.push(key.clone());
		msa_storage.set(&msa_keys);
	} else {
		log::warn!("{:?} already added!", &key);
	}
}

fn read_offchain_events<T: Config>(block_number: BlockNumberFor<T>) -> Vec<IndexedEvent<T>> {
	let current_block: u32 = block_number.try_into().unwrap_or_default();
	let count_key = [BLOCK_EVENT_COUNT_KEY, current_block.encode().as_slice()].concat();
	let optional_event_count = get_offchain_index::<u16>(count_key.encode().as_slice());
	let mut events = vec![];
	let event_count = optional_event_count.unwrap_or_default();

	for i in 1..=event_count {
		let key =
			[BLOCK_EVENT_KEY, block_number.encode().as_slice(), i.encode().as_slice()].concat();
		let optional_decoded_event = get_offchain_index::<IndexedEvent<T>>(key.encode().as_slice());
		if let Some(decoded_event) = optional_decoded_event {
			events.push(decoded_event);
		} else {
			log::warn!(
				"Indexed event does not exist for block {:?} and number {}",
				current_block,
				i
			);
		}
	}
	events
}

/// cleans the events from offchain storage
fn clean_offchain_events<T: Config>(block_number: BlockNumberFor<T>) {
	let current_block: u32 = block_number.try_into().unwrap_or_default();
	let count_key = [BLOCK_EVENT_COUNT_KEY, current_block.encode().as_slice()].concat();
	let optional_event_count = get_offchain_index::<u16>(count_key.encode().as_slice());
	let event_count = optional_event_count.unwrap_or_default();
	for i in 1..=event_count {
		let key =
			[BLOCK_EVENT_KEY, block_number.encode().as_slice(), i.encode().as_slice()].concat();

		offchain_common::remove_offchain_index_value(key.encode().as_slice());
	}
	offchain_common::remove_offchain_index_value(count_key.encode().as_slice());
}

/// offchain worker callback for indexing msa keys
/// return true if there are events and false if not
fn reverse_map_msa_keys<T: Config>(block_number: BlockNumberFor<T>) -> bool {
	// read the events indexed for the current block
	let events_to_process: Vec<IndexedEvent<T>> = read_offchain_events(block_number);
	let events_exists = !events_to_process.is_empty();
	if events_exists {
		log::info!("found {} indexed events for block {:?}", events_to_process.len(), block_number);
	}

	// collect a replay of all events by MSA id
	let mut events_by_msa_id: BTreeMap<MessageSourceId, Vec<IndexedEvent<T>>> = BTreeMap::new();

	// collect relevant events
	for event in events_to_process {
		match event {
			IndexedEvent::IndexedPublicKeyAdded { msa_id, .. } |
			IndexedEvent::IndexedMsaCreated { msa_id, .. } => {
				let events = events_by_msa_id.entry(msa_id).or_default();
				events.push(event);
			},
			IndexedEvent::IndexedPublicKeyDeleted { msa_id, .. } => {
				let events = events_by_msa_id.entry(msa_id).or_default();
				events.push(event);
			},
		}
	}

	// process and save to offchain db
	for (msa_id, events) in events_by_msa_id {
		if !events.is_empty() {
			process_offchain_events(msa_id, events);
		}
	}

	if events_exists {
		clean_offchain_events::<T>(block_number);
	}

	events_exists
}

fn process_offchain_events<T: Config>(msa_id: MessageSourceId, events: Vec<IndexedEvent<T>>) {
	// Lock will specifically prevent multiple offchain workers from
	// processing the same msa events at the same time
	// let lock_status = offchain_common::lock(b"msa::", msa_id.encode().as_slice(), || {
	let msa_lock_name = get_msa_account_lock_name(msa_id);
	let mut msa_lock = StorageLock::<'_, Time>::with_deadline(
		&msa_lock_name,
		Duration::from_millis(MSA_ACCOUNT_LOCK_TIMEOUT_EXPIRATION),
	);
	let _ = msa_lock.lock();
	let msa_storage_name = get_msa_account_storage_key_name(msa_id);
	let msa_storage = StorageValueRef::persistent(&msa_storage_name);

	let mut msa_keys =
		msa_storage.get::<Vec<T::AccountId>>().unwrap_or(None).unwrap_or(Vec::default());

	for event in events {
		match &event {
			IndexedEvent::IndexedPublicKeyAdded { key, .. } |
			IndexedEvent::IndexedMsaCreated { key, .. } =>
				if !msa_keys.contains(key) {
					msa_keys.push(key.clone());
				} else {
					log::warn!("{:?} already added!", key);
				},
			IndexedEvent::IndexedPublicKeyDeleted { key, .. } =>
				if msa_keys.contains(key) {
					msa_keys.retain(|k| k != key);
				} else {
					log::warn!("{:?} already removed!", key);
				},
		}
	}
	msa_storage.set(&msa_keys);
}

#[derive(Deserialize, Encode, Decode, Default, Debug)]
struct FinalizedResult {
	result: String,
}

/// get finalized
pub fn fetch_finalized<T: Config>() -> Result<T::Hash, sp_runtime::offchain::http::Error> {
	// We want to keep the offchain worker execution time reasonable, so we set a hard-coded
	// deadline to 2s to complete the external call.
	// You can also wait indefinitely for the response, however you may still get a timeout
	// coming from the host machine.
	let deadline =
		sp_io::offchain::timestamp().add(Duration::from_millis(HTTP_REQUEST_DEADLINE_MILLISECONDS));
	let body = vec![b"{\"id\": 10, \"jsonrpc\": \"2.0\", \"method\": \"chain_getFinalizedHead\", \"params\": []}"];
	let request = sp_runtime::offchain::http::Request::post(RPC_REQUEST_URL, body);
	let pending = request
		.add_header("Content-Type", "application/json")
		.deadline(deadline)
		.send()
		.map_err(|_| sp_runtime::offchain::http::Error::IoError)?;

	let response = pending
		.try_wait(deadline)
		.map_err(|_| sp_runtime::offchain::http::Error::DeadlineReached)??;
	// Let's check the status code before we proceed to reading the response.
	if response.code != 200 {
		log::warn!("Unexpected status code: {}", response.code);
		return Err(sp_runtime::offchain::http::Error::Unknown)
	}

	// Next we want to fully read the response body and collect it to a vector of bytes.
	// Note that the return object allows you to read the body in chunks as well
	// with a way to control the deadline.
	let body = response.body().collect::<Vec<u8>>();

	// Create a str slice from the body.
	let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
		log::warn!("No UTF8 body");
		sp_runtime::offchain::http::Error::Unknown
	})?;

	log::debug!("{}", body_str);
	let gh_info: FinalizedResult =
		serde_json::from_str(body_str).map_err(|_| sp_runtime::offchain::http::Error::Unknown)?;

	let decoded_from_hex = hex::decode(&gh_info.result[2..])
		.map_err(|_| sp_runtime::offchain::http::Error::Unknown)?;

	let val = T::Hash::decode(&mut &decoded_from_hex[..])
		.map_err(|_| sp_runtime::offchain::http::Error::Unknown)?;
	Ok(val)
}

/// iterates on imported blocks to find the block_number from block_hash
pub fn find_block_number_from_block_hash<T: Config>(
	block_hash: T::Hash,
	current_block: BlockNumberFor<T>,
) -> Option<BlockNumberFor<T>> {
	let mut finalized_block_number = None;
	let mut current_block_number = current_block;
	let last_block_number =
		current_block.saturating_sub(BlockNumberFor::<T>::from(NUMBER_OF_BLOCKS_TO_EXPLORE));
	while current_block_number > last_block_number {
		if block_hash == frame_system::Pallet::<T>::block_hash(current_block_number) {
			finalized_block_number = Some(current_block_number);
			break
		}
		current_block_number.saturating_dec();
	}
	return finalized_block_number
}
