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

//! Autogenerated weights for pallet_msa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-14, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency"), DB CACHE: 1024

// Executed Command:
// ./target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_msa
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
// --output=./pallets/msa/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
rustdoc::all,
missing_docs,
unused_parens,
unused_imports
)]

use frame_support::{traits::Get, weights::{constants::RocksDbWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	fn create(s: u32) -> Weight;
	fn create_sponsored_account_with_delegation() -> Weight;
	fn revoke_delegation_by_provider(s: u32) -> Weight;
	fn add_key_to_msa() -> Weight;
	fn delete_msa_key() -> Weight;
	fn retire_msa() -> Weight;
	fn add_provider_to_msa() -> Weight;
	fn revoke_msa_delegation_by_delegator() -> Weight;
	fn register_provider() -> Weight;
	fn on_initialize(s: u32) -> Weight;
}

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa MessageSourceIdOf (r:1 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn create(s: u32) -> Weight {
		Weight::from_ref_time(22_102_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(9_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:1)
	// Storage: Msa ProviderRegistry (r:1 w:0)
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn create_sponsored_account_with_delegation() -> Weight {
		Weight::from_ref_time(61_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn revoke_delegation_by_provider(s: u32) -> Weight {
		Weight::from_ref_time(20_484_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(22_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn add_key_to_msa() -> Weight {
		Weight::from_ref_time(57_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn delete_msa_key() -> Weight {
		Weight::from_ref_time(22_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:1)
	// Storage: Msa ProviderRegistry (r:1 w:0)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn retire_msa() -> Weight {
		Weight::from_ref_time(25_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:0)
	// Storage: Msa ProviderRegistry (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn add_provider_to_msa() -> Weight {
		Weight::from_ref_time(58_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn revoke_msa_delegation_by_delegator() -> Weight {
		Weight::from_ref_time(18_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:0)
	// Storage: Msa ProviderRegistry (r:1 w:1)
	fn register_provider() -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(s as u64))
			.saturating_mul(2 as u64)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa MessageSourceIdOf (r:1 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn create(s: u32) -> Weight {
		Weight::from_ref_time(22_102_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(9_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:1)
	// Storage: Msa ProviderRegistry (r:1 w:0)
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn create_sponsored_account_with_delegation() -> Weight {
		Weight::from_ref_time(61_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn revoke_delegation_by_provider(s: u32) -> Weight {
		Weight::from_ref_time(20_484_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(22_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn add_key_to_msa() -> Weight {
		Weight::from_ref_time(57_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:1)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn delete_msa_key() -> Weight {
		Weight::from_ref_time(22_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:1)
	// Storage: Msa ProviderRegistry (r:1 w:0)
	// Storage: Msa MsaInfoOf (r:1 w:1)
	fn retire_msa() -> Weight {
		Weight::from_ref_time(25_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:2 w:0)
	// Storage: Msa ProviderRegistry (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn add_provider_to_msa() -> Weight {
		Weight::from_ref_time(58_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn revoke_msa_delegation_by_delegator() -> Weight {
		Weight::from_ref_time(18_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa MessageSourceIdOf (r:1 w:0)
	// Storage: Msa ProviderRegistry (r:1 w:1)
	fn register_provider() -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(s as u64))
			.saturating_mul(2 as u64)
	}
}
