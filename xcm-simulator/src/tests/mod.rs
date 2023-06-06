use super::*;

use codec::Encode;
use dip_support::*;
use frame_support::assert_ok;
use xcm::latest::prelude::*;
use xcm_simulator::TestExt;
// use sp_runtime::AccountId32;
use sp_runtime::testing::H256;

// Helper function for forming buy execution message
fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
	BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

// #[test]
// fn dmp() {
// 	MockNet::reset();

// 	let remark = parachain::RuntimeCall::System(
// 		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
// 	);
// 	Relay::execute_with(|| {
// 		assert_ok!(RelayChainPalletXcm::send_xcm(
// 			Here,
// 			Parachain(1),
// 			Xcm(vec![Transact {
// 				origin_type: OriginKind::SovereignAccount,
// 				require_weight_at_most: INITIAL_BALANCE as u64,
// 				call: remark.encode().into(),
// 			}]),
// 		));
// 	});

// 	ParaA::execute_with(|| {
// 		use parachain::{RuntimeEvent, System};
// 		assert!(System::events().iter().any(|r| matches!(
// 			r.event,
// 			RuntimeEvent::System(frame_system::Event::Remarked { .. })
// 		)));
// 	});
// }

// #[test]
// fn ump() {
// 	MockNet::reset();

// 	let remark = relay_chain::RuntimeCall::System(
// 		frame_system::Call::<relay_chain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
// 	);
// 	ParaA::execute_with(|| {
// 		assert_ok!(ParachainPalletXcm::send_xcm(
// 			Here,
// 			Parent,
// 			Xcm(vec![Transact {
// 				origin_type: OriginKind::SovereignAccount,
// 				require_weight_at_most: INITIAL_BALANCE as u64,
// 				call: remark.encode().into(),
// 			}]),
// 		));
// 	});

// 	Relay::execute_with(|| {
// 		use relay_chain::{RuntimeEvent, System};
// 		assert!(System::events().iter().any(|r| matches!(
// 			r.event,
// 			RuntimeEvent::System(frame_system::Event::Remarked { .. })
// 		)));
// 	});
// }

// #[test]
// fn xcmp() {
// 	MockNet::reset();

// 	let remark = parachain::RuntimeCall::System(
// 		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
// 	);
// 	ParaA::execute_with(|| {
// 		assert_ok!(ParachainPalletXcm::send_xcm(
// 			Here,
// 			(Parent, Parachain(2)),
// 			Xcm(vec![Transact {
// 				origin_type: OriginKind::SovereignAccount,
// 				require_weight_at_most: INITIAL_BALANCE as u64,
// 				call: remark.encode().into(),
// 			}]),
// 		));
// 	});

// 	ParaB::execute_with(|| {
// 		use parachain::{RuntimeEvent, System};
// 		assert!(System::events().iter().any(|r| matches!(
// 			r.event,
// 			RuntimeEvent::System(frame_system::Event::Remarked { .. })
// 		)));
// 	});
// }

// // #[test]
// // fn reserve_transfer() {
// // 	MockNet::reset();

// // 	let withdraw_amount = 123;

// // 	Relay::execute_with(|| {
// // 		assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
// // 			relay_chain::RuntimeOrigin::signed(ALICE),
// // 			Box::new(X1(Parachain(1)).into().into()),
// // 			Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
// // 			Box::new((Here, withdraw_amount).into()),
// // 			0,
// // 		));
// // 		assert_eq!(
// // 			parachain::Balances::free_balance(&para_account_id(1)),
// // 			INITIAL_BALANCE + withdraw_amount
// // 		);
// // 	});

// // 	ParaA::execute_with(|| {
// // 		// free execution, full amount received
// // 		assert_eq!(
// // 			pallet_balances::Pallet::<parachain::Runtime>::free_balance(&ALICE),
// // 			INITIAL_BALANCE + withdraw_amount
// // 		);
// // 	});
// // }

// /// Scenario:
// /// A parachain transfers funds on the relay chain to another parachain account.
// ///
// /// Asserts that the parachain accounts are updated as expected.
// #[test]
// fn withdraw_and_deposit() {
// 	MockNet::reset();

// 	let send_amount = 100_000_000;

// 	ParaA::execute_with(|| {
// 		let message = Xcm(vec![
// 			WithdrawAsset((Here, send_amount).into()),
// 			buy_execution((Here, send_amount)),
// 			DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Parachain(2).into() },
// 		]);
// 		// Send withdraw and deposit
// 		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
// 	});

