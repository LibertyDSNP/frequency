use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

/// Base types
#[derive(
	Clone, PartialEq, Debug, Eq, Serialize, Deserialize,
)]
pub enum ParquetBaseType {
	/// Encapsulates true / false values
	Boolean,
	/// Encapsulates 32 bit ints
	Int32,
	/// Encapsulates 64 bit ints
	Int64,
	/// Encapsulates floats
	Float,
	/// Encapsulates doubles
	Double,
	/// Encapsulates arrays
	ByteArray,
	/// Encapsulates fixed length arrays
	FixedLengthByteArray,
}
