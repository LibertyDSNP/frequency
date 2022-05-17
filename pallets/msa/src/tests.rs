use crate::{
	mock::*,
	types::{AddKeyData, KeyInfo},
	Call, Config, Error, Event, MsaIdentifier,
};
use common_primitives::{msa::KeyInfoResponse, utils::wrap_binary_data};
use frame_support::{assert_noop, assert_ok, weights::GetDispatchInfo};
use sp_core::{crypto::AccountId32, sr25519, Encode, Pair};
use sp_runtime::MultiSignature;

#[test]
fn it_creates_an_msa_account() {
	new_test_ext().execute_with(|| {
		assert_ok!(Msa::create(test_origin_signed(1)));

		assert_eq!(
			Msa::get_key_info(test_public(1)),
			Some(KeyInfo { msa_id: 1, expired: 0, nonce: 0 })
		);

		assert_eq!(Msa::get_identifier(), 1);

		System::assert_last_event(
			Event::MsaCreated { msa_id: 1, key: test_public(1).into() }.into(),
		);
	});
}

#[test]
fn it_throws_msa_identifier_overflow() {
	new_test_ext().execute_with(|| {
		MsaIdentifier::<Test>::set(u64::MAX);

		assert_noop!(Msa::create(test_origin_signed(1)), Error::<Test>::MsaIdOverflow);
	});
}

#[test]
#[allow(unused_must_use)]
fn it_does_not_allow_duplicate_keys() {
	new_test_ext().execute_with(|| {
		Msa::create(test_origin_signed(1));

		assert_noop!(Msa::create(test_origin_signed(1)), Error::<Test>::DuplicatedKey);

		assert_eq!(Msa::get_identifier(), 1);
	});
}

#[test]
fn it_create_has_weight() {
	new_test_ext().execute_with(|| {
		let call = Call::<Test>::create {};
		let dispatch_info = call.get_dispatch_info();

		assert!(dispatch_info.weight > 10_000);
	});
}

#[test]
fn it_throws_error_when_key_verification_fails() {
	new_test_ext().execute_with(|| {
		let (key_pair, _) = sr25519::Pair::generate();
		let (key_pair_2, _) = sr25519::Pair::generate();

		let new_account = key_pair.public();
		let (new_msa_id, _) = Msa::create_account(new_account.into()).unwrap();

		let fake_account = key_pair_2.public();

		let add_new_key_data = AddKeyData { nonce: 1, msa_id: new_msa_id };
		let encode_data_new_key_data = wrap_binary_data(add_new_key_data.encode());

		let signature: MultiSignature = key_pair.sign(&encode_data_new_key_data).into();

		assert_noop!(
			Msa::add_key_to_msa(
				test_origin_signed(1),
				fake_account.into(),
				signature,
				add_new_key_data.clone()
			),
			Error::<Test>::KeyVerificationFailed
		);
	});
}

#[test]
fn it_throws_error_when_not_msa_owner() {
	new_test_ext().execute_with(|| {
		let (key_pair, _) = sr25519::Pair::generate();
		let (key_pair_2, _) = sr25519::Pair::generate();

		let account = key_pair.public();

		let (new_msa_id, _) = Msa::create_account(account.into()).unwrap();
		assert_ok!(Msa::create_account(test_public(1).into()));

		let new_account = key_pair_2.public();

		let add_new_key_data = AddKeyData { nonce: 0, msa_id: new_msa_id };
		let encode_data_new_key_data = wrap_binary_data(add_new_key_data.encode());

		let signature: MultiSignature = key_pair_2.sign(&encode_data_new_key_data).into();

		assert_noop!(
			Msa::add_key_to_msa(
				test_origin_signed(1),
				new_account.into(),
				signature,
				add_new_key_data.clone()
			),
			Error::<Test>::NotMsaOwner
		);
	});
}

#[test]
fn it_throws_error_when_for_duplicate_key() {
	new_test_ext().execute_with(|| {
		let (key_pair, _) = sr25519::Pair::generate();

		let new_account = key_pair.public();

		let (new_msa_id, _) = Msa::create_account(new_account.into()).unwrap();

		let add_new_key_data = AddKeyData { nonce: 1, msa_id: new_msa_id };
		let encode_data_new_key_data = wrap_binary_data(add_new_key_data.encode());

		let signature: MultiSignature = key_pair.sign(&encode_data_new_key_data).into();

		assert_noop!(
			Msa::add_key_to_msa(
				Origin::signed(new_account.into()),
				new_account.into(),
				signature,
				add_new_key_data.clone()
			),
			Error::<Test>::DuplicatedKey
		);
	});
}

