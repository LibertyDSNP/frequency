//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-22, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_scheduler
// --extrinsic
// *
// --chain=frequency
// --execution
// wasm
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 10
// --output=./scripts/../runtime/common/src/weights/pallet_scheduler.rs
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for SubstrateWeight<T> {
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		(7_761_000 as Weight)
			// Standard Error: 65_000
			.saturating_add((21_394_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
		(3_107_000 as Weight)
			// Standard Error: 45_000
			.saturating_add((17_164_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
		(7_211_000 as Weight)
			// Standard Error: 50_000
			.saturating_add((18_743_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_resolved(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 43_000
			.saturating_add((16_723_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_named_aborted(s: u32, ) -> Weight {
		(6_761_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((5_674_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_aborted(s: u32, ) -> Weight {
		(8_290_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((3_443_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic_named(s: u32, ) -> Weight {
		(9_987_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((10_793_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_periodic(s: u32, ) -> Weight {
		(11_372_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((8_350_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize_named(s: u32, ) -> Weight {
		(10_614_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((6_510_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn on_initialize(s: u32, ) -> Weight {
		(9_395_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((5_637_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn schedule(s: u32, ) -> Weight {
		(16_457_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((23_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		(16_239_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((553_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn schedule_named(s: u32, ) -> Weight {
		(19_050_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((26_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		(17_462_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((585_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
