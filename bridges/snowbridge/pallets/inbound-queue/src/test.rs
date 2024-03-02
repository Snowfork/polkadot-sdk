// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use super::*;

use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use snowbridge_core::{
	inbound::{Log, Proof},
	ChannelId,
};
use sp_keyring::AccountKeyring as Keyring;
use sp_runtime::DispatchError;
use sp_std::convert::From;

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
fn test_submit_no_funds_to_reward_relayers_just_ignore() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);

		// Reset balance of sovereign_account to zero first
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
		// Check submit successfully in case no funds available
		assert_ok!(InboundQueue::submit(origin.clone(), message.clone()));
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
fn test_submit_no_funds_to_reward_relayers_and_ed_preserved() {
	new_tester().execute_with(|| {
		let relayer: AccountId = Keyring::Bob.into();
		let origin = RuntimeOrigin::signed(relayer);

		// Reset balance of sovereign account to (ED+1) first
		let sovereign_account = sibling_sovereign_account::<Test>(ASSET_HUB_PARAID.into());
		Balances::set_balance(&sovereign_account, ExistentialDeposit::get() + 1);

		// Submit message successfully
		let message = Message {
			event_log: mock_event_log(),
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};
		assert_ok!(InboundQueue::submit(origin.clone(), message.clone()));

		// Check balance of sovereign account to ED
		let amount = Balances::balance(&sovereign_account);
		assert_eq!(amount, ExistentialDeposit::get());

		// Submit another message with nonce set as 2
		let mut event_log = mock_event_log();
		event_log.data[31] = 2;
		let message = Message {
			event_log,
			proof: Proof {
				block_hash: Default::default(),
				tx_index: Default::default(),
				data: Default::default(),
			},
		};
		assert_ok!(InboundQueue::submit(origin.clone(), message.clone()));
		// Check balance of sovereign account as ED does not change
		let amount = Balances::balance(&sovereign_account);
		assert_eq!(amount, ExistentialDeposit::get());
	});
}

#[test]
#[should_panic]
fn test_decode_event_log() {
	new_tester().execute_with(|| {
		// data from https://bridgehub-rococo.stg.subscan.io/extrinsic/2583529-2
		let event_log = Log {
			address: hex!("5b4909ce6ca82d2ce23bd46738953c7959e710cd").into(),
			topics: vec![
				hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
				hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
				hex!("e1f34efbe357e26b778c73724e9a64777d95cf5b54e146de8e8ea9a78cf7cda7").into(),
			],
			data: hex!("000000000000000000000000000000000000000000000000000000000000001d0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300a736aa000000000001fff9976782d46cc05630d1f6ebab18b2324d6b1401ee070000904411b468d36fc59a7440fac24170a40f5c74d3a6571cf3de326300cd43874a0000000000000000000000000000000000ac23fc06000000000000000000000000e40b5402000000000000000000000000000000000000000000000000").into(),
		};
		let envelope =
			Envelope::try_from(&event_log).map_err(|_| Error::<Test>::InvalidEnvelope).unwrap();
		let message = inbound::VersionedMessage::decode_all(&mut envelope.payload.as_ref()).map_err(|_| Error::<Test>::InvalidPayload).unwrap();
		let _ = InboundQueue::do_convert(envelope.message_id, message);
	});
}
