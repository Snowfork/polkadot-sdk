// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use super::*;

use frame_support::{assert_noop, assert_ok, weights::Weight};
use hex_literal::hex;
use snowbridge_core::{inbound::Proof, ChannelId};
use sp_keyring::AccountKeyring as Keyring;
use sp_runtime::{DispatchError, TokenError};
use sp_std::convert::From;
use xcm::prelude::{OriginKind, Transact};

use crate::{Error, Event as InboundQueueEvent};

use crate::mock::*;

#[test]
fn test_submit_happy_path() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let channel_sovereign = sibling_sovereign_account::<Test>(ASSET_HUB_PARAID.into());

		let origin = RuntimeOrigin::signed(relayer.clone());

		// Submit message
		let message = Message {
			event_log: mock_event_log(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};

		let initial_fund = InitialFund::get();
		assert_eq!(Balances::balance(&relayer), 0);
		assert_eq!(Balances::balance(&channel_sovereign), initial_fund);

		assert_ok!(InboundQueue::submit(origin.clone(), message.clone()));
		expect_events(vec![InboundQueueEvent::MessageReceived {
			channel_id: hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539")
				.into(),
			nonce: 1,
			message_id: [
				57, 61, 232, 3, 66, 61, 25, 190, 234, 188, 193, 174, 13, 186, 1, 64, 237, 94, 73,
				83, 14, 18, 209, 213, 78, 121, 43, 108, 251, 245, 107, 67,
			],
			fee_burned: 110000000000,
		}
		.into()]);

		let delivery_cost = InboundQueue::calculate_delivery_cost(message.encode().len() as u32);
		assert!(
			Parameters::get().rewards.local < delivery_cost,
			"delivery cost exceeds pure reward"
		);

		assert_eq!(Balances::balance(&relayer), delivery_cost, "relayer was rewarded");
		assert!(
			Balances::balance(&channel_sovereign) <= initial_fund - delivery_cost,
			"sovereign account paid reward"
		);
	});
}

#[test]
fn test_submit_xcm_invalid_channel() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);

		// Deposit funds into sovereign account of parachain 1001
		let sovereign_account = sibling_sovereign_account::<Test>(TEMPLATE_PARAID.into());
		println!("account: {}", sovereign_account);
		let _ = Balances::mint_into(&sovereign_account, 10000);

		// Submit message
		let message = Message {
			event_log: mock_event_log_invalid_channel(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};
		assert_noop!(
			InboundQueue::submit(origin.clone(), message.clone()),
			Error::<Test>::InvalidChannel,
		);
	});
}

#[test]
fn test_submit_with_invalid_gateway() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);

		// Deposit funds into sovereign account of Asset Hub (Statemint)
		let sovereign_account = sibling_sovereign_account::<Test>(ASSET_HUB_PARAID.into());
		let _ = Balances::mint_into(&sovereign_account, 10000);

		// Submit message
		let message = Message {
			event_log: mock_event_log_invalid_gateway(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};
		assert_noop!(
			InboundQueue::submit(origin.clone(), message.clone()),
			Error::<Test>::InvalidGateway
		);
	});
}

#[test]
fn test_submit_with_invalid_nonce() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);

		// Deposit funds into sovereign account of Asset Hub (Statemint)
		let sovereign_account = sibling_sovereign_account::<Test>(ASSET_HUB_PARAID.into());
		let _ = Balances::mint_into(&sovereign_account, 10000);

		// Submit message
		let message = Message {
			event_log: mock_event_log(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};
		assert_ok!(InboundQueue::submit(origin.clone(), message.clone()));

		let nonce: u64 = <Nonce<Test>>::get(ChannelId::from(hex!(
			"c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539"
		)));
		assert_eq!(nonce, 1);

		// Submit the same again
		assert_noop!(
			InboundQueue::submit(origin.clone(), message.clone()),
			Error::<Test>::InvalidNonce
		);
	});
}

#[test]
fn test_submit_no_funds_to_reward_relayers() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);

		// Reset balance of sovereign_account to zero so to trigger the FundsUnavailable error
		let sovereign_account = sibling_sovereign_account::<Test>(ASSET_HUB_PARAID.into());
		Balances::set_balance(&sovereign_account, 0);

		// Submit message
		let message = Message {
			event_log: mock_event_log(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};
		assert_noop!(
			InboundQueue::submit(origin.clone(), message.clone()),
			TokenError::FundsUnavailable
		);
	});
}

#[test]
fn test_set_operating_mode() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);
		let message = Message {
			event_log: mock_event_log(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};

		assert_ok!(InboundQueue::set_operating_mode(
			RuntimeOrigin::root(),
			snowbridge_core::BasicOperatingMode::Halted
		));

		assert_noop!(InboundQueue::submit(origin, message), Error::<Test>::Halted);
	});
}

#[test]
fn test_set_operating_mode_root_only() {
	new_tester().execute_with(|| {
		assert_noop!(
			InboundQueue::set_operating_mode(
				RuntimeOrigin::signed(Keyring::Bob.into()),
				snowbridge_core::BasicOperatingMode::Halted
			),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn test_convert_transact() {
	new_tester().execute_with(|| {
		let message_id: H256 = [1; 32].into();
		let sender: H160 = hex!("ee9170abfbf9421ad6dd07f6bdec9d89f2b581e0").into();
		let fee: u128 = 40_000_000_000;
		let weight_at_most = Weight::from_parts(40_000_000, 8_000);
		let origin_kind = OriginKind::SovereignAccount;
		let payload = hex!("00071468656c6c6f").to_vec();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: 11155111,
			command: Command::Transact {
				sender,
				fee,
				weight_ref_time: weight_at_most.ref_time(),
				weight_proof_size: weight_at_most.proof_size(),
				origin_kind,
				payload: payload.clone(),
			},
		});
		// Convert the message to XCM
		let (xcm, dest_fee) = InboundQueue::do_convert(message_id, message).unwrap();
		let instructions = xcm.into_inner();
		assert_eq!(instructions.len(), 8);
		assert_eq!(dest_fee, fee.into());
		let transact = instructions.get(6).unwrap().clone();
		let expected =
			Transact { origin_kind, require_weight_at_most: weight_at_most, call: payload.into() };
		assert_eq!(transact, expected);
	});
}
