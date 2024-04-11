// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::imports::*;
use bridge_hub_rococo_runtime::{EthereumBeaconClient, EthereumInboundQueue, RuntimeOrigin};
use codec::{Decode, Encode};
use emulated_integration_tests_common::xcm_emulator::ConvertLocation;
use frame_support::pallet_prelude::TypeInfo;
use hex_literal::hex;
use rococo_system_emulated_network::penpal_emulated_chain::CustomizableAssetFromSystemAssetHub;
use rococo_westend_system_emulated_network::BridgeHubRococoParaSender as BridgeHubRococoSender;
use snowbridge_core::{inbound::InboundQueueFixture, outbound::OperatingMode, Channel, ChannelId};
use snowbridge_pallet_inbound_queue_fixtures::{
	register_token::make_register_token_message, send_token::make_send_token_message,
	send_token_to_penpal::make_send_token_to_penpal_message,
};
use snowbridge_pallet_system;
use snowbridge_router_primitives::inbound::{
	Command, ConvertMessage, GlobalConsensusEthereumConvertsFor, MessageV1, VersionedMessage,
};
use sp_core::{H160, H256};
use sp_io::hashing::blake2_256;
use sp_runtime::{DispatchError::Token, TokenError::FundsUnavailable};
use testnet_parachains_constants::rococo::snowbridge::EthereumNetwork;

const INITIAL_FUND: u128 = 5_000_000_000 * ROCOCO_ED;
const CHAIN_ID: u64 = 11155111;
const TREASURY_ACCOUNT: [u8; 32] =
	hex!("6d6f646c70792f74727372790000000000000000000000000000000000000000");
const WETH: [u8; 20] = hex!("87d1f7fdfEe7f651FaBc8bFCB6E086C278b77A7d");
const ETHEREUM_DESTINATION_ADDRESS: [u8; 20] = hex!("44a57ee2f2FCcb85FDa2B0B18EBD0D8D2333700e");
const XCM_FEE: u128 = 40_000_000_000;
const XCM_WEIGHT: Weight = Weight::from_parts(40_000_000, 8_000);
const INSUFFICIENT_XCM_FEE: u128 = 1_000;
const INSUFFICIENT_XCM_WEIGHT: Weight = Weight::from_parts(1_000, 1_000);

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum ControlCall {
	#[codec(index = 3)]
	CreateAgent,
	#[codec(index = 4)]
	CreateChannel { mode: OperatingMode },
}

#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum SnowbridgeControl {
	#[codec(index = 83)]
	Control(ControlCall),
}

pub fn send_inbound_message(fixture: InboundQueueFixture) -> DispatchResult {
	EthereumBeaconClient::store_finalized_header(
		fixture.finalized_header,
		fixture.block_roots_root,
	)
	.unwrap();
	EthereumInboundQueue::submit(
		RuntimeOrigin::signed(BridgeHubRococoSender::get()),
		fixture.message,
	)
}

/// Create an agent on Ethereum. An agent is a representation of an entity in the Polkadot
/// ecosystem (like a parachain) on Ethereum.
#[test]
fn create_agent() {
	let origin_para: u32 = 2000;
	// Fund the origin parachain sovereign account so that it can pay execution fees.
	BridgeHubRococo::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let destination = Rococo::child_location_of(BridgeHubRococo::para_id()).into();

	let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
	// Construct XCM to create an agent for para 1001
	let remote_xcm = VersionedXcm::from(Xcm::<()>(vec![
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		DescendOrigin(Parachain(origin_para).into()),
		Transact {
			require_weight_at_most: 3000000000.into(),
			origin_kind: OriginKind::Xcm,
			call: create_agent_call.encode().into(),
		},
	]));

	// Rococo Global Consensus
	// Send XCM message from Relay Chain to Bridge Hub source Parachain
	Rococo::execute_with(|| {
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send_blob(
			sudo_origin,
			bx!(destination),
			remote_xcm.encode().try_into().unwrap(),
		));

		type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;
		// Check that the Transact message was sent
		assert_expected_events!(
			Rococo,
			vec![
				RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		// Check that a message was sent to Ethereum to create the agent
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::CreateAgent {
					..
				}) => {},
			]
		);
	});
}

