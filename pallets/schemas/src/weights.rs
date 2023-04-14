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

//! Autogenerated weights for pallet_schemas
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-16, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/production/frequency
// benchmark
// pallet
// --pallet
// pallet_schemas
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
// --output=./scripts/../pallets/schemas/src/weights.rs
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

/// Weight functions needed for pallet_schemas.
pub trait WeightInfo {
	fn create_schema(m: u32, ) -> Weight;
	fn create_schema_via_governance(m: u32, ) -> Weight;
	fn propose_to_create_schema(m: u32, ) -> Weight;
	fn set_max_schema_model_bytes() -> Weight;
}

/// Weights for pallet_schemas using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	// Storage: Schemas Schemas (r:0 w:1)
	fn create_schema(m: u32, ) -> Weight {
		Weight::from_ref_time(19_730_000 as u64)
			// Standard Error: 48
			.saturating_add(Weight::from_ref_time(28_294 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	// Storage: Schemas Schemas (r:0 w:1)
	fn create_schema_via_governance(m: u32, ) -> Weight {
		Weight::from_ref_time(19_380_000 as u64)
			// Standard Error: 43
			.saturating_add(Weight::from_ref_time(28_250 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_to_create_schema(m: u32, ) -> Weight {
		Weight::from_ref_time(11_234_174 as u64)
			// Standard Error: 55
			.saturating_add(Weight::from_ref_time(4_457 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Schemas GovernanceSchemaModelMaxBytes (r:0 w:1)
	fn set_max_schema_model_bytes() -> Weight {
		Weight::from_ref_time(13_266_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	// Storage: Schemas Schemas (r:0 w:1)
	fn create_schema(m: u32, ) -> Weight {
		Weight::from_ref_time(19_730_000 as u64)
			// Standard Error: 48
			.saturating_add(Weight::from_ref_time(28_294 as u64).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Schemas GovernanceSchemaModelMaxBytes (r:1 w:0)
	// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:1)
	// Storage: Schemas Schemas (r:0 w:1)
	fn create_schema_via_governance(m: u32, ) -> Weight {
		Weight::from_ref_time(19_380_000 as u64)
			// Standard Error: 43
			.saturating_add(Weight::from_ref_time(28_250 as u64).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_to_create_schema(m: u32, ) -> Weight {
		Weight::from_ref_time(11_234_174 as u64)
			// Standard Error: 55
			.saturating_add(Weight::from_ref_time(4_457 as u64).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Schemas GovernanceSchemaModelMaxBytes (r:0 w:1)
	fn set_max_schema_model_bytes() -> Weight {
		Weight::from_ref_time(13_266_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}