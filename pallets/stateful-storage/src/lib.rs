//! # Stateful Storage Pallet
//! The Stateful Storage pallet provides functionality for reading and writing messages
//! representing stateful data for which we are only ever interested in the latest state.
//!
//! ## Overview
//! For state transitions for which we only care about the latest state, Stateful Storage provides a way to store and retrieve such data
//! outside of the existing Announcement mechanism, which would require the latest state to be tracked using some kind of 3rd-party indexer.
//!
//! This pallet supports two models for storing stateful data:
//! 1. **Itemized:** Data is stored in a single **page** (max size: `MaxItemizedPageSizeBytes`) containing multiple items (max item size `MaxItemizedBlobSizeBytes`) of the associated schema.
//! Useful for schemas with a relative small item size and higher potential item count.
//! 2. **Paginated:** Data is stored in multiple **pages** of size `MaxPaginatedPageSizeBytes`, each containing a single item of the associated schema,
//! up to `MaxPaginatedPageCount`. Useful for schemas with a larger item size and smaller potential item count.
//!
//! ## Features
//! * Provide for storage of stateful data with flexible schemas on-chain.
//! * Data stored for one MSA does not have impact read/write access times or storage costs for any data stored for another MSA.
//! * High write throughput.
//! * High read throughput.
//! * Data race condition protection
//!
//! The Stateful Storage pallet provides functions for:
//!
//! - Appending items in an **itemized** model
//! - Removing items in an **itemized** model
//! - Upserting items in a **paginated** model
//! - Removing pages in a **paginated**  model
//!
//! ## Terminology
//! * **Item:** Data payload mapping to a defined schema
//! * **Page:** Block of on-chain data of a fixed size, containing one (Paginated model) or more (Itemized model) **items**.
//!

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
// #![deny(
// 	rustdoc::broken_intra_doc_links,
// 	rustdoc::missing_crate_level_docs,
// 	rustdoc::invalid_codeblock_attributes,
// 	missing_docs
// )]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(feature = "runtime-benchmarks")]
use common_primitives::benchmarks::{MsaBenchmarkHelper, SchemaBenchmarkHelper};

mod stateful_child_tree;
pub mod types;

pub mod weights;

use common_primitives::{
	msa::{DelegatorId, ProviderId},
	schema::PayloadLocation,
};
use frame_support::{dispatch::DispatchResult, ensure, traits::Get};
use sp_std::prelude::*;

