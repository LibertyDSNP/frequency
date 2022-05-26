#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use common_primitives::msa::*;
use frame_support::dispatch::DispatchError;
use sp_std::vec::Vec;
use sp_std::collections::btree_map::BTreeMap;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait MsaApi<AccountId, BlockNumber> where
		AccountId: Codec,
		BlockNumber: Codec,
	{
		fn get_msa_keys(msa_id: MessageSenderId) ->	Result<Vec<KeyInfoResponse<AccountId, BlockNumber>>, DispatchError>;

		fn get_msa_id(key: AccountId) -> Result<Option<MessageSenderId>, DispatchError>;

		fn check_delegations(delegator_msa_ids: Vec<MessageSenderId>, provider_msa_id: MessageSenderId) -> Result<BTreeMap<MessageSenderId, bool>, DispatchError>;
	}
}
