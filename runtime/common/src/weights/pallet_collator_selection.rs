//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-qpqf8-nrqm7`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_collator_selection
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for pallet_collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for SubstrateWeight<T> {
	/// Storage: Session NextKeys (r:16 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(513), added: 1008, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 16]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `334 + b * (79 ±0)`
		//  Estimated: `1324 + b * (2554 ±0)`
		// Minimum execution time: 13_091_000 picoseconds.
		Weight::from_parts(11_591_722, 1324)
			// Standard Error: 9_764
			.saturating_add(Weight::from_parts(3_294_156, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_939_000 picoseconds.
		Weight::from_parts(9_715_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_388_000 picoseconds.
		Weight::from_parts(6_598_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(2401), added: 2896, mode: MaxEncodedLen)
	/// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(513), added: 1008, mode: MaxEncodedLen)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 49]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `752 + c * (57 ±0)`
		//  Estimated: `4221 + c * (57 ±0)`
		// Minimum execution time: 44_954_000 picoseconds.
		Weight::from_parts(47_996_797, 4221)
			// Standard Error: 4_124
			.saturating_add(Weight::from_parts(266_121, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 57).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(2401), added: 2896, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[2, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276 + c * (49 ±0)`
		//  Estimated: `3886`
		// Minimum execution time: 30_798_000 picoseconds.
		Weight::from_parts(32_553_534, 3886)
			// Standard Error: 2_746
			.saturating_add(Weight::from_parts(226_318, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `212`
		//  Estimated: `6196`
		// Minimum execution time: 47_261_000 picoseconds.
		Weight::from_parts(48_296_000, 6196)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:0)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(2401), added: 2896, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:50 w:0)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(513), added: 1008, mode: MaxEncodedLen)
	/// Storage: System Account (r:49 w:49)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 50]`.
	/// The range of component `c` is `[1, 50]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1213 + c * (98 ±0) + r * (126 ±0)`
		//  Estimated: `3886 + c * (2519 ±0) + r * (2603 ±0)`
		// Minimum execution time: 16_245_000 picoseconds.
		Weight::from_parts(16_444_000, 3886)
			// Standard Error: 387_485
			.saturating_add(Weight::from_parts(14_130_044, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(r.into()))
	}
}
