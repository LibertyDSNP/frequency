//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
		Weight::from_ref_time(10_321_000 as u64)
			// Standard Error: 56_336
			.saturating_add(Weight::from_ref_time(2_621_241 as u64).saturating_mul(m as u64))
			// Standard Error: 22_893
			.saturating_add(Weight::from_ref_time(3_079_614 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(18_817_000 as u64)
			// Standard Error: 14
			.saturating_add(Weight::from_ref_time(2_043 as u64).saturating_mul(b as u64))
			// Standard Error: 1_573
			.saturating_add(Weight::from_ref_time(24_139 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(20_919_258 as u64)
			// Standard Error: 14
			.saturating_add(Weight::from_ref_time(1_969 as u64).saturating_mul(b as u64))
			// Standard Error: 1_582
			.saturating_add(Weight::from_ref_time(22_774 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(25_011_139 as u64)
			// Standard Error: 53
			.saturating_add(Weight::from_ref_time(3_169 as u64).saturating_mul(b as u64))
			// Standard Error: 6_337
			.saturating_add(Weight::from_ref_time(169_597 as u64).saturating_mul(m as u64))
			// Standard Error: 2_211
			.saturating_add(Weight::from_ref_time(261_138 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// The range of component `m` is `[5, 10]`.
	/// The range of component `m` is `[5, 10]`.
	fn vote(m: u32, ) -> Weight {
		Weight::from_ref_time(25_386_091 as u64)
			// Standard Error: 6_421
			.saturating_add(Weight::from_ref_time(94_564 as u64).saturating_mul(m as u64))
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
		Weight::from_ref_time(27_989_524 as u64)
			// Standard Error: 7_620
			.saturating_add(Weight::from_ref_time(180_603 as u64).saturating_mul(m as u64))
			// Standard Error: 2_032
			.saturating_add(Weight::from_ref_time(227_701 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(35_615_147 as u64)
			// Standard Error: 56
			.saturating_add(Weight::from_ref_time(2_137 as u64).saturating_mul(b as u64))
			// Standard Error: 8_734
			.saturating_add(Weight::from_ref_time(146_888 as u64).saturating_mul(m as u64))
			// Standard Error: 2_336
			.saturating_add(Weight::from_ref_time(336_697 as u64).saturating_mul(p as u64))
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
		Weight::from_ref_time(30_487_865 as u64)
			// Standard Error: 6_588
			.saturating_add(Weight::from_ref_time(176_502 as u64).saturating_mul(m as u64))
			// Standard Error: 1_757
			.saturating_add(Weight::from_ref_time(224_584 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(38_105_430 as u64)
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(1_800 as u64).saturating_mul(b as u64))
			// Standard Error: 9_270
			.saturating_add(Weight::from_ref_time(177_993 as u64).saturating_mul(m as u64))
			// Standard Error: 2_480
			.saturating_add(Weight::from_ref_time(341_085 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `p` is `[1, 25]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(20_439_653 as u64)
			// Standard Error: 1_712
			.saturating_add(Weight::from_ref_time(192_962 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}