//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for SubstrateWeight<T> {
	// Storage: Session NextKeys (r:16 w:0)
	// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(513), added: 1008, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 16]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		Weight::from_ref_time(11_789_817 as u64)
			// Standard Error: 10_423
			.saturating_add(Weight::from_ref_time(3_409_624 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates() -> Weight {
		Weight::from_ref_time(8_282_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond() -> Weight {
		Weight::from_ref_time(6_769_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(2401), added: 2896, mode: MaxEncodedLen)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(513), added: 1008, mode: MaxEncodedLen)
	// Storage: Session NextKeys (r:1 w:0)
	// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 49]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		Weight::from_ref_time(43_023_052 as u64)
			// Standard Error: 3_610
			.saturating_add(Weight::from_ref_time(265_573 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(2401), added: 2896, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[2, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		Weight::from_ref_time(28_056_365 as u64)
			// Standard Error: 2_503
			.saturating_add(Weight::from_ref_time(217_534 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author() -> Weight {
		Weight::from_ref_time(31_702_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(2401), added: 2896, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:50 w:0)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(513), added: 1008, mode: MaxEncodedLen)
	// Storage: System Account (r:49 w:49)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 50]`.
	/// The range of component `c` is `[1, 50]`.
	fn new_session(_r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(16_467_000 as u64)
			// Standard Error: 315_567
			.saturating_add(Weight::from_ref_time(12_250_934 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
