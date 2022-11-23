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
//! DATE: 2022-11-22, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_msa
// --extrinsic
// *
// --chain=frequency-bench
// --execution
// wasm
// --heap-pages=4096
// --wasm-execution
// compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/msa/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	fn create() -> Weight;
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_provider() -> Weight;
	fn add_public_key_to_msa() -> Weight;
	fn delete_msa_public_key() -> Weight;
	fn retire_msa(s: u32, ) -> Weight;
	fn grant_delegation() -> Weight;
	fn revoke_delegation_by_delegator() -> Weight;
	fn create_provider() -> Weight;
	fn on_initialize(m: u32, ) -> Weight;
	fn grant_schema_permissions(s: u32, ) -> Weight;
	fn revoke_schema_permissions(s: u32, ) -> Weight;
}

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(20_540_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		Weight::from_ref_time(101_103_000 as u64)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(84_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_provider() -> Weight {
		Weight::from_ref_time(27_933_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:2 w:2)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn add_public_key_to_msa() -> Weight {
		Weight::from_ref_time(144_572_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn delete_msa_public_key() -> Weight {
		Weight::from_ref_time(29_673_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:0 w:3)
	fn retire_msa(s: u32, ) -> Weight {
		Weight::from_ref_time(27_470_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_211_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn grant_delegation() -> Weight {
		Weight::from_ref_time(94_815_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_delegator() -> Weight {
		Weight::from_ref_time(26_332_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	fn create_provider() -> Weight {
		Weight::from_ref_time(20_813_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn on_initialize(m: u32, ) -> Weight {
		Weight::from_ref_time(12_542_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(m as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn grant_schema_permissions(s: u32, ) -> Weight {
		Weight::from_ref_time(56_259_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(107_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		Weight::from_ref_time(56_742_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(104_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(20_540_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		Weight::from_ref_time(101_103_000 as u64)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(84_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_provider() -> Weight {
		Weight::from_ref_time(27_933_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PayloadSignatureRegistry (r:2 w:2)
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn add_public_key_to_msa() -> Weight {
		Weight::from_ref_time(144_572_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	fn delete_msa_public_key() -> Weight {
		Weight::from_ref_time(29_673_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	// Storage: Msa DelegatorAndProviderToDelegation (r:0 w:3)
	fn retire_msa(s: u32, ) -> Weight {
		Weight::from_ref_time(27_470_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_211_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Msa PayloadSignatureRegistry (r:1 w:1)
	// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn grant_delegation() -> Weight {
		Weight::from_ref_time(94_815_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	fn revoke_delegation_by_delegator() -> Weight {
		Weight::from_ref_time(26_332_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	fn create_provider() -> Weight {
		Weight::from_ref_time(20_813_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn on_initialize(m: u32, ) -> Weight {
		Weight::from_ref_time(12_542_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(m as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn grant_schema_permissions(s: u32, ) -> Weight {
		Weight::from_ref_time(56_259_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(107_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		Weight::from_ref_time(56_742_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(104_000 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
