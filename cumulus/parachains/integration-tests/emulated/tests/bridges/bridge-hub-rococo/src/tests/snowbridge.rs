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
use crate::*;
use bridge_hub_rococo_runtime::EthereumBeaconClient;
use codec::{Decode, Encode};
use emulated_integration_tests_common::xcm_emulator::ConvertLocation;
use frame_support::pallet_prelude::TypeInfo;
use hex_literal::hex;
use parachains_common::rococo::snowbridge::EthereumNetwork;
use rococo_westend_system_emulated_network::BridgeHubRococoParaSender as BridgeHubRococoSender;
use snowbridge_core::outbound::OperatingMode;
use snowbridge_pallet_inbound_queue::fixtures::make_create_message;
use snowbridge_pallet_system;
use snowbridge_router_primitives::inbound::{
	Command, Destination, GlobalConsensusEthereumConvertsFor, MessageV1, VersionedMessage,
};
use sp_core::H256;

const INITIAL_FUND: u128 = 5_000_000_000 * ROCOCO_ED;
const CHAIN_ID: u64 = 11155111;
const TREASURY_ACCOUNT: [u8; 32] =
	hex!("6d6f646c70792f74727372790000000000000000000000000000000000000000");
const WETH: [u8; 20] = hex!("87d1f7fdfEe7f651FaBc8bFCB6E086C278b77A7d");
const ETHEREUM_DESTINATION_ADDRESS: [u8; 20] = hex!("44a57ee2f2FCcb85FDa2B0B18EBD0D8D2333700e");
const XCM_FEE: u128 = 4_000_000_000;

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