#[test]
fn add_key_with_more_than_allowed_should_panic() {
	new_test_ext().execute_with(|| {
		// arrange
		let (key_pair, _) = sr25519::Pair::generate();
		let account = key_pair.public();
		let (new_msa_id, _) = Msa::create_account(account.into()).unwrap();
		let add_new_key_data = AddKeyData { nonce: 1, msa_id: new_msa_id };
		let encode_data_new_key_data = wrap_binary_data(add_new_key_data.encode());

		for _ in 1..<Test as Config>::MaxKeys::get() {
			let (new_key_pair, _) = sr25519::Pair::generate();
			let new_account = new_key_pair.public();
			let signature: MultiSignature = new_key_pair.sign(&encode_data_new_key_data).into();
			assert_ok!(Msa::add_key_to_msa(
				Origin::signed(account.into()),
				new_account.into(),
				signature,
				add_new_key_data.clone()
			));
		}

		// act
		let (final_key_pair, _) = sr25519::Pair::generate();
		let final_account = final_key_pair.public();
		let signature: MultiSignature = final_key_pair.sign(&encode_data_new_key_data).into();
		assert_noop!(
			Msa::add_key_to_msa(
				Origin::signed(account.into()),
				final_account.into(),
				signature,
				add_new_key_data.clone()
			),
			Error::<Test>::KeyLimitExceeded
		);
	});
}

#[test]
fn it_logs_key_added_event() {
	new_test_ext().execute_with(|| {
		// arrange
		let (key_pair, _) = sr25519::Pair::generate();
		let (key_pair_2, _) = sr25519::Pair::generate();

		let account = key_pair.public();
		let (new_msa_id, _) = Msa::create_account(account.into()).unwrap();

		let new_key = key_pair_2.public();

		let add_new_key_data = AddKeyData { nonce: 1, msa_id: new_msa_id };
		let encode_data_new_key_data = wrap_binary_data(add_new_key_data.encode());

		let signature: MultiSignature = key_pair_2.sign(&encode_data_new_key_data).into();

		// act
		assert_ok!(Msa::add_key_to_msa(
			Origin::signed(account.into()),
			new_key.into(),
			signature,
			add_new_key_data.clone(),
		));

		// assert
		let keys = Msa::fetch_msa_keys(new_msa_id);
		assert_eq!(keys.len(), 2);
		assert_eq!{keys.contains(&KeyInfoResponse {key: AccountId32::from(new_key), msa_id: new_msa_id, nonce: 0, expired: 0}), true}
		System::assert_last_event(Event::KeyAdded { msa_id: 1, key: new_key.into() }.into());
	});
}

#[test]
fn it_revokes_msa_key_successfully() {
	new_test_ext().execute_with(|| {
		assert_ok!(Msa::add_key(2, &test_public(1)));
		assert_ok!(Msa::add_key(2, &test_public(2)));

		assert_ok!(Msa::revoke_msa_key(test_origin_signed(1), test_public(2)));

		let info = Msa::get_key_info(&test_public(2));

		assert_eq!(info, Some(KeyInfo { msa_id: 2, expired: 1, nonce: 0 }));

		System::assert_last_event(Event::KeyRevoked { key: test_public(2).into() }.into());
	})
}

#[test]
pub fn test_get_onwner_of() {
	new_test_ext().execute_with(|| {
		assert_eq!(Msa::get_owner_of(&test_public(1)), None);

		assert_ok!(Msa::create(test_origin_signed(1)));

		assert_eq!(Msa::get_owner_of(&test_public(1)), Some(1));
	});
}

#[test]
pub fn test_revoke_key() {
	new_test_ext().execute_with(|| {
		assert_ok!(Msa::add_key(1, &test_public(1)));

		let info = Msa::get_key_info(&test_public(1));
		assert_eq!(info, Some(KeyInfo { msa_id: 1, expired: 0, nonce: 0 }));

		assert_ok!(Msa::revoke_key(&test_public(1)));

		let info = Msa::get_key_info(&test_public(1));

		assert_eq!(info, Some(KeyInfo { msa_id: 1, expired: 1, nonce: 0 }));
	});
}

#[test]
pub fn test_revoke_key_errors() {
	new_test_ext().execute_with(|| {
		assert_noop!(Msa::revoke_key(&test_public(1)), Error::<Test>::NoKeyExists);

		assert_ok!(Msa::add_key(1, &test_public(1)));
		assert_ok!(Msa::revoke_key(&test_public(1)));

		assert_noop!(Msa::revoke_key(&test_public(1)), Error::<Test>::KeyRevoked);
	});
}

#[test]
pub fn test_ensure_msa_owner() {
	new_test_ext().execute_with(|| {
		assert_noop!(Msa::ensure_msa_owner(&test_public(1), 1), Error::<Test>::NoKeyExists);

		assert_ok!(Msa::add_key(1, &test_public(1)));

		assert_eq!(Msa::ensure_msa_owner(&test_public(1), 1), Ok(()));
	});
}
