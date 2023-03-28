//! # Handles Pallet
//! The Handles pallet provides functionality for generating user handles.
//!
//! - [Configuration: `Config`](Config)
//! - [Extrinsics: `Call`](Call)
//! - [Event Enum: `Event`](Event)
//! - [Error Enum: `Error`](Error)
//!
//! ## Overview
//!
//! ## Features
//!
//! * Handle creation and retirement
//! * Shuffled sequence generation for handle suffixes
//! * Homoglyph detection
//!
//! ## Terminology
//! - Handle
//! - Suffix
//! - PRNG

// Ensure we're `no_std` when compiling for WASM.
#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes,
	missing_docs
)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

// pub mod weights;

use sp_std::prelude::*;

use common_primitives::{
	handles::*,
	msa::{MessageSourceId, MsaLookup, MsaValidator},
	utils::wrap_binary_data,
};
use frame_support::{dispatch::DispatchResult, ensure, pallet_prelude::*, traits::Get};
use frame_system::pallet_prelude::*;
use numtoa::*;
pub use pallet::*;
use sp_core::crypto::AccountId32;
use sp_runtime::{
	traits::{Convert, Verify},
	MultiSignature,
};

// pub mod suffix;
// use suffix::SuffixGenerator;

// pub mod confusables;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		// type WeightInfo: WeightInfo;

		/// AccountId truncated to 32 bytes
		type ConvertIntoAccountId32: Convert<Self::AccountId, AccountId32>;

		/// A type that will supply MSA related information
		type MsaInfoProvider: MsaLookup + MsaValidator<AccountId = Self::AccountId>;

		/// The minimum suffix value
		#[pallet::constant]
		type HandleSuffixMin: Get<u32>;

		/// The maximum suffix value
		#[pallet::constant]
		type HandleSuffixMax: Get<u32>;
	}

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	/// - Keys: k1: Canonical base handle, k2: Suffix
	/// - Value: MSA id
	#[pallet::storage]
	#[pallet::getter(fn get_msa_id_for_canonical_and_suffix)]
	pub type CanonicalBaseHandleAndSuffixToMSAId<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Handle,
		Twox64Concat,
		HandleSuffix,
		MessageSourceId,
		OptionQuery,
	>;

	/// - Key: MSA id
	/// - Value: Display name
	#[pallet::storage]
	#[pallet::getter(fn get_display_name_for_msa_id)]
	pub type MSAIdToDisplayName<T: Config> =
		StorageMap<_, Twox64Concat, MessageSourceId, Handle, ValueQuery>;

	/// - Keys: Canonical base handle (no delimeter, no suffix)
	/// - Value: Cursor u16
	#[pallet::storage]
	#[pallet::getter(fn get_current_suffix_index_for_canonical_handle)]
	pub type CanonicalBaseHandleToSuffixIndex<T: Config> =
		StorageMap<_, Blake2_128Concat, Handle, SequenceIndex, OptionQuery>;

	#[derive(PartialEq, Eq)] // for testing
	#[pallet::error]
	pub enum Error<T> {
		/// Invalid handle encoding (should be UTF-8)
		InvalidHandleEncoding,
		/// Invalid handle byte length
		InvalidHandleByteLength,
		/// Invalid handle character length
		InvalidHandleCharacterLength,
		/// Suffixes exhausted
		SuffixesExhausted,
		/// Invalid MSA
		InvalidMessageSourceAccount,
		/// Cryptographic signature failed verification
		InvalidSignature,
		/// The MSA already has a handle
		MSAHandleExists,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted when a handle is created. [MSA id, full handle in UTF-8 bytes]
		HandleCreated {
			/// MSA id of handle owner
			msa_id: MessageSourceId,
			/// UTF-8 string in bytes
			handle: Handle,
		},
	}

	fn convert_to_canonical(handle_str: &str) -> codec::alloc::string::String {
		let mut normalized =
			unicode_security::skeleton(handle_str).collect::<codec::alloc::string::String>();
		normalized.make_ascii_lowercase();
		log::info!("normalized={}", normalized.clone());
		normalized
	}

	use core::hash::Hasher;
	use rand::{rngs::SmallRng, Rng, SeedableRng};
	use twox_hash::XxHash64;

	fn unique_sequence(
		min: usize,
		max: usize,
		rng: &mut SmallRng,
	) -> impl Iterator<Item = usize> + '_ {
		let mut indices: Vec<usize> = (min..=max).into_iter().collect();
		(0..max).rev().map(move |i| {
			// Make sure j is never higher than i
			// To keep the prior values stable (because it is reversed)
			let j = rng.gen_range(min..i + 1);
			indices.swap(i, j);
			indices[i]
		})
	}

	fn generate_suffix_for_canonical_handle(canonical_handle: &str, cursor: usize) -> u16 {
		log::info!("generate_suffix_for_canonical_handle() cursor={}", &cursor);

		let mut hasher = XxHash64::with_seed(0);
		sp_std::hash::Hash::hash(&canonical_handle, &mut hasher);
		let value_bytes: [u8; 4] = [0; 4];
		hasher.write(&value_bytes);
		let seed = hasher.finish();

		log::info!("seed={}", seed);

		let mut rng: SmallRng = SmallRng::seed_from_u64(seed);

		let mut sequence = unique_sequence(0, 10000, &mut rng);
		if let Some(suffix) = sequence.nth(cursor) {
			suffix.try_into().unwrap()
		} else {
			0
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get the next index into the shuffled suffix sequence for the specified canonical base handle
		///
		/// # Errors
		/// * [`Error::SuffixesExhausted`]
		///
		pub fn get_next_suffix_index_for_canonical_handle(
			canonical_handle: Handle,
		) -> Result<SequenceIndex, DispatchError> {
			let next;
			match Self::get_current_suffix_index_for_canonical_handle(canonical_handle) {
				None => {
					next = 0;
				},
				Some(current) => {
					next = current.checked_add(1).ok_or(Error::<T>::SuffixesExhausted)?;
				},
			}

			Ok(next)
		}

		/// Verify the `signature` was signed by `signer` on `payload` by a wallet
		/// Note the `wrap_binary_data` follows the Polkadot wallet pattern of wrapping with `<Byte>` tags.
		///
		/// # Errors
		/// * [`Error::InvalidSignature`]
		///
		pub fn verify_signature(
			signature: &MultiSignature,
			signer: &T::AccountId,
			payload: Vec<u8>,
		) -> DispatchResult {
			let key = T::ConvertIntoAccountId32::convert((*signer).clone());
			let wrapped_payload = wrap_binary_data(payload);

			ensure!(signature.verify(&wrapped_payload[..], &key), Error::<T>::InvalidSignature);

			Ok(())
		}
	}

	// EXTRINSICS

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claim handle
		///
		#[pallet::call_index(0)]
		#[pallet::weight(1000)]
		pub fn claim_handle(
			origin: OriginFor<T>,
			delegator_key: T::AccountId,
			proof: MultiSignature,
			payload: ClaimHandlePayload,
		) -> DispatchResult {
			log::info!("claim_handle()");
			let provider_key = ensure_signed(origin)?;
			T::MsaInfoProvider::ensure_valid_msa_key(&provider_key)
				.map_err(|_| Error::<T>::InvalidMessageSourceAccount)?;

			Self::verify_signature(&proof, &delegator_key, payload.encode())?;

			// Validation:  The delegator must already have a MSA id
			let delegator_msa_id = T::MsaInfoProvider::ensure_valid_msa_key(&delegator_key)
				.map_err(|_| Error::<T>::InvalidMessageSourceAccount)?;

			// Validation:  The MSA must not already have a handle associated with it
			ensure!(
				MSAIdToDisplayName::<T>::try_get(delegator_msa_id).is_err(),
				Error::<T>::MSAHandleExists
			);

			// Validation:  The base handle MUST be UTF-8 encoded.
			let base_handle_str = core::str::from_utf8(&payload.base_handle)
				.map_err(|_| Error::<T>::InvalidHandleEncoding)?;

			// Validation:  The handle length must be valid.
			// Note the count() can panic but won't because the base_handle is already bounded with a BoundedVec
			let len = base_handle_str.chars().count() as u32;
			log::info!("handle length={}", len);
			log::info!("handle={}", base_handle_str);

			// Validate byte length
			ensure!(
				len >= HANDLE_BASE_BYTES_MIN && len <= HANDLE_BASE_BYTES_MAX,
				Error::<T>::InvalidHandleByteLength
			);

			// Validate character length
			ensure!(
				len >= HANDLE_BASE_CHARS_MIN && len <= HANDLE_BASE_CHARS_MAX,
				Error::<T>::InvalidHandleCharacterLength
			);

			// Save canonical to shuffled suffix sequence cursor
			let canonical_handle_vec = convert_to_canonical(base_handle_str).as_bytes().to_vec();
			let canonical_handle: Handle = canonical_handle_vec.try_into().unwrap();
			let canonical_handle_str = core::str::from_utf8(&canonical_handle)
				.map_err(|_| Error::<T>::InvalidHandleEncoding)?;
			log::info!("canonical={}", canonical_handle_str);

			// Generate suffix
			let suffix_index =
				Self::get_next_suffix_index_for_canonical_handle(canonical_handle.clone())
					.unwrap_or_default();
			log::info!("suffix_index={}", suffix_index);
			let suffix =
				generate_suffix_for_canonical_handle(&canonical_handle_str, suffix_index as usize);
			CanonicalBaseHandleAndSuffixToMSAId::<T>::insert(
				canonical_handle.clone(),
				suffix,
				delegator_msa_id,
			);
			CanonicalBaseHandleToSuffixIndex::<T>::set(
				canonical_handle.clone(),
				Some(suffix_index),
			);

			let mut full_handle_vec: Vec<u8> = vec![];
			full_handle_vec.extend(base_handle_str.as_bytes());
			full_handle_vec.extend(b".");
			let mut buff = [0u8; 30];
			full_handle_vec.extend(suffix.numtoa(10, &mut buff));

			let full_handle: Handle = full_handle_vec.try_into().ok().unwrap();

			// Save display handle to MSA id
			//let full_handle_str = base_handle_str;
			MSAIdToDisplayName::<T>::insert(delegator_msa_id, full_handle.clone());

			Self::deposit_event(Event::HandleCreated {
				msa_id: delegator_msa_id,
				handle: full_handle.clone(),
			});
			Ok(())
		}

		/// Change handle
		#[pallet::call_index(1)]
		#[pallet::weight(1000)]
		pub fn change_handle(origin: OriginFor<T>) -> DispatchResult {
			let _delegator_key = ensure_signed(origin)?;
			Ok(())
		}

		/// Retire handle
		#[pallet::call_index(2)]
		#[pallet::weight(1000)]
		pub fn retire_handle(origin: OriginFor<T>) -> DispatchResult {
			let _delegator_key = ensure_signed(origin)?;
			Ok(())
		}
	}
}