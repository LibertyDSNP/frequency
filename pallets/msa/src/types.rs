//! Types for the MSA Pallet
use super::*;
pub use common_primitives::msa::{Delegator, KeyInfoResponse, MessageSourceId, Provider};
use scale_info::TypeInfo;

use codec::{Decode, Encode};

/// Dispatch Empty
pub const EMPTY_FUNCTION: fn(MessageSourceId) -> DispatchResult = |_| Ok(());

/// The maximum number of future blocks that a proof expiration can be valid.
pub const PROOF_VALID_BLOCKS: u32 = 128;

/// A type definition for the payload of adding an MSA key - `pallet_msa::add_key_to_msa`
#[derive(TypeInfo, Debug, Clone, Decode, Encode, PartialEq, Eq)]
pub struct AddKeyData<BlockNumber> {
	/// Message Source Account identifier
	pub msa_id: MessageSourceId,
	/// A cryptographic nonce.
	pub nonce: u32,
	/// The block number at which the signed proof for add_key_to_msa expires.
	pub expiration: BlockNumber,
}

/// Structure that is signed for granting permissions to a Provider
#[derive(TypeInfo, Debug, Clone, Decode, Encode, PartialEq, Eq)]
pub struct AddProvider {
	/// The provider being granted permissions
	pub authorized_msa_id: MessageSourceId,
	/// The permissions granted
	pub permission: u8,
}
