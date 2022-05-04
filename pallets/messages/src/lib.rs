//! # Messages pallet
//! A pallet for storing messages.
//!
//! ## Overview
//!
//! This pallet contains functionality for storing, retrieving and eventually removing messages for
//! registered schemas on chain.
//!
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

mod types;

use frame_support::{dispatch::DispatchResult, ensure, pallet_prelude::Weight, traits::Get};
use sp_runtime::traits::One;
use sp_std::{collections::btree_map::BTreeMap, convert::TryInto, prelude::*};

pub use pallet::*;
pub use types::*;
pub use weights::*;

use common_primitives::messages::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use common_primitives::msa::AccountProvider;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// A type that will supply account related information
		type AccountProvider: AccountProvider<AccountId = Self::AccountId>;

		/// The maximum number of messages in a block
		#[pallet::constant]
		type MaxMessagesPerBlock: Get<u16>;

		/// The maximum size of a message [Byte]
		#[pallet::constant]
		type MaxMessageSizeInBytes: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_block_messages)]
	pub(super) type BlockMessages<T: Config> //TODO: convert Vec to BoundedVec
		= StorageValue<_, Vec<(Message<T::AccountId>, SchemaId)>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_messages)]
	pub(super) type Messages<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::BlockNumber,
		Twox64Concat,
		SchemaId,
		Vec<Message<T::AccountId>>,
		ValueQuery,
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Too many messages are added to existing block
		TooManyMessagesInBlock,
		/// Message size is too large
		TooLargeMessage,
		/// Invalid Pagination Request
		InvalidPaginationRequest,
		/// Type Conversion Overflow
		TypeConversionOverflow,
		/// Invalid Message Source Account
		InvalidMessageSourceAccount,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Messages are stored for a specified schema id and block number
		MessagesStored { schema_id: SchemaId, block_number: T::BlockNumber, count: u16 },
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(current: T::BlockNumber) -> Weight {
			let prev_block = current - T::BlockNumber::one();
			Self::move_messages_into_final_storage(prev_block)
			// TODO: add retention policy execution
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a message into storage
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `schema_id`: Registered schema id for current message
		/// - `message`: Serialized message data
		///
		/// Result is equivalent to the dispatched result.
		///
		/// # <weight>
		/// Execution weight
		/// # </weight>
		#[pallet::weight(T::WeightInfo::add(message.len() as u32, 1_000))]
		pub fn add(origin: OriginFor<T>, schema_id: SchemaId, message: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				message.len() < T::MaxMessageSizeInBytes::get().try_into().unwrap(),
				Error::<T>::TooLargeMessage
			);

			let msa_id = T::AccountProvider::get_msa_id(&who);
			ensure!(msa_id.is_some(), Error::<T>::InvalidMessageSourceAccount);

			// TODO: validate schema existence and validity from schema pallet

			let mut block_messages = <BlockMessages<T>>::get();
			let current_size: u16 = block_messages
				.len()
				.try_into()
				.map_err(|_| Error::<T>::TypeConversionOverflow)?;
			ensure!(
				current_size < T::MaxMessagesPerBlock::get(),
				Error::<T>::TooManyMessagesInBlock
			);

			let m = Message {
				data: message,
				signer: who,
				index: current_size,
				msa_id: msa_id.unwrap(),
			};
			block_messages.push((m, schema_id));
			<BlockMessages<T>>::set(block_messages);
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Moves messages from temporary storage `BlockMessages` into final storage `Messages`
	/// and calculates execution weight
	///
	/// - `block_number`: Target Block Number
	///
	/// Returns execution weights
	fn move_messages_into_final_storage(block_number: T::BlockNumber) -> Weight {
		let mut map = BTreeMap::new();
		let block_messages = BlockMessages::<T>::get();
		let message_count = block_messages.len() as u32;
		let mut schema_count = 0u32;

		// grouping messages by schema_id
		for (m, schema_id) in block_messages {
			let list = map.entry(schema_id).or_insert(vec![]);
			list.push(m);
		}

		// insert into storage and create events
		for (schema_id, messages) in &map {
			let count = messages.len() as u16;
			Messages::<T>::insert(&block_number, schema_id, messages);
			Self::deposit_event(Event::MessagesStored {
				schema_id: *schema_id,
				block_number,
				count,
			});
			schema_count += 1;
		}

		BlockMessages::<T>::set(vec![]);
		T::WeightInfo::on_initialize(message_count, schema_count)
	}
}
