// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_stateful_storage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-09, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_stateful-storage
// --extrinsic
// *
// --chain=frequency-bench
// --execution
// wasm
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 10
// --output=./pallets/stateful-storage/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_stateful_storage.
pub trait WeightInfo {
	fn apply_item_actions(n: u32, s: u32, ) -> Weight;
	fn upsert_page(s: u32, ) -> Weight;
	fn delete_page() -> Weight;
}

/// Weights for pallet_stateful_storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x0f7b5703746f38b9086400] (r:1 w:1)
	fn apply_item_actions(n: u32, s: u32, ) -> Weight {
		Weight::from_ref_time(19_731_864 as u64)
			// Standard Error: 21_514
			.saturating_add(Weight::from_ref_time(667_436 as u64).saturating_mul(n as u64))
			// Standard Error: 72
			.saturating_add(Weight::from_ref_time(2_554 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x6920a30fa06f2d600865004e79bd92607c0d88080100] (r:0 w:1)
	fn upsert_page(s: u32, ) -> Weight {
		Weight::from_ref_time(20_134_539 as u64)
			// Standard Error: 56
			.saturating_add(Weight::from_ref_time(692 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x6920a30fa06f2d600865004e79bd92607c0d88080100] (r:0 w:1)
	fn delete_page() -> Weight {
		Weight::from_ref_time(20_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x0f7b5703746f38b9086400] (r:1 w:1)
	fn apply_item_actions(n: u32, s: u32, ) -> Weight {
		Weight::from_ref_time(19_731_864 as u64)
			// Standard Error: 21_514
			.saturating_add(Weight::from_ref_time(667_436 as u64).saturating_mul(n as u64))
			// Standard Error: 72
			.saturating_add(Weight::from_ref_time(2_554 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x6920a30fa06f2d600865004e79bd92607c0d88080100] (r:0 w:1)
	fn upsert_page(s: u32, ) -> Weight {
		Weight::from_ref_time(20_134_539 as u64)
			// Standard Error: 56
			.saturating_add(Weight::from_ref_time(692 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:0)
	// Storage: unknown [0x6920a30fa06f2d600865004e79bd92607c0d88080100] (r:0 w:1)
	fn delete_page() -> Weight {
		Weight::from_ref_time(20_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
