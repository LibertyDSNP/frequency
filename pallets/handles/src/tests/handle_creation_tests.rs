use crate::{utils::converter::HandleConverter, tests::mock::*, Error};
use codec::Decode;
use common_primitives::{handles::*, msa::MessageSourceId, utils::wrap_binary_data};
use frame_support::{assert_noop, assert_ok};
use sp_core::{sr25519, Encode, Pair};
use sp_runtime::MultiSignature;

fn get_signed_claims_payload(
	account: &sr25519::Pair,
	handle: Vec<u8>,
) -> (ClaimHandlePayload, MultiSignature) {
	let base_handle = handle;
	let payload = ClaimHandlePayload::new(base_handle.clone());
	let encoded_payload = wrap_binary_data(payload.encode());
	let proof: MultiSignature = account.sign(&encoded_payload).into();

	(payload, proof)
}

#[test]
fn test_full_handle_creation() {
	new_test_ext().execute_with(|| {
		for sequence_index in 0..100 {
			let full_handle = Handles::create_full_handle("test", sequence_index);
			let full_handle_str = core::str::from_utf8(&full_handle).ok().unwrap();
			println!("full_handle_str={}", full_handle_str);
		}
	})
}

#[test]
fn claim_handle_happy_path() {
	new_test_ext().execute_with(|| {
		let alice = sr25519::Pair::from_seed(&[0; 32]);
		let (payload, proof) = get_signed_claims_payload(&alice, "test1".as_bytes().to_vec());
		assert_ok!(Handles::claim_handle(
			RuntimeOrigin::signed(alice.public().into()),
			alice.public().into(),
			proof,
			payload
		));
	});
}

#[test]
fn claim_handle_already_claimed() {
	new_test_ext().execute_with(|| {
		let alice = sr25519::Pair::from_seed(&[0; 32]);
		let (payload, proof) = get_signed_claims_payload(&alice, "test1".as_bytes().to_vec());
		assert_ok!(Handles::claim_handle(
			RuntimeOrigin::signed(alice.public().into()),
			alice.public().into(),
			proof,
			payload.clone()
		));

		let (payload, proof) = get_signed_claims_payload(&alice, "test1".as_bytes().to_vec());
		assert_noop!(
			Handles::claim_handle(
				RuntimeOrigin::signed(alice.public().into()),
				alice.public().into(),
				proof,
				payload
			),
			Error::<Test>::MSAHandleAlreadyExists
		);
	});
}

#[test]
fn claim_handle_already_claimed_with_different_case() {
	new_test_ext().execute_with(|| {
		let alice = sr25519::Pair::from_seed(&[0; 32]);
		let (payload, proof) = get_signed_claims_payload(&alice, "test1".as_bytes().to_vec());
		assert_ok!(Handles::claim_handle(
			RuntimeOrigin::signed(alice.public().into()),
			alice.public().into(),
			proof,
			payload.clone()
		));

		let (payload, proof) = get_signed_claims_payload(&alice, "TEST1".as_bytes().to_vec());
		assert_noop!(
			Handles::claim_handle(
				RuntimeOrigin::signed(alice.public().into()),
				alice.public().into(),
				proof,
				payload
			),
			Error::<Test>::MSAHandleAlreadyExists
		);
	});
}

#[test]
fn claim_handle_already_claimed_with_homoglyph() {
	new_test_ext().execute_with(|| {
		let alice = sr25519::Pair::from_seed(&[0; 32]);
		let (payload, proof) = get_signed_claims_payload(&alice, "test1".as_bytes().to_vec());
		assert_ok!(Handles::claim_handle(
			RuntimeOrigin::signed(alice.public().into()),
			alice.public().into(),
			proof,
			payload.clone()
		));

		let (payload, proof) = get_signed_claims_payload(&alice, "tést1".as_bytes().to_vec());
		assert_noop!(
			Handles::claim_handle(
				RuntimeOrigin::signed(alice.public().into()),
				alice.public().into(),
				proof,
				payload
			),
			Error::<Test>::MSAHandleAlreadyExists
		);
	});
}

#[test]
fn claim_handle_get_msa_handle() {
	new_test_ext().execute_with(|| {
		let alice = sr25519::Pair::from_seed(&[0; 32]);
		let (payload, proof) = get_signed_claims_payload(&alice, "test1".as_bytes().to_vec());
		assert_ok!(Handles::claim_handle(
			RuntimeOrigin::signed(alice.public().into()),
			alice.public().into(),
			proof,
			payload
		));
		let msa_id = MessageSourceId::decode(&mut &alice.public().encode()[..]).unwrap();
		let handle = Handles::get_handle_for_msa(msa_id);
		assert!(handle.is_some());
		let handle_result = handle.unwrap();
		let base_handle = handle_result.base_handle;
		assert_eq!(base_handle, "test1".as_bytes().to_vec());
	});
}