/// Create a channel for a consensus system. A channel is a bidirectional messaging channel
/// between BridgeHub and Ethereum.
#[test]
fn create_channel() {
	let origin_para: u32 = 2000;
	// Fund AssetHub sovereign account so that it can pay execution fees.
	BridgeHubRococo::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let destination: VersionedLocation =
		Rococo::child_location_of(BridgeHubRococo::para_id()).into();

	let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
	// Construct XCM to create an agent for para 1001
	let create_agent_xcm = VersionedXcm::from(Xcm::<()>(vec![
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		DescendOrigin(Parachain(origin_para).into()),
		Transact {
			require_weight_at_most: 3000000000.into(),
			origin_kind: OriginKind::Xcm,
			call: create_agent_call.encode().into(),
		},
	]));

	let create_channel_call =
		SnowbridgeControl::Control(ControlCall::CreateChannel { mode: OperatingMode::Normal });
	// Construct XCM to create a channel for para 1001
	let create_channel_xcm = VersionedXcm::from(Xcm::<()>(vec![
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		DescendOrigin(Parachain(origin_para).into()),
		Transact {
			require_weight_at_most: 3000000000.into(),
			origin_kind: OriginKind::Xcm,
			call: create_channel_call.encode().into(),
		},
	]));

	// Rococo Global Consensus
	// Send XCM message from Relay Chain to Bridge Hub source Parachain
	Rococo::execute_with(|| {
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send_blob(
			sudo_origin.clone(),
			bx!(destination.clone()),
			create_agent_xcm.encode().try_into().unwrap(),
		));

		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send_blob(
			sudo_origin,
			bx!(destination),
			create_channel_xcm.encode().try_into().unwrap(),
		));

		type RuntimeEvent = <Rococo as Chain>::RuntimeEvent;

		assert_expected_events!(
			Rococo,
			vec![
				RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

		// Check that the Channel was created
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::CreateChannel {
					..
				}) => {},
			]
		);
	});
}

/// Tests the registering of a token as an asset on AssetHub.
#[test]
fn register_weth_token_from_ethereum_to_asset_hub() {
	// Fund AssetHub sovereign account so that it can pay execution fees.
	BridgeHubRococo::fund_para_sovereign(AssetHubRococo::para_id().into(), INITIAL_FUND);

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

		// Construct RegisterToken message and sent to inbound queue
		let register_token_message = make_register_token_message();
		send_inbound_message(register_token_message.clone()).unwrap();

		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubRococo::execute_with(|| {
		type RuntimeEvent = <AssetHubRococo as Chain>::RuntimeEvent;

		assert_expected_events!(
			AssetHubRococo,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Created { .. }) => {},
			]
		);
	});
}

/// Tests the registering of a token as an asset on AssetHub, and then subsequently sending
/// a token from Ethereum to AssetHub.
#[test]
fn send_token_from_ethereum_to_asset_hub() {
	BridgeHubRococo::fund_para_sovereign(AssetHubRococo::para_id().into(), INITIAL_FUND);

	// Fund ethereum sovereign on AssetHub
	AssetHubRococo::fund_accounts(vec![(AssetHubRococoReceiver::get(), INITIAL_FUND)]);

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

		// Construct RegisterToken message and sent to inbound queue
		send_inbound_message(make_register_token_message()).unwrap();

		// Construct SendToken message and sent to inbound queue
		send_inbound_message(make_send_token_message()).unwrap();

		// Check that the message was sent
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubRococo::execute_with(|| {
		type RuntimeEvent = <AssetHubRococo as Chain>::RuntimeEvent;

		// Check that the token was received and issued as a foreign asset on AssetHub
		assert_expected_events!(
			AssetHubRococo,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Issued { .. }) => {},
			]
		);
	});
}