// 	Relay::execute_with(|| {
// 		assert_eq!(
// 			relay_chain::Balances::free_balance(para_account_id(1)),
// 			INITIAL_BALANCE - send_amount
// 		);
// 		assert_eq!(relay_chain::Balances::free_balance(para_account_id(2)), send_amount);
// 	});
// }

// /// Scenario:
// /// A parachain wants to be notified that a transfer worked correctly.
// /// It sends a `QueryHolding` after the deposit to get notified on success.
// ///
// /// Asserts that the balances are updated correctly and the expected XCM is sent.
// #[test]
// fn query_holding() {
// 	MockNet::reset();

// 	let send_amount = 100_000_000;
// 	let query_id_set = 1234;

// 	// Send a message which fully succeeds on the relay chain
// 	ParaA::execute_with(|| {
// 		let message = Xcm(vec![
// 			WithdrawAsset((Here, send_amount).into()),
// 			buy_execution((Here, send_amount)),
// 			DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Parachain(2).into() },
// 			QueryHolding {
// 				query_id: query_id_set,
// 				dest: Parachain(1).into(),
// 				assets: All.into(),
// 				max_response_weight: 1_000_000_000,
// 			},
// 		]);
// 		// Send withdraw and deposit with query holding
// 		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
// 	});

// 	// Check that transfer was executed
// 	Relay::execute_with(|| {
// 		// Withdraw executed
// 		assert_eq!(
// 			relay_chain::Balances::free_balance(para_account_id(1)),
// 			INITIAL_BALANCE - send_amount
// 		);
// 		// Deposit executed
// 		assert_eq!(relay_chain::Balances::free_balance(para_account_id(2)), send_amount);
// 	});

// 	// Check that QueryResponse message was received
// 	ParaA::execute_with(|| {
// 		assert_eq!(
// 			parachain::MsgQueue::received_dmp(),
// 			vec![Xcm(vec![QueryResponse {
// 				query_id: query_id_set,
// 				response: Response::Assets(MultiAssets::new()),
// 				max_weight: 1_000_000_000,
// 			}])],
// 		);
// 	});
// }

#[test]
fn frequency_xcmp() {
	MockNet::reset();

	let send_amount = 6_000_000_000;

	let proccess_identity_call = frequency::RuntimeCall::DipConsumer(pallet_dip_consumer::Call::<
		frequency::Runtime,
	>::process_identity_action {
		action: IdentityDetailsAction::Updated([0; 32].into(), H256::zero(), ()),
	});
	ParaA::execute_with(|| {
		let xcm: Xcm<()> = Xcm(vec![
			DescendOrigin(X1(AccountId32 { network: NetworkId::Any, id: [0; 32] })),
			WithdrawAsset((Here, send_amount).into()),
			// to make test with Limited
			// BuyExecution { fees: (Here, 7_000_000u128).into(), weight_limit: Limited(4000003333 * 2 * 1)},
			BuyExecution { fees: (Here, 7_000_000u128).into(), weight_limit: Limited(4000003333)},
			// BuyExecution { fees: (Here, INITIAL_BALANCE).into(), weight_limit: Unlimited },
			// SetAppendix(Xcm(vec![RefundSurplus])),
			// SetAppendix(Xcm(vec![RefundSurplus, 
				// DepositAsset {
				// 	max_assets: 1,
				// 	assets: Wild(All),
				// 	beneficiary: MultiLocation {
				// 		parents: 1,
				// 		interior: X1(AccountId32 { network: NetworkId::Any, id: [0; 32] }),
				// 	}
				// }
			// ])),
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 3333 as u64,
				call: proccess_identity_call.encode().into(),
			},
			// RefundSurplus
		]);

		assert_ok!(ParachainPalletXcm::send_xcm(Here, (Parent, Parachain(3)), xcm,));
	});

	Frequency::execute_with(|| {
		use frequency::{RuntimeEvent, System};

		let xcms = parachain::MsgQueue::received_xcmp();
		println!("xcms[0].0.as_slice() {:?}", xcms[0].0.as_slice());
		assert!(xcms.len() == 1, "Expected only one XCMP message, found {}", xcms.len());

		assert_eq!(
			frequency::Balances::free_balance(alice_foreign_alias_account()),
			INITIAL_BALANCE - send_amount
		);

		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::DipConsumer(pallet_dip_consumer::Event::IdentityInfoUpdated { .. })
		)));

		System::reset_events();
	});
}