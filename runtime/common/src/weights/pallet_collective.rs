//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_collective
// --extrinsic
// *
// --chain=frequency-bench
// --execution
// wasm
// --heap-pages=4096
// --wasm-execution
// compiled
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights/pallet_collective.rs
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	// Storage: TechnicalCommittee Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Storage: TechnicalCommittee Voting (r:25 w:25)
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 25]`.
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 25]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(12_059_000 as u64)
			// Standard Error: 64_288
			.saturating_add(Weight::from_ref_time(2_623_233 as u64).saturating_mul(m as u64))
			// Standard Error: 26_124
			.saturating_add(Weight::from_ref_time(3_478_259 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(21_670_977 as u64)
			// Standard Error: 15
			.saturating_add(Weight::from_ref_time(2_029 as u64).saturating_mul(b as u64))
			// Standard Error: 1_665
			.saturating_add(Weight::from_ref_time(12_916 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(23_659_967 as u64)
			// Standard Error: 17
			.saturating_add(Weight::from_ref_time(2_014 as u64).saturating_mul(b as u64))
			// Standard Error: 1_886
			.saturating_add(Weight::from_ref_time(33_749 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(30_353_031 as u64)
			// Standard Error: 51
			.saturating_add(Weight::from_ref_time(3_056 as u64).saturating_mul(b as u64))
			// Standard Error: 6_154
			.saturating_add(Weight::from_ref_time(31_424 as u64).saturating_mul(m as u64))
			// Standard Error: 2_147
			.saturating_add(Weight::from_ref_time(356_061 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// The range of component `m` is `[5, 10]`.
	/// The range of component `m` is `[5, 10]`.
	fn vote(m: u32, ) -> Weight {
		Weight::from_ref_time(31_002_204 as u64)
			// Standard Error: 7_543
			.saturating_add(Weight::from_ref_time(90_767 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(33_008_898 as u64)
			// Standard Error: 7_006
			.saturating_add(Weight::from_ref_time(76_380 as u64).saturating_mul(m as u64))
			// Standard Error: 1_869
			.saturating_add(Weight::from_ref_time(339_627 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_approved(b: u32, _m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(42_599_680 as u64)
			// Standard Error: 58
			.saturating_add(Weight::from_ref_time(2_584 as u64).saturating_mul(b as u64))
			// Standard Error: 2_410
			.saturating_add(Weight::from_ref_time(419_205 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(35_676_009 as u64)
			// Standard Error: 6_579
			.saturating_add(Weight::from_ref_time(67_734 as u64).saturating_mul(m as u64))
			// Standard Error: 1_755
			.saturating_add(Weight::from_ref_time(331_129 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(45_151_121 as u64)
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(2_382 as u64).saturating_mul(b as u64))
			// Standard Error: 9_381
			.saturating_add(Weight::from_ref_time(21_535 as u64).saturating_mul(m as u64))
			// Standard Error: 2_509
			.saturating_add(Weight::from_ref_time(427_513 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `p` is `[1, 25]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(22_991_978 as u64)
			// Standard Error: 1_534
			.saturating_add(Weight::from_ref_time(307_333 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