/// Tests sending a token to a 3rd party parachain, called PenPal. The token reserve is
/// still located on AssetHub.
#[test]
fn send_token_from_ethereum_to_penpal() {
	let asset_hub_sovereign = BridgeHubRococo::sovereign_account_id_of(Location::new(
		1,
		[Parachain(AssetHubRococo::para_id().into())],
	));
	// Fund AssetHub sovereign account so it can pay execution fees for the asset transfer
	BridgeHubRococo::fund_accounts(vec![(asset_hub_sovereign.clone(), INITIAL_FUND)]);

	// Fund PenPal sender and receiver
	PenpalA::fund_accounts(vec![
		(PenpalAReceiver::get(), INITIAL_FUND),
		(PenpalASender::get(), INITIAL_FUND),
	]);

	PenpalA::execute_with(|| {
		assert_ok!(<PenpalA as Chain>::System::set_storage(
			<PenpalA as Chain>::RuntimeOrigin::root(),
			vec![(
				CustomizableAssetFromSystemAssetHub::key().to_vec(),
				Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]).encode(),
			)],
		));
	});

	// The Weth asset location, identified by the contract address on Ethereum
	let weth_asset_location: Location =
		(Parent, Parent, EthereumNetwork::get(), AccountKey20 { network: None, key: WETH }).into();
	// Converts the Weth asset location into an asset ID
	let weth_asset_id: v3::Location = weth_asset_location.try_into().unwrap();

	let origin_location = (Parent, Parent, EthereumNetwork::get()).into();

	// Fund ethereum sovereign on AssetHub
	let ethereum_sovereign: AccountId =
		GlobalConsensusEthereumConvertsFor::<AccountId>::convert_location(&origin_location)
			.unwrap();
	AssetHubRococo::fund_accounts(vec![(ethereum_sovereign.clone(), INITIAL_FUND)]);

	// Create asset on the Penpal parachain.
	PenpalA::execute_with(|| {
		assert_ok!(<PenpalA as PenpalAPallet>::ForeignAssets::create(
			<PenpalA as Chain>::RuntimeOrigin::signed(PenpalASender::get()),
			weth_asset_id,
			asset_hub_sovereign.into(),
			1000,
		));

		assert!(<PenpalA as PenpalAPallet>::ForeignAssets::asset_exists(weth_asset_id));
	});

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

		// Construct RegisterToken message and sent to inbound queue
		send_inbound_message(make_register_token_message()).unwrap();

		// Construct SendToken message to AssetHub(only for increase the nonce as the same order in
		// smoke test)
		send_inbound_message(make_send_token_message()).unwrap();

		// Construct SendToken message and sent to inbound queue
		send_inbound_message(make_send_token_to_penpal_message()).unwrap();

		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubRococo::execute_with(|| {
		type RuntimeEvent = <AssetHubRococo as Chain>::RuntimeEvent;
		// Check that the assets were issued on AssetHub
		assert_expected_events!(
			AssetHubRococo,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Issued { .. }) => {},
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	PenpalA::execute_with(|| {
		type RuntimeEvent = <PenpalA as Chain>::RuntimeEvent;
		// Check that the assets were issued on PenPal
		assert_expected_events!(
			PenpalA,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Issued { .. }) => {},
			]
		);
	});
}