pub use pallet::*;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::{
		stateful_child_tree::StatefulChildTree,
		types::{ItemAction, Page},
	};
	use common_primitives::{
		msa::{MessageSourceId, MsaLookup, MsaValidator, SchemaGrantValidator},
		schema::{SchemaId, SchemaProvider},
	};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// A type that will supply MSA related information
		type MsaInfoProvider: MsaLookup + MsaValidator<AccountId = Self::AccountId>;

		/// A type that will validate schema grants
		type SchemaGrantValidator: SchemaGrantValidator<Self::BlockNumber>;

		/// A type that will supply schema related information.
		type SchemaProvider: SchemaProvider<SchemaId>;

		/// The maximum size of a page (in bytes) for an Itemized storage model
		#[pallet::constant]
		type MaxItemizedPageSizeBytes: Get<u32> + Default;

		/// The maximum size of a page (in bytes) for a Paginated storage model
		#[pallet::constant]
		type MaxPaginatedPageSizeBytes: Get<u32>;

		/// The maximum size of a single item in an itemized storage model (in bytes)
		#[pallet::constant]
		type MaxItemizedBlobSizeBytes: Get<u32>;

		/// The maximum number of pages in a Paginated storage model
		#[pallet::constant]
		type MaxPaginatedPageCount: Get<u32>;

		/// The maximum number of actions in itemized actions
		#[pallet::constant]
		type MaxItemizedActionsCount: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type MsaBenchmarkHelper: MsaBenchmarkHelper<Self::AccountId>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type SchemaBenchmarkHelper: SchemaBenchmarkHelper;
	}

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		/// Item payload exceeds max item blob size
		ItemExceedsMaxBlobSizeBytes,

		/// Additional item unable to fit in item page
		ItemPageFull,

		/// Additional page would exceed maximum number of allowable pages
		PageCountOverflow,

		/// Page size exceeds max allowable page size
		PageExceedsMaxPageSizeBytes,

		/// Invalid SchemaId or Schema not found
		InvalidSchemaId,

		/// Invalid Message Source Account
		InvalidMessageSourceAccount,

		/// UnAuthorizedDelegate
		UnAuthorizedDelegate,

		/// Corrupted State
		CorruptedState,

		/// Invalid item action
		InvalidItemAction,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PageUpdated { msa_id: MessageSourceId, schema_id: SchemaId },
		PageRemoved { msa_id: MessageSourceId, schema_id: SchemaId },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::apply_item_actions( actions.len() as u32 ,
			actions.iter().fold(0, |acc, a| acc + match a {
				ItemAction::Add { data } => data.len() as u32,
				_ => 0,
			})
		))]
		pub fn apply_item_actions(
			origin: OriginFor<T>,
			state_owner_msa_id: MessageSourceId,
			#[pallet::compact] schema_id: SchemaId,
			actions: BoundedVec<ItemAction, T::MaxItemizedActionsCount>,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;
			ensure!(
				actions.as_slice().iter().all(|a| match a {
					ItemAction::Add { data } =>
						data.len() <= T::MaxItemizedBlobSizeBytes::get() as usize,
					_ => true,
				}),
				Error::<T>::ItemExceedsMaxBlobSizeBytes
			);

			let provider_msa_id = T::MsaInfoProvider::ensure_valid_msa_key(&provider_key)
				.map_err(|_| Error::<T>::InvalidMessageSourceAccount)?;

			let schema = T::SchemaProvider::get_schema_by_id(schema_id);
			ensure!(schema.is_some(), Error::<T>::InvalidSchemaId);
			ensure!(
				schema.unwrap().payload_location == PayloadLocation::Itemized,
				Error::<T>::InvalidSchemaId
			);

			let current_block = frame_system::Pallet::<T>::block_number();
			T::SchemaGrantValidator::ensure_valid_schema_grant(
				ProviderId(provider_msa_id),
				DelegatorId(state_owner_msa_id),
				schema_id,
				current_block,
			)
			.map_err(|_| Error::<T>::UnAuthorizedDelegate)?;

			let storage_key = &schema_id.encode()[..];
			let keys = vec![storage_key.to_vec()];
			let updated_page = StatefulChildTree::try_read::<Page<T::MaxItemizedPageSizeBytes>>(
				&state_owner_msa_id,
				&keys,
			)
			.map_err(|_| {
				log::warn!(
					"failed decoding Itemized msa={:?} schema_id={:?}",
					state_owner_msa_id,
					schema_id
				);
				Error::<T>::CorruptedState
			})?
			.unwrap_or_default()
			.apply_item_actions(&actions[..])
			.map_err(|_| Error::<T>::InvalidItemAction)?;

			match updated_page.is_empty() {
				true => {
					StatefulChildTree::kill::<Page<T::MaxItemizedPageSizeBytes>>(
						&state_owner_msa_id,
						&keys,
					);
					Self::deposit_event(Event::PageRemoved {
						msa_id: state_owner_msa_id,
						schema_id,
					});
				},
				false => {
					StatefulChildTree::write::<Page<T::MaxItemizedPageSizeBytes>>(
						&state_owner_msa_id,
						&keys,
						updated_page,
					);
					Self::deposit_event(Event::PageUpdated {
						msa_id: state_owner_msa_id,
						schema_id,
					});
				},
			};
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn upsert_page(_origin: OriginFor<T>, payload: Vec<u8>) -> DispatchResult {
			ensure!(
				payload.len() as u32 <= T::MaxPaginatedPageSizeBytes::get(),
				Error::<T>::PageExceedsMaxPageSizeBytes
			);
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn remove_page(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get the itemized page for the given msa_id and schema_id.
		pub fn get_itemized_page(
			msa_id: MessageSourceId,
			schema_id: SchemaId,
		) -> Page<T::MaxItemizedPageSizeBytes> {
			let storage_key = &schema_id.encode()[..];
			let keys = vec![storage_key.to_vec()];
			let page_response =
				StatefulChildTree::try_read::<Page<T::MaxItemizedPageSizeBytes>>(&msa_id, &keys)
					.map_or_else(|_| Page::default(), |page| page.unwrap_or_default());
			page_response
		}
	}
}

impl<T: Config> Pallet<T> {}
