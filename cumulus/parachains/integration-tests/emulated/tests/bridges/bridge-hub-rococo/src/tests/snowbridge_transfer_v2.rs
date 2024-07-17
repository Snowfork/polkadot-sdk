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
use snowbridge_core::{inbound::InboundQueueFixture, outbound::OperatingMode};
use snowbridge_pallet_inbound_queue_fixtures::{
	register_token::make_register_token_message, send_token::make_send_token_message,
	send_token_to_penpal::make_send_token_to_penpal_message,
};
use snowbridge_pallet_system;
use snowbridge_router_primitives::inbound::{
	Command, ConvertMessage, Destination, GlobalConsensusEthereumConvertsFor, MessageV1,
	VersionedMessage,
};
use sp_core::H256;
use sp_runtime::{DispatchError::Token, TokenError::FundsUnavailable};
use testnet_parachains_constants::rococo::snowbridge::EthereumNetwork;

const INITIAL_FUND: u128 = 5_000_000_000 * ROCOCO_ED;
const CHAIN_ID: u64 = 11155111;
const TREASURY_ACCOUNT: [u8; 32] =
	hex!("6d6f646c70792f74727372790000000000000000000000000000000000000000");
const WETH: [u8; 20] = hex!("87d1f7fdfEe7f651FaBc8bFCB6E086C278b77A7d");
const ETHEREUM_DESTINATION_ADDRESS: [u8; 20] = hex!("44a57ee2f2FCcb85FDa2B0B18EBD0D8D2333700e");
const INSUFFICIENT_XCM_FEE: u128 = 1000;
const XCM_FEE: u128 = 4_000_000_000;
const ETHEREUM_EXECUTION_FEE: u128 = 2_750_872_500_000;

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

/// Tests the full cycle of token transfers:
/// - registering a token on AssetHub
/// - sending a token to AssetHub
/// - returning the token to Ethereum
#[test]
fn send_weth_asset_from_asset_hub_to_ethereum() {
	use asset_hub_rococo_runtime::xcm_config::bridging::to_ethereum::DefaultBridgeHubEthereumBaseFee;
	let assethub_location = BridgeHubRococo::sibling_location_of(AssetHubRococo::para_id());
	let assethub_sovereign = BridgeHubRococo::sovereign_account_id_of(assethub_location);

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
		let asset = Asset {
			id: AssetId(Location::new(
				2,
				[
					GlobalConsensus(Ethereum { chain_id: CHAIN_ID }),
					AccountKey20 { network: None, key: WETH },
				],
			)),
			fun: Fungible(WETH_AMOUNT),
		};

		let free_balance_before = <AssetHubRococo as AssetHubRococoPallet>::Balances::free_balance(
			AssetHubRococoReceiver::get(),
		);

		// Todo: Change fee asset to WETH and remove the exchange_rate config on BH
		let fee_asset: Asset = (AssetId::from(Location::parent()), ETHEREUM_EXECUTION_FEE).into();
		// Send the Weth back to Ethereum
		<AssetHubRococo as AssetHubRococoPallet>::SnowbridgeXcmHelper::transfer_to_ethereum(
			RuntimeOrigin::signed(AssetHubRococoReceiver::get()),
			ETHEREUM_DESTINATION_ADDRESS.into(),
			Box::new(VersionedAsset::V4(asset)),
			Box::new(VersionedAsset::V4(fee_asset)),
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
				RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued
{..}) => {}, 			]
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
	});
}