/// Tests the full cycle of token transfers:
/// - registering a token on AssetHub
/// - sending a token to AssetHub
/// - returning the token to Ethereum
#[test]
fn send_weth_asset_from_asset_hub_to_ethereum() {
	use asset_hub_rococo_runtime::xcm_config::bridging::to_ethereum::DefaultBridgeHubEthereumBaseFee;
	let assethub_sovereign = BridgeHubRococo::sovereign_account_id_of(Location::new(
		1,
		[Parachain(AssetHubRococo::para_id().into())],
	));

	AssetHubRococo::force_default_xcm_version(Some(XCM_VERSION));
	BridgeHubRococo::force_default_xcm_version(Some(XCM_VERSION));
	AssetHubRococo::force_xcm_version(
		Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]),
		XCM_VERSION,
	);

	BridgeHubRococo::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);
	AssetHubRococo::fund_accounts(vec![(AssetHubRococoReceiver::get(), INITIAL_FUND)]);

	const WETH_AMOUNT: u128 = 1_000_000_000;

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

		// Construct RegisterToken message and sent to inbound queue
		send_inbound_message(make_register_token_message()).unwrap();

		// Check that the register token message was sent using xcm
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);

		// Construct SendToken message and sent to inbound queue
		send_inbound_message(make_send_token_message()).unwrap();

		// Check that the send token message was sent using xcm
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubRococo::execute_with(|| {
		type RuntimeEvent = <AssetHubRococo as Chain>::RuntimeEvent;
		type RuntimeOrigin = <AssetHubRococo as Chain>::RuntimeOrigin;

		// Check that AssetHub has issued the foreign asset
		assert_expected_events!(
			AssetHubRococo,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Issued { .. }) => {},
			]
		);
		let assets = vec![Asset {
			id: AssetId(Location::new(
				2,
				[
					GlobalConsensus(Ethereum { chain_id: CHAIN_ID }),
					AccountKey20 { network: None, key: WETH },
				],
			)),
			fun: Fungible(WETH_AMOUNT),
		}];
		let multi_assets = VersionedAssets::V4(Assets::from(assets));

		let destination = VersionedLocation::V4(Location::new(
			2,
			[GlobalConsensus(Ethereum { chain_id: CHAIN_ID })],
		));

		let beneficiary = VersionedLocation::V4(Location::new(
			0,
			[AccountKey20 { network: None, key: ETHEREUM_DESTINATION_ADDRESS.into() }],
		));

		let free_balance_before = <AssetHubRococo as AssetHubRococoPallet>::Balances::free_balance(
			AssetHubRococoReceiver::get(),
		);
		// Send the Weth back to Ethereum
		<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::reserve_transfer_assets(
			RuntimeOrigin::signed(AssetHubRococoReceiver::get()),
			Box::new(destination),
			Box::new(beneficiary),
			Box::new(multi_assets),
			0,
		)
		.unwrap();
		let free_balance_after = <AssetHubRococo as AssetHubRococoPallet>::Balances::free_balance(
			AssetHubRococoReceiver::get(),
		);
		// Assert at least DefaultBridgeHubEthereumBaseFee charged from the sender
		let free_balance_diff = free_balance_before - free_balance_after;
		assert!(free_balance_diff > DefaultBridgeHubEthereumBaseFee::get());
	});

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		// Check that the transfer token back to Ethereum message was queue in the Ethereum
		// Outbound Queue
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued {..}) => {},
			]
		);
		let events = BridgeHubRococo::events();
		// Check that the local fee was credited to the Snowbridge sovereign account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Minted { who, amount })
					if *who == TREASURY_ACCOUNT.into() && *amount == 16903333
			)),
			"Snowbridge sovereign takes local fee."
		);
		// Check that the remote fee was credited to the AssetHub sovereign account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Minted { who, amount })
					if *who == assethub_sovereign && *amount == 2680000000000,
			)),
			"AssetHub sovereign takes remote fee."
		);
	});
}

#[test]
fn send_token_from_ethereum_to_asset_hub_fail_for_insufficient_fund() {
	// Insufficient fund
	BridgeHubRococo::fund_para_sovereign(AssetHubRococo::para_id().into(), 1_000);

	BridgeHubRococo::execute_with(|| {
		assert_err!(send_inbound_message(make_register_token_message()), Token(FundsUnavailable));
	});
}

#[test]
fn register_weth_token_in_asset_hub_fail_for_insufficient_fee() {
	BridgeHubRococo::fund_para_sovereign(AssetHubRococo::para_id().into(), INITIAL_FUND);

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type EthereumInboundQueue =
			<BridgeHubRococo as BridgeHubRococoPallet>::EthereumInboundQueue;
		type Converter = <bridge_hub_rococo_runtime::Runtime as snowbridge_pallet_inbound_queue::Config>::MessageConverter;

		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::RegisterToken {
				token: WETH.into(),
				// Insufficient fee which should trigger the trap
				fee: INSUFFICIENT_XCM_FEE,
			},
		});
		let (xcm, _) = Converter::convert(message_id, message).unwrap();
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubRococo::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubRococo::execute_with(|| {
		type RuntimeEvent = <AssetHubRococo as Chain>::RuntimeEvent;

		assert_expected_events!(
			AssetHubRococo,
			vec![
				RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success:false, .. }) => { },
			]
		);
	});
}

