//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_democracy
// --extrinsic
// *
// --chain=frequency
// --execution
// wasm
// --heap-pages=4096
// --wasm-execution
// compiled
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights/pallet_democracy.rs
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_democracy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for SubstrateWeight<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		Weight::from_ref_time(63_514_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn second(s: u32, ) -> Weight {
		Weight::from_ref_time(35_931_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(178_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn vote_new(r: u32, ) -> Weight {
		Weight::from_ref_time(48_199_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(307_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn vote_existing(r: u32, ) -> Weight {
		Weight::from_ref_time(48_632_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(302_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		Weight::from_ref_time(23_117_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn blacklist(p: u32, ) -> Weight {
		Weight::from_ref_time(57_154_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(368_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	/// The range of component `v` is `[1, 100]`.
	fn external_propose(v: u32, ) -> Weight {
		Weight::from_ref_time(17_047_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(15_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		Weight::from_ref_time(5_386_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		Weight::from_ref_time(5_353_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		Weight::from_ref_time(23_877_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	/// The range of component `v` is `[0, 100]`.
	fn veto_external(v: u32, ) -> Weight {
		Weight::from_ref_time(25_609_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(34_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn cancel_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(46_018_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(325_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		Weight::from_ref_time(15_189_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn cancel_queued(r: u32, ) -> Weight {
		Weight::from_ref_time(28_003_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(646_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// The range of component `r` is `[1, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		Weight::from_ref_time(8_250_000 as u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(2_620_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// The range of component `r` is `[1, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		Weight::from_ref_time(11_622_000 as u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(2_613_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn delegate(r: u32, ) -> Weight {
		Weight::from_ref_time(51_987_000 as u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(3_947_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		Weight::from_ref_time(30_528_000 as u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(3_890_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		Weight::from_ref_time(6_043_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	/// The range of component `b` is `[0, 16384]`.
	fn note_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(32_337_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	/// The range of component `b` is `[0, 16384]`.
	fn note_imminent_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(25_056_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `b` is `[0, 16384]`.
	fn reap_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(43_166_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		Weight::from_ref_time(34_975_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(209_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		Weight::from_ref_time(34_087_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(262_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn remove_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(20_344_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(258_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(20_718_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(252_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
