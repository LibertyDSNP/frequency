use crate::{BlockNumberFor, Config, Message, MessagesV2, Pallet, SchemaId, LOG_TARGET};
use frame_support::{log, storage_alias, traits::OnRuntimeUpgrade};

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// Migration to v2 module
pub mod v2 {
	use super::*;
	use frame_support::{pallet_prelude::*, weights::Weight};

	/// A permanent storage for messages mapped by block number and schema id.
	/// - Keys: BlockNumber, Schema Id
	/// - Value: List of Messages
	#[storage_alias]
	pub(crate) type Messages<T: Config> = StorageDoubleMap<
		Pallet<T>,
		Twox64Concat,
		BlockNumberFor<T>,
		Twox64Concat,
		SchemaId,
		BoundedVec<
			Message<<T as crate::pallet::Config>::MessagesMaxPayloadSizeBytes>,
			<T as crate::pallet::Config>::MaxMessagesPerBlock,
		>,
		ValueQuery,
	>;
	/// migration to v2 implementation
	pub struct MigrateToV2<T>(PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
		fn on_runtime_upgrade() -> Weight {
			migrate_to_v2::<T>()
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			log::info!(target: LOG_TARGET, "Running pre_upgrade...");
			let mut count = 0u32;
			for (_, _, v) in v2::Messages::<T>::iter() {
				count += v.len() as u32;
			}
			log::info!(target: LOG_TARGET, "Finish pre_upgrade for {:?}", count);
			Ok(count.encode())
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(state: Vec<u8>) -> Result<(), TryRuntimeError> {
			log::info!(target: LOG_TARGET, "Running post_upgrade...");

			let old_count: u32 = Decode::decode(&mut state.as_slice()).expect(
				"the state parameter should be something that was generated by pre_upgrade",
			);

			let count = Messages::<T>::iter().count();
			let moved_count = MessagesV2::<T>::iter().count();

			log::info!(target: LOG_TARGET, "Finish post_upgrade for {:?}", moved_count);
			let onchain_version = Pallet::<T>::on_chain_storage_version();

			assert_eq!(count, 0usize);
			assert_eq!(moved_count, old_count as usize);
			assert_eq!(onchain_version, crate::pallet::STORAGE_VERSION);
			Ok(())
		}
	}
	/// migrating to v2
	pub fn migrate_to_v2<T: Config>() -> Weight {
		log::info!(target: LOG_TARGET, "Running storage migration...");
		let onchain_version = Pallet::<T>::on_chain_storage_version();
		let current_version = Pallet::<T>::current_storage_version();
		log::info!(target: LOG_TARGET, "onchain_version= {:?}, current_version={:?}", onchain_version, current_version);

		if onchain_version < 2 {
			let mut reads = 1u64;
			let mut writes = 0u64;
			for (block_number, schema_id, messages) in v2::Messages::<T>::drain() {
				for (idx, message) in messages.iter().enumerate() {
					MessagesV2::<T>::insert((block_number, schema_id, idx as u16), message);
				}
				reads += 1;
				writes += messages.len() as u64 + 1;
			}

			// Set storage version to `2`.
			StorageVersion::new(2).put::<Pallet<T>>();
			writes += 1;

			log::info!(target: LOG_TARGET, "Storage migrated to version 2  read={:?}, write={:?}", reads, writes);
			T::DbWeight::get().reads_writes(reads, writes)
		} else {
			log::info!(
				target: LOG_TARGET,
				"Migration did not execute. This probably should be removed onchain:{:?}, current:{:?}",
				onchain_version,
				current_version
			);
			T::DbWeight::get().reads(1)
		}
	}
}
