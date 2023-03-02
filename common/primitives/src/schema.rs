use crate::impl_codec_bitflags;
#[cfg(feature = "std")]
use crate::utils;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use enumflags2::{bitflags, BitFlags};
use frame_support::RuntimeDebug;
use scale_info::{build::Fields, meta_type, Path, Type, TypeInfo, TypeParameter};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use utils::*;

/// Schema Id is the unique identifier for a Schema
pub type SchemaId = u16;

/// Types of modeling in which a message payload may be defined
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, MaxEncodedLen)]
pub enum ModelType {
	/// Message payload modeled with Apache Avro: <https://avro.apache.org/docs/current/spec.html>
	AvroBinary,
	/// Message payload modeled with Apache Parquet: <https://parquet.apache.org/>
	Parquet,
}

/// Types of payload locations
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, MaxEncodedLen)]
pub enum PayloadLocation {
	/// Message payload is located on chain
	OnChain,
	/// Message payload is located on IPFS
	IPFS,
	/// Itemized payload location for onchain storage in itemized form
	Itemized,
	/// Paginated payload location for onchain storage in paginated form
	Paginated,
}

/// Support for up to 16 user-enabled features on a collection.
#[bitflags]
#[repr(u16)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum SchemaSetting {
	/// Schema setting to enforce append-only behavior on payload.
	/// Applied to schemas of type `PayloadLocation::Itemized` or `PayloadLocation::Paginated`.
	AppendOnly,
	/// Schema may enforce signature requirement on payload.
	/// Applied to schemas of type `PayloadLocation::Paginated`.
	SignatureRequired,
}

/// Wrapper type for `BitFlags<SchemaSetting>` that implements `Codec`.
#[derive(Clone, Copy, PartialEq, Eq, Default, RuntimeDebug)]
pub struct SchemaSettings(pub BitFlags<SchemaSetting>);

/// RPC Response form for a Schema
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
pub struct SchemaResponse {
	/// The unique identifier for this Schema
	pub schema_id: SchemaId,
	/// The data that represents how this schema is structured
	#[cfg_attr(feature = "std", serde(with = "as_string"))]
	pub model: Vec<u8>,
	/// The model format type for how the schema model is represented
	pub model_type: ModelType,
	/// The payload location
	pub payload_location: PayloadLocation,
	/// grants for the schema
	pub settings: Vec<SchemaSetting>,
}

/// This allows other pallets to resolve Schema information. With generic SchemaId
pub trait SchemaProvider<SchemaId> {
	/// Gets the Schema details associated with this `SchemaId` if any
	fn get_schema_by_id(schema_id: SchemaId) -> Option<SchemaResponse>;
}

/// This allows other Pallets to check validity of schema ids.
pub trait SchemaValidator<SchemaId> {
	/// Checks that a collection of SchemaIds are all valid
	fn are_all_schema_ids_valid(schema_ids: &Vec<SchemaId>) -> bool;

	/// Set the schema counter for testing purposes.
	#[cfg(any(feature = "std", feature = "runtime-benchmarks", test))]
	fn set_schema_count(n: SchemaId);
}

impl SchemaSettings {
	/// Set all settings to disabled
	pub fn all_disabled() -> Self {
		Self(BitFlags::EMPTY)
	}
	/// Get all setting enabled
	pub fn get_enabled(&self) -> BitFlags<SchemaSetting> {
		self.0
	}
	/// Check if a setting is enabled
	pub fn is_enabled(&self, grant: SchemaSetting) -> bool {
		self.0.contains(grant)
	}
	/// Enable a setting
	pub fn set(&mut self, grant: SchemaSetting) {
		self.0.insert(grant)
	}
	/// Copy the settings from a BitFlags
	pub fn from(settings: BitFlags<SchemaSetting>) -> Self {
		Self(settings)
	}
}
impl_codec_bitflags!(SchemaSettings, u16, SchemaSetting);
