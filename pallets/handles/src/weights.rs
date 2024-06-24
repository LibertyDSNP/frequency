
//! Autogenerated weights for `pallet_handles`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-24, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-164`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_handles
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=5
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/handles/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_handles`.
pub trait WeightInfo {
	fn claim_handle(b: u32, ) -> Weight;
	fn change_handle(b: u32, ) -> Weight;
	fn retire_handle() -> Weight;
}

/// Weights for `pallet_handles` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Handles::MSAIdToDisplayName` (r:1 w:1)
	/// Proof: `Handles::MSAIdToDisplayName` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleToSuffixIndex` (r:1 w:1)
	/// Proof: `Handles::CanonicalBaseHandleToSuffixIndex` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (r:0 w:1)
	/// Proof: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[3, 30]`.
	fn claim_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `116`
		//  Estimated: `5009`
		// Minimum execution time: 78_291_000 picoseconds.
		Weight::from_parts(80_302_371, 5009)
			// Standard Error: 12_151
			.saturating_add(Weight::from_parts(65_937, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Handles::MSAIdToDisplayName` (r:1 w:1)
	/// Proof: `Handles::MSAIdToDisplayName` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleToSuffixIndex` (r:1 w:1)
	/// Proof: `Handles::CanonicalBaseHandleToSuffixIndex` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (r:0 w:2)
	/// Proof: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[3, 30]`.
	fn change_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `222 + b * (1 ±0)`
		//  Estimated: `5009`
		// Minimum execution time: 87_080_000 picoseconds.
		Weight::from_parts(88_770_583, 5009)
			// Standard Error: 14_395
			.saturating_add(Weight::from_parts(169_095, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Handles::MSAIdToDisplayName` (r:1 w:1)
	/// Proof: `Handles::MSAIdToDisplayName` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (r:0 w:1)
	/// Proof: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn retire_handle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `231`
		//  Estimated: `5009`
		// Minimum execution time: 19_542_000 picoseconds.
		Weight::from_parts(20_398_000, 5009)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Handles::MSAIdToDisplayName` (r:1 w:1)
	/// Proof: `Handles::MSAIdToDisplayName` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleToSuffixIndex` (r:1 w:1)
	/// Proof: `Handles::CanonicalBaseHandleToSuffixIndex` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (r:0 w:1)
	/// Proof: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[3, 30]`.
	fn claim_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `116`
		//  Estimated: `5009`
		// Minimum execution time: 78_291_000 picoseconds.
		Weight::from_parts(80_302_371, 5009)
			// Standard Error: 12_151
			.saturating_add(Weight::from_parts(65_937, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Handles::MSAIdToDisplayName` (r:1 w:1)
	/// Proof: `Handles::MSAIdToDisplayName` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleToSuffixIndex` (r:1 w:1)
	/// Proof: `Handles::CanonicalBaseHandleToSuffixIndex` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (r:0 w:2)
	/// Proof: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[3, 30]`.
	fn change_handle(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `222 + b * (1 ±0)`
		//  Estimated: `5009`
		// Minimum execution time: 87_080_000 picoseconds.
		Weight::from_parts(88_770_583, 5009)
			// Standard Error: 14_395
			.saturating_add(Weight::from_parts(169_095, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Handles::MSAIdToDisplayName` (r:1 w:1)
	/// Proof: `Handles::MSAIdToDisplayName` (`max_values`: None, `max_size`: Some(59), added: 2534, mode: `MaxEncodedLen`)
	/// Storage: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (r:0 w:1)
	/// Proof: `Handles::CanonicalBaseHandleAndSuffixToMSAId` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn retire_handle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `231`
		//  Estimated: `5009`
		// Minimum execution time: 19_542_000 picoseconds.
		Weight::from_parts(20_398_000, 5009)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

  struct BlockWeights;
  impl Get<frame_system::limits::BlockWeights> for BlockWeights {
  	fn get() -> frame_system::limits::BlockWeights {
  		frame_system::limits::BlockWeights::builder()
  			.base_block(Weight::zero())
  			.for_class(DispatchClass::all(), |weights| {
  				weights.base_extrinsic = ExtrinsicBaseWeight::get().into();
  			})
  			.for_class(DispatchClass::non_mandatory(), |weights| {
  				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
  			})
  			.build_or_panic()
  	}
  }

	#[test]
	fn test_claim_handle() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5009
		);
	}
	#[test]
	fn test_change_handle() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5009
		);
	}
	#[test]
	fn test_retire_handle() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5009
		);
	}
}
