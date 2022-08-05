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

//! Autogenerated weights for pallet_messages
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-05, STEPS: `20`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("./res/genesis/frequency-weights.json"), DB CACHE: 1024

// Executed Command:
// ./target/release/frequency
// benchmark
// pallet
// --chain
// ./res/genesis/frequency-weights.json
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_messages
// --extrinsic
// *
// --steps
// 20
// --repeat
// 5
// --output
// ./pallets/messages/src/weights.rs
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

/// Weight functions needed for pallet_messages.
pub trait WeightInfo {
	fn add_onchain_message(n: u32, m: u32, ) -> Weight;
	fn add_ipfs_message(n: u32, m: u32, ) -> Weight;
	fn on_initialize(m: u32, s: u32, ) -> Weight;
}

/// Weights for pallet_messages using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_onchain_message(n: u32, m: u32, ) -> Weight {
		(22_998_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 5_000
			.saturating_add((311_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_ipfs_message(n: u32, m: u32, ) -> Weight {
		(13_374_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 8_000
			.saturating_add((329_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Messages BlockMessages (r:1 w:1)
	// Storage: Messages Messages (r:0 w:1)
	fn on_initialize(m: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 37_000
			.saturating_add((677_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 365_000
			.saturating_add((9_529_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_onchain_message(n: u32, m: u32, ) -> Weight {
		(22_998_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 5_000
			.saturating_add((311_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Schemas Schemas (r:1 w:0)
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Messages BlockMessages (r:1 w:1)
	fn add_ipfs_message(n: u32, m: u32, ) -> Weight {
		(13_374_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 8_000
			.saturating_add((329_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Messages BlockMessages (r:1 w:1)
	// Storage: Messages Messages (r:0 w:1)
	fn on_initialize(m: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 37_000
			.saturating_add((677_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 365_000
			.saturating_add((9_529_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
}
