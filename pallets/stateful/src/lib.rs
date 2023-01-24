//! # Stateful Message Storage Pallet
//! The Stateful Message Storage pallet provides functionality for reading and writing messages
//! representing stateful data for which we are only ever interested in the latest state.
//!
//! ## Overview
//! For state transitions for which we only care about the latest state, Stateful Message Storage provides a way to store and retrieve such data
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
//! The Stateful Message Storage pallet provides functions for:
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

pub mod weights;

// use frame_system::{Config, Pallet};

use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
pub use pallet::*;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The maximum size of a page (in bytes) for an Itemized storage model
		#[pallet::constant]
		type MaxItemizedPageSizeBytes: Get<u32>;

		/// The maximum size of a page (in bytes) for a Paginated storage model
		#[pallet::constant]
		type MaxPaginatedPageSizeBytes: Get<u32>;

		/// The maximum size of a single item in an itemized storage model (in bytes)
		#[pallet::constant]
		type MaxItemizedBlobSizeBytes: Get<u32>;

		/// The maximum number of pages in a Paginated storage model
		#[pallet::constant]
		type MaxPaginatedPageCount: Get<u32>;
	}

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {}

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
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ItemAppended,
		ItemRemoved,
		PageAppended,
		PageUpdated,
		PageRemoved,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Stakes some amount of tokens to the network and generates Capacity.
		///
		/// ### Errors
		///
		/// - Returns Error::InsufficientBalance if the sender does not have free balance amount needed to stake.
		/// - Returns Error::InvalidTarget if attempting to stake to an invalid target.
		/// - Returns Error::InsufficientStakingAmount if attempting to stake an amount below the minimum amount.
		#[pallet::weight(T::WeightInfo::add_item(n: u32))]
		pub fn add_item(origin: OriginFor<T>, _payload: Vec<u8>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::remove_item())]
		pub fn remove_item(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::upsert_page())]
		pub fn upsert_page(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::remove_page())]
		pub fn remove_page(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}