/// Create an agent on Ethereum. An agent is a representation of an entity in the Polkadot
/// ecosystem (like a parachain) on Ethereum.
#[test]
fn create_agent() {
	let origin_para: u32 = 1001;
	// Fund the origin parachain sovereign account so that it can pay execution fees.
	BridgeHubRococo::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let destination = Rococo::child_location_of(BridgeHubRococo::para_id()).into();

	let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
	// Construct XCM to create an agent for para 1001
	let remote_xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		DescendOrigin(X1(Parachain(origin_para))),
		Transact {
			require_weight_at_most: 3000000000.into(),
			origin_kind: OriginKind::Xcm,
			call: create_agent_call.encode().into(),
		},
	]));

	// Rococo Global Consensus
	// Send XCM message from Relay Chain to Bridge Hub source Parachain
	Rococo::execute_with(|| {
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send(
			sudo_origin,
			bx!(destination),
			bx!(remote_xcm),
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
	let origin_para: u32 = 1001;
	// Fund AssetHub sovereign account so that it can pay execution fees.
	BridgeHubRococo::fund_para_sovereign(origin_para.into(), INITIAL_FUND);

	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let destination: VersionedMultiLocation =
		Rococo::child_location_of(BridgeHubRococo::para_id()).into();

	let create_agent_call = SnowbridgeControl::Control(ControlCall::CreateAgent {});
	// Construct XCM to create an agent for para 1001
	let create_agent_xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		DescendOrigin(X1(Parachain(origin_para))),
		Transact {
			require_weight_at_most: 3000000000.into(),
			origin_kind: OriginKind::Xcm,
			call: create_agent_call.encode().into(),
		},
	]));

	let create_channel_call =
		SnowbridgeControl::Control(ControlCall::CreateChannel { mode: OperatingMode::Normal });
	// Construct XCM to create a channel for para 1001
	let create_channel_xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		DescendOrigin(X1(Parachain(origin_para))),
		Transact {
			require_weight_at_most: 3000000000.into(),
			origin_kind: OriginKind::Xcm,
			call: create_channel_call.encode().into(),
		},
	]));

	// Rococo Global Consensus
	// Send XCM message from Relay Chain to Bridge Hub source Parachain
	Rococo::execute_with(|| {
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send(
			sudo_origin.clone(),
			bx!(destination.clone()),
			bx!(create_agent_xcm),
		));

		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send(
			sudo_origin,
			bx!(destination),
			bx!(create_channel_xcm),
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

	let message_id: H256 = [1; 32].into();

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type RuntimeOrigin = <BridgeHubRococo as Chain>::RuntimeOrigin;
		type EthereumInboundQueue =
			<BridgeHubRococo as BridgeHubRococoPallet>::EthereumInboundQueue;

		let create_message = make_create_message();

		EthereumBeaconClient::store_execution_header(
			create_message.message.proof.block_hash,
			create_message.execution_header,
			0,
			H256::default(),
		);

		EthereumInboundQueue::submit(
			RuntimeOrigin::signed(BridgeHubRococoSender::get()),
			create_message.message,
		)
		.unwrap();

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

/// Tests sending a token to a 3rd party parachain, called PenPal. The token reserve is
/// still located on AssetHub.
#[test]
fn send_token_from_ethereum_to_penpal() {
	let asset_hub_sovereign = BridgeHubRococo::sovereign_account_id_of(MultiLocation {
		parents: 1,
		interior: X1(Parachain(AssetHubRococo::para_id().into())),
	});
	// Fund AssetHub sovereign account so it can pay execution fees for the asset transfer
	BridgeHubRococo::fund_accounts(vec![(asset_hub_sovereign.clone(), INITIAL_FUND)]);

	// Fund PenPal sender and receiver
	PenpalA::fund_accounts(vec![
		(PenpalAReceiver::get(), INITIAL_FUND), // for receiving the sent asset on PenPal
		(PenpalASender::get(), INITIAL_FUND), // for creating the asset on PenPal
	]);

	// The Weth asset location, identified by the contract address on Ethereum
	let weth_asset_location: MultiLocation =
		(Parent, Parent, EthereumNetwork::get(), AccountKey20 { network: None, key: WETH }).into();
	// Converts the Weth asset location into an asset ID
	let weth_asset_id = weth_asset_location.into();

	let origin_location = (Parent, Parent, EthereumNetwork::get()).into();

	// Fund ethereum sovereign on AssetHub
	let ethereum_sovereign: AccountId =
		GlobalConsensusEthereumConvertsFor::<AccountId>::convert_location(&origin_location)
			.unwrap();
	AssetHubRococo::fund_accounts(vec![(ethereum_sovereign.clone(), INITIAL_FUND)]);

	// Create asset on AssetHub, since that is where the asset reserve is located
	AssetHubRococo::execute_with(|| {
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::ForeignAssets::create(
			pallet_xcm::Origin::Xcm(origin_location).into(),
			weth_asset_id,
			asset_hub_sovereign.clone().into(),
			1000,
		));

		assert!(<AssetHubRococo as AssetHubRococoPallet>::ForeignAssets::asset_exists(
			weth_asset_id
		));
	});

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

	let message_id: H256 = [1; 32].into();

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type EthereumInboundQueue =
			<BridgeHubRococo as BridgeHubRococoPallet>::EthereumInboundQueue;
		// Construct SendToken message
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendToken {
				token: WETH.into(),
				destination: Destination::ForeignAccountId32 {
					para_id: 2000,
					id: PenpalAReceiver::get().into(),
					fee: XCM_FEE,
				},
				amount: 1_000_000_000,
				fee: XCM_FEE,
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
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

/// Tests the registering of a token as an asset on AssetHub, and then subsequently sending
/// a token from Ethereum to AssetHub.
#[test]
fn send_token_from_ethereum_to_asset_hub() {
	BridgeHubRococo::fund_para_sovereign(AssetHubRococo::para_id().into(), INITIAL_FUND);

	// Fund ethereum sovereign on AssetHub
	AssetHubRococo::fund_accounts(vec![(AssetHubRococoReceiver::get(), INITIAL_FUND)]);

	let message_id: H256 = [1; 32].into();

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type EthereumInboundQueue =
			<BridgeHubRococo as BridgeHubRococoPallet>::EthereumInboundQueue;
		// Construct RegisterToken message
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::RegisterToken { token: WETH.into(), fee: XCM_FEE },
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubRococo::para_id().into()).unwrap();

		// Construct SendToken message
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendToken {
				token: WETH.into(),
				destination: Destination::AccountId32 { id: AssetHubRococoReceiver::get().into() },
				amount: 1_000_000_000,
				fee: XCM_FEE,
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubRococo::para_id().into()).unwrap();

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

/// Tests the full cycle of token transfers:
/// - registering a token on AssetHub
/// - sending a token to AssetHub
/// - returning the token to Ethereum
#[test]
fn send_weth_asset_from_asset_hub_to_ethereum() {
	use asset_hub_rococo_runtime::xcm_config::bridging::to_ethereum::DefaultBridgeHubEthereumBaseFee;
	let assethub_sovereign = BridgeHubRococo::sovereign_account_id_of(MultiLocation {
		parents: 1,
		interior: X1(Parachain(AssetHubRococo::para_id().into())),
	});

	AssetHubRococo::force_default_xcm_version(Some(XCM_VERSION));
	BridgeHubRococo::force_default_xcm_version(Some(XCM_VERSION));
	AssetHubRococo::force_xcm_version(
		MultiLocation {
			parents: 2,
			interior: X1(GlobalConsensus(Ethereum { chain_id: CHAIN_ID })),
		},
		XCM_VERSION,
	);

	BridgeHubRococo::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);
	AssetHubRococo::fund_accounts(vec![(AssetHubRococoReceiver::get(), INITIAL_FUND)]);

	const WETH_AMOUNT: u128 = 1_000_000_000;
	let message_id_register_token: H256 = [1; 32].into();
	let message_id_send_token: H256 = [2; 32].into();

	BridgeHubRococo::execute_with(|| {
		type RuntimeEvent = <BridgeHubRococo as Chain>::RuntimeEvent;
		type EthereumInboundQueue =
			<BridgeHubRococo as BridgeHubRococoPallet>::EthereumInboundQueue;

		// Register ERC-20 token on AssetHub
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::RegisterToken { token: WETH.into(), fee: XCM_FEE },
		});
		// Converts the versioned message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id_register_token, message).unwrap();
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubRococo::para_id().into()).unwrap();

		// Check that the register token message was sent using xcm
		assert_expected_events!(
			BridgeHubRococo,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);

		// Send ERC-20 token to AssetHub
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendToken {
				token: WETH.into(),
				destination: Destination::AccountId32 { id: AssetHubRococoReceiver::get().into() },
				amount: WETH_AMOUNT,
				fee: XCM_FEE,
			},
		});
		// Converts the versioned message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id_send_token, message).unwrap();
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubRococo::para_id().into()).unwrap();

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
		let assets = vec![MultiAsset {
			id: Concrete(MultiLocation {
				parents: 2,
				interior: X2(
					GlobalConsensus(Ethereum { chain_id: CHAIN_ID }),
					AccountKey20 { network: None, key: WETH },
				),
			}),
			fun: Fungible(WETH_AMOUNT),
		}];
		let multi_assets = VersionedMultiAssets::V3(MultiAssets::from(assets));

		let destination = VersionedMultiLocation::V3(MultiLocation {
			parents: 2,
			interior: X1(GlobalConsensus(Ethereum { chain_id: CHAIN_ID })),
		});

		let beneficiary = VersionedMultiLocation::V3(MultiLocation {
			parents: 0,
			interior: X1(AccountKey20 { network: None, key: ETHEREUM_DESTINATION_ADDRESS.into() }),
		});

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
				RuntimeEvent::Balances(pallet_balances::Event::Deposit{ who, amount })
					if *who == TREASURY_ACCOUNT.into() && *amount == 16903333
			)),
			"Snowbridge sovereign takes local fee."
		);
		// Check that the remote fee was credited to the AssetHub sovereign account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Deposit{ who, amount })
					if *who == assethub_sovereign && *amount == 2680000000000,
			)),
			"AssetHub sovereign takes remote fee."
		);
	});
}
