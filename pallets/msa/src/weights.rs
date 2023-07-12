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
//! DATE: 2023-07-12, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-qpqf8-kfvv2`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_msa
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=20
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/msa/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	fn create() -> Weight;
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_provider() -> Weight;
	fn add_public_key_to_msa() -> Weight;
	fn delete_msa_public_key() -> Weight;
	fn retire_msa() -> Weight;
	fn grant_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_delegator() -> Weight;
	fn create_provider() -> Weight;
	fn create_provider_via_governance() -> Weight;
	fn propose_to_be_provider() -> Weight;
	fn revoke_schema_permissions(s: u32, ) -> Weight;
}

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	/// Proof: Msa CurrentMsaIdentifierMaximum (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `12423`
		// Minimum execution time: 13_571_000 picoseconds.
		Weight::from_parts(13_899_000, 12423)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Msa PayloadSignatureRegistryList (r:2 w:2)
	/// Proof: Msa PayloadSignatureRegistryList (max_values: Some(50000), max_size: Some(144), added: 2124, mode: MaxEncodedLen)
	/// Storage: Msa PayloadSignatureRegistryPointer (r:1 w:1)
	/// Proof: Msa PayloadSignatureRegistryPointer (max_values: Some(1), max_size: Some(140), added: 635, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	/// Proof: Msa CurrentMsaIdentifierMaximum (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	/// Proof Skipped: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 30]`.
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1393`
		//  Estimated: `14946 + s * (1 ±0)`
		// Minimum execution time: 119_154_000 picoseconds.
		Weight::from_parts(126_310_912, 14946)
			// Standard Error: 117_267
			.saturating_add(Weight::from_parts(56_514, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(s.into()))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	fn revoke_delegation_by_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `12592`
		// Minimum execution time: 17_328_000 picoseconds.
		Weight::from_parts(18_211_000, 12592)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PayloadSignatureRegistryList (r:4 w:4)
	/// Proof: Msa PayloadSignatureRegistryList (max_values: Some(50000), max_size: Some(144), added: 2124, mode: MaxEncodedLen)
	/// Storage: Msa PayloadSignatureRegistryPointer (r:1 w:1)
	/// Proof: Msa PayloadSignatureRegistryPointer (max_values: Some(1), max_size: Some(140), added: 635, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn add_public_key_to_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1596`
		//  Estimated: `18396`
		// Minimum execution time: 174_439_000 picoseconds.
		Weight::from_parts(177_210_000, 18396)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn delete_msa_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `14946`
		// Minimum execution time: 24_445_000 picoseconds.
		Weight::from_parts(25_492_000, 14946)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn retire_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `12423`
		// Minimum execution time: 20_340_000 picoseconds.
		Weight::from_parts(20_809_000, 12423)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Msa PayloadSignatureRegistryList (r:2 w:2)
	/// Proof: Msa PayloadSignatureRegistryList (max_values: Some(50000), max_size: Some(144), added: 2124, mode: MaxEncodedLen)
	/// Storage: Msa PayloadSignatureRegistryPointer (r:1 w:1)
	/// Proof: Msa PayloadSignatureRegistryPointer (max_values: Some(1), max_size: Some(140), added: 635, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	/// Proof Skipped: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 30]`.
	fn grant_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1443`
		//  Estimated: `14946`
		// Minimum execution time: 113_212_000 picoseconds.
		Weight::from_parts(117_059_102, 14946)
			// Standard Error: 24_081
			.saturating_add(Weight::from_parts(113_125, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	fn revoke_delegation_by_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `12592`
		// Minimum execution time: 17_910_000 picoseconds.
		Weight::from_parts(18_252_000, 12592)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	fn create_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `12423`
		// Minimum execution time: 14_257_000 picoseconds.
		Weight::from_parts(14_676_000, 12423)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	fn create_provider_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `12423`
		// Minimum execution time: 14_426_000 picoseconds.
		Weight::from_parts(14_651_000, 12423)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	fn propose_to_be_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `222`
		//  Estimated: `12597`
		// Minimum execution time: 26_502_000 picoseconds.
		Weight::from_parts(27_163_000, 12597)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	/// Proof Skipped: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 30]`.
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391 + s * (6 ±0)`
		//  Estimated: `12592 + s * (6 ±0)`
		// Minimum execution time: 21_494_000 picoseconds.
		Weight::from_parts(23_037_458, 12592)
			// Standard Error: 5_166
			.saturating_add(Weight::from_parts(140_572, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 6).saturating_mul(s.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	/// Proof: Msa CurrentMsaIdentifierMaximum (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `12423`
		// Minimum execution time: 13_571_000 picoseconds.
		Weight::from_parts(13_899_000, 12423)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Msa PayloadSignatureRegistryList (r:2 w:2)
	/// Proof: Msa PayloadSignatureRegistryList (max_values: Some(50000), max_size: Some(144), added: 2124, mode: MaxEncodedLen)
	/// Storage: Msa PayloadSignatureRegistryPointer (r:1 w:1)
	/// Proof: Msa PayloadSignatureRegistryPointer (max_values: Some(1), max_size: Some(140), added: 635, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Msa CurrentMsaIdentifierMaximum (r:1 w:1)
	/// Proof: Msa CurrentMsaIdentifierMaximum (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	/// Proof Skipped: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 30]`.
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1393`
		//  Estimated: `14946 + s * (1 ±0)`
		// Minimum execution time: 119_154_000 picoseconds.
		Weight::from_parts(126_310_912, 14946)
			// Standard Error: 117_267
			.saturating_add(Weight::from_parts(56_514, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(s.into()))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	fn revoke_delegation_by_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `12592`
		// Minimum execution time: 17_328_000 picoseconds.
		Weight::from_parts(18_211_000, 12592)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PayloadSignatureRegistryList (r:4 w:4)
	/// Proof: Msa PayloadSignatureRegistryList (max_values: Some(50000), max_size: Some(144), added: 2124, mode: MaxEncodedLen)
	/// Storage: Msa PayloadSignatureRegistryPointer (r:1 w:1)
	/// Proof: Msa PayloadSignatureRegistryPointer (max_values: Some(1), max_size: Some(140), added: 635, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn add_public_key_to_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1596`
		//  Estimated: `18396`
		// Minimum execution time: 174_439_000 picoseconds.
		Weight::from_parts(177_210_000, 18396)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:2 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn delete_msa_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `14946`
		// Minimum execution time: 24_445_000 picoseconds.
		Weight::from_parts(25_492_000, 14946)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyCountForMsaId (r:1 w:1)
	/// Proof: Msa PublicKeyCountForMsaId (max_values: None, max_size: Some(17), added: 2492, mode: MaxEncodedLen)
	fn retire_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `12423`
		// Minimum execution time: 20_340_000 picoseconds.
		Weight::from_parts(20_809_000, 12423)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Msa PayloadSignatureRegistryList (r:2 w:2)
	/// Proof: Msa PayloadSignatureRegistryList (max_values: Some(50000), max_size: Some(144), added: 2124, mode: MaxEncodedLen)
	/// Storage: Msa PayloadSignatureRegistryPointer (r:1 w:1)
	/// Proof: Msa PayloadSignatureRegistryPointer (max_values: Some(1), max_size: Some(140), added: 635, mode: MaxEncodedLen)
	/// Storage: Msa PublicKeyToMsaId (r:2 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:0)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	/// Proof Skipped: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 30]`.
	fn grant_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1443`
		//  Estimated: `14946`
		// Minimum execution time: 113_212_000 picoseconds.
		Weight::from_parts(117_059_102, 14946)
			// Standard Error: 24_081
			.saturating_add(Weight::from_parts(113_125, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	fn revoke_delegation_by_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `12592`
		// Minimum execution time: 17_910_000 picoseconds.
		Weight::from_parts(18_252_000, 12592)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	fn create_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `12423`
		// Minimum execution time: 14_257_000 picoseconds.
		Weight::from_parts(14_676_000, 12423)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa ProviderToRegistryEntry (r:1 w:1)
	/// Proof: Msa ProviderToRegistryEntry (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	fn create_provider_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `12423`
		// Minimum execution time: 14_426_000 picoseconds.
		Weight::from_parts(14_651_000, 12423)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	fn propose_to_be_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `222`
		//  Estimated: `12597`
		// Minimum execution time: 26_502_000 picoseconds.
		Weight::from_parts(27_163_000, 12597)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Msa PublicKeyToMsaId (r:1 w:0)
	/// Proof: Msa PublicKeyToMsaId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Msa DelegatorAndProviderToDelegation (r:1 w:1)
	/// Proof: Msa DelegatorAndProviderToDelegation (max_values: None, max_size: Some(217), added: 2692, mode: MaxEncodedLen)
	/// Storage: Schemas CurrentSchemaIdentifierMaximum (r:1 w:0)
	/// Proof Skipped: Schemas CurrentSchemaIdentifierMaximum (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `s` is `[0, 30]`.
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391 + s * (6 ±0)`
		//  Estimated: `12592 + s * (6 ±0)`
		// Minimum execution time: 21_494_000 picoseconds.
		Weight::from_parts(23_037_458, 12592)
			// Standard Error: 5_166
			.saturating_add(Weight::from_parts(140_572, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 6).saturating_mul(s.into()))
	}
}