#[test]
fn transact_from_ethereum_to_penpal_success() {
	BridgeHubRococo::fund_para_sovereign(PenpalA::para_id().into(), INITIAL_FUND);

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type Runtime = <BridgeHubRococo as Chain>::Runtime;

		let agent_id = snowbridge_pallet_system::agent_id_of::<Runtime>(&Location::new(
			1,
			[Parachain(PenpalA::para_id().into())],
		))
		.unwrap();
		snowbridge_pallet_system::Agents::<Runtime>::insert(agent_id, ());
		let channel_id: ChannelId = PenpalA::para_id().into();
		snowbridge_pallet_system::Channels::<Runtime>::insert(
			channel_id,
			Channel { agent_id, para_id: PenpalA::para_id() },
		);

		let message_id: H256 = [1; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::Transact {
				sender: hex!("90A987B944Cb1dCcE5564e5FDeCD7a54D3de27Fe").into(),
				fee: XCM_FEE,
				weight_at_most: XCM_WEIGHT,
				origin_kind: OriginKind::SovereignAccount,
				payload: hex!("00071468656c6c6f").to_vec(),
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, PenpalA::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	PenpalA::execute_with(|| {
		type RuntimeEvent = <PenpalA as Chain>::RuntimeEvent;
		// Check that system event remarked on PenPal
		assert_expected_events!(
			PenpalA,
			vec![
				RuntimeEvent::System(frame_system::Event::Remarked { .. }) => {},
			]
		);
	});
}

#[test]
fn transact_from_ethereum_to_penpal_insufficient_weight() {
	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;

		let message_id: H256 = [1; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::Transact {
				sender: hex!("90A987B944Cb1dCcE5564e5FDeCD7a54D3de27Fe").into(),
				fee: XCM_FEE,
				weight_at_most: INSUFFICIENT_XCM_WEIGHT,
				origin_kind: OriginKind::SovereignAccount,
				payload: hex!("00071468656c6c6f").to_vec(),
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, PenpalA::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	PenpalA::execute_with(|| {
		type RuntimeEvent = <PenpalA as Chain>::RuntimeEvent;
		// Check xcm execution fails on PenPal
		assert_expected_events!(
			PenpalA,
			vec![
				RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success:false,.. }) => {},
			]
		);
	});
}

#[test]
fn transact_from_ethereum_to_penpal_insufficient_fee() {
	BridgeHubRococo::fund_para_sovereign(PenpalA::para_id().into(), INITIAL_FUND);

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type Runtime = <BridgeHubRococo as Chain>::Runtime;

		let agent_id = snowbridge_pallet_system::agent_id_of::<Runtime>(&Location::new(
			1,
			[Parachain(PenpalA::para_id().into())],
		))
		.unwrap();
		snowbridge_pallet_system::Agents::<Runtime>::insert(agent_id, ());
		let channel_id: ChannelId = PenpalA::para_id().into();
		snowbridge_pallet_system::Channels::<Runtime>::insert(
			channel_id,
			Channel { agent_id, para_id: PenpalA::para_id() },
		);

		let message_id: H256 = [1; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::Transact {
				sender: hex!("90A987B944Cb1dCcE5564e5FDeCD7a54D3de27Fe").into(),
				fee: INSUFFICIENT_XCM_FEE,
				weight_at_most: XCM_WEIGHT,
				origin_kind: OriginKind::SovereignAccount,
				payload: hex!("00071468656c6c6f").to_vec(),
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, PenpalA::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	PenpalA::execute_with(|| {
		type RuntimeEvent = <PenpalA as Chain>::RuntimeEvent;
		// Check xcm execution fails on PenPal
		assert_expected_events!(
			PenpalA,
			vec![
				RuntimeEvent::MessageQueue(pallet_message_queue::Event::ProcessingFailed{ .. }) => {},
			]
		);
	});
}
