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
use asset_hub_westend_runtime::xcm_config::bridging::to_ethereum::DefaultBridgeHubEthereumBaseFee;
use bridge_hub_westend_runtime::EthereumInboundQueue;
use codec::{Decode, Encode};
use emulated_integration_tests_common::{
	PenpalBSiblingSovereignAccount, RESERVABLE_ASSET_ID, TELEPORTABLE_ASSET_ID,
};
use frame_support::pallet_prelude::TypeInfo;
use hex_literal::hex;
use rococo_westend_system_emulated_network::{
	asset_hub_westend_emulated_chain::genesis::AssetHubWestendAssetOwner,
	penpal_emulated_chain::penpal_runtime::xcm_config::RelayLocation,
};
use snowbridge_core::{outbound::OperatingMode, AssetRegistrarMetadata, TokenIdOf};
use snowbridge_router_primitives::inbound::{
	Command, ConvertMessage, Destination, GlobalConsensusEthereumConvertsFor, MessageV1,
	VersionedMessage,
};
use sp_core::H256;
use testnet_parachains_constants::westend::snowbridge::EthereumNetwork;
use xcm::v3::MultiLocation;
use xcm_executor::traits::ConvertLocation;

const INITIAL_FUND: u128 = 5_000_000_000_000;
pub const CHAIN_ID: u64 = 11155111;
pub const WETH: [u8; 20] = hex!("87d1f7fdfEe7f651FaBc8bFCB6E086C278b77A7d");
const ETHEREUM_DESTINATION_ADDRESS: [u8; 20] = hex!("44a57ee2f2FCcb85FDa2B0B18EBD0D8D2333700e");
const XCM_FEE: u128 = 100_000_000_000;
const WETH_AMOUNT: u128 = 1_000_000_000;

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum ControlCall {
	#[codec(index = 3)]
	CreateAgent,
	#[codec(index = 4)]
	CreateChannel { mode: OperatingMode },
	#[codec(index = 11)]
	ForceRegisterToken {
		location: Box<VersionedLocation>,
		asset: Box<VersionedLocation>,
		metadata: AssetRegistrarMetadata,
	},
}

#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum SnowbridgeControl {
	#[codec(index = 83)]
	Control(ControlCall),
}

/// Tests the registering of a token as an asset on AssetHub.
#[test]
fn register_weth_token_from_ethereum_to_asset_hub() {
	// Fund AssetHub sovereign account so that it can pay execution fees.
	BridgeHubWestend::fund_para_sovereign(AssetHubWestend::para_id().into(), INITIAL_FUND);

	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		type Converter = <bridge_hub_westend_runtime::Runtime as snowbridge_pallet_inbound_queue::Config>::MessageConverter;

		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::RegisterToken { token: WETH.into(), fee: XCM_FEE },
		});
		let (xcm, _) = Converter::convert(message_id, message).unwrap();
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		assert_expected_events!(
			AssetHubWestend,
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
	let asset_hub_sovereign = BridgeHubWestend::sovereign_account_id_of(Location::new(
		1,
		[Parachain(AssetHubWestend::para_id().into())],
	));
	// Fund AssetHub sovereign account so it can pay execution fees for the asset transfer
	BridgeHubWestend::fund_accounts(vec![(asset_hub_sovereign.clone(), INITIAL_FUND)]);

	// Fund ethereum sovereign on AssetHub
	AssetHubWestend::fund_accounts(vec![(AssetHubWestendReceiver::get(), INITIAL_FUND)]);

	let weth_asset_location: Location =
		(Parent, Parent, EthereumNetwork::get(), AccountKey20 { network: None, key: WETH }).into();

	AssetHubWestend::execute_with(|| {
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::ForeignAssets::force_create(
			RuntimeOrigin::root(),
			weth_asset_location.clone().try_into().unwrap(),
			asset_hub_sovereign.into(),
			false,
			1,
		));

		assert!(<AssetHubWestend as AssetHubWestendPallet>::ForeignAssets::asset_exists(
			weth_asset_location.clone().try_into().unwrap(),
		));
	});

	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		type Converter = <bridge_hub_westend_runtime::Runtime as snowbridge_pallet_inbound_queue::Config>::MessageConverter;

		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendToken {
				token: WETH.into(),
				destination: Destination::AccountId32 { id: AssetHubWestendReceiver::get().into() },
				amount: WETH_AMOUNT,
				fee: XCM_FEE,
			},
		});
		let (xcm, _) = Converter::convert(message_id, message).unwrap();
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		// Check that the message was sent
		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		// Check that the token was received and issued as a foreign asset on AssetHub
		assert_expected_events!(
			AssetHubWestend,
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
	let assethub_location = BridgeHubWestend::sibling_location_of(AssetHubWestend::para_id());
	let assethub_sovereign = BridgeHubWestend::sovereign_account_id_of(assethub_location);
	let weth_asset_location: Location =
		(Parent, Parent, EthereumNetwork::get(), AccountKey20 { network: None, key: WETH }).into();

	AssetHubWestend::force_default_xcm_version(Some(XCM_VERSION));
	BridgeHubWestend::force_default_xcm_version(Some(XCM_VERSION));
	AssetHubWestend::force_xcm_version(
		Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]),
		XCM_VERSION,
	);

	BridgeHubWestend::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);

	AssetHubWestend::execute_with(|| {
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::ForeignAssets::force_create(
			RuntimeOrigin::root(),
			weth_asset_location.clone().try_into().unwrap(),
			assethub_sovereign.clone().into(),
			false,
			1,
		));

		assert!(<AssetHubWestend as AssetHubWestendPallet>::ForeignAssets::asset_exists(
			weth_asset_location.clone().try_into().unwrap(),
		));
	});

	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;
		type Converter = <bridge_hub_westend_runtime::Runtime as
	snowbridge_pallet_inbound_queue::Config>::MessageConverter;

		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendToken {
				token: WETH.into(),
				destination: Destination::AccountId32 { id: AssetHubWestendReceiver::get().into() },
				amount: WETH_AMOUNT,
				fee: XCM_FEE,
			},
		});
		let (xcm, _) = Converter::convert(message_id, message).unwrap();
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		// Check that the send token message was sent using xcm
		assert_expected_events!(
			BridgeHubWestend,
			vec![
	         RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) =>{},]
		);
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;

		// Check that AssetHub has issued the foreign asset
		assert_expected_events!(
			AssetHubWestend,
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

		let free_balance_before =
			<AssetHubWestend as AssetHubWestendPallet>::Balances::free_balance(
				AssetHubWestendReceiver::get(),
			);
		// Send the Weth back to Ethereum
		<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::limited_reserve_transfer_assets(
			RuntimeOrigin::signed(AssetHubWestendReceiver::get()),
			Box::new(destination),
			Box::new(beneficiary),
			Box::new(multi_assets),
			0,
			Unlimited,
		)
		.unwrap();
		let free_balance_after = <AssetHubWestend as AssetHubWestendPallet>::Balances::free_balance(
			AssetHubWestendReceiver::get(),
		);
		// Assert at least DefaultBridgeHubEthereumBaseFee charged from the sender
		let free_balance_diff = free_balance_before - free_balance_after;
		assert!(free_balance_diff > DefaultBridgeHubEthereumBaseFee::get());
	});

	BridgeHubWestend::execute_with(|| {
		use bridge_hub_westend_runtime::xcm_config::TreasuryAccount;
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;
		// Check that the transfer token back to Ethereum message was queue in the Ethereum
		// Outbound Queue
		assert_expected_events!(
			BridgeHubWestend,
			vec![

	RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued
	{..}) => {},             ]
		);
		let events = BridgeHubWestend::events();
		// Check that the local fee was credited to the Snowbridge sovereign account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Minted { who, amount })
					if *who == TreasuryAccount::get().into() && *amount == 5071000000
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
fn register_relay_token() {
	BridgeHubWestend::execute_with(|| {
		type RuntimeOrigin = <BridgeHubWestend as Chain>::RuntimeOrigin;
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		assert_ok!(<BridgeHubWestend as BridgeHubWestendPallet>::Balances::force_set_balance(
			RuntimeOrigin::root(),
			sp_runtime::MultiAddress::Id(BridgeHubWestendSender::get()),
			INITIAL_FUND * 10,
		));

		assert_ok!(<BridgeHubWestend as BridgeHubWestendPallet>::EthereumSystem::register_token(
			RuntimeOrigin::signed(BridgeHubWestendSender::get()),
			Box::new(VersionedLocation::V4(Location::parent())),
			AssetRegistrarMetadata {
				name: "wnd".as_bytes().to_vec(),
				symbol: "wnd".as_bytes().to_vec(),
				decimals: 12,
			},
		));
		// Check that a message was sent to Ethereum to create the agent
		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::EthereumSystem(snowbridge_pallet_system::Event::RegisterToken {
					..
				}) => {},
			]
		);
	});
}

#[test]
fn send_relay_token_to_ethereum() {
	let assethub_sovereign = BridgeHubWestend::sovereign_account_id_of(
		BridgeHubWestend::sibling_location_of(AssetHubWestend::para_id()),
	);
	BridgeHubWestend::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);

	AssetHubWestend::force_xcm_version(
		Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]),
		XCM_VERSION,
	);

	let asset_id: Location = Location::parent();
	let token_id = TokenIdOf::convert_location(&asset_id).unwrap();

	// create token
	BridgeHubWestend::execute_with(|| {
		type Runtime = <BridgeHubWestend as Chain>::Runtime;

		snowbridge_pallet_system::Tokens::<Runtime>::insert(token_id, asset_id.clone());
	});

	const TOKEN_AMOUNT: u128 = 100_000_000_000;

	// Send relay token to Ethereum
	AssetHubWestend::execute_with(|| {
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		let assets = vec![Asset { id: AssetId(Location::parent()), fun: Fungible(TOKEN_AMOUNT) }];
		let multi_assets = VersionedAssets::V4(Assets::from(assets));

		let destination = VersionedLocation::V4(Location::new(
			2,
			[GlobalConsensus(Ethereum { chain_id: CHAIN_ID })],
		));

		let beneficiary = VersionedLocation::V4(Location::new(
			0,
			[AccountKey20 { network: None, key: ETHEREUM_DESTINATION_ADDRESS.into() }],
		));

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::limited_reserve_transfer_assets(
			RuntimeOrigin::signed(AssetHubWestendSender::get()),
			Box::new(destination),
			Box::new(beneficiary),
			Box::new(multi_assets),
			0,
			Unlimited,
		));

		let events = AssetHubWestend::events();
		// Check that the native asset transferred to some reserved account(sovereign of Ethereum)
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Transfer { amount, ..})
					if *amount == TOKEN_AMOUNT,
			)),
			"native token reserved to Ethereum sovereign account."
		);
	});

	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;
		// Check that the transfer token back to Ethereum message was queue in the Ethereum
		// Outbound Queue
		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued
	{..}) => {}, 		]
		);
	});
}

#[test]
fn send_relay_token_from_ethereum() {
	let asset_id: Location = Location::parent();
	let token_id = TokenIdOf::convert_location(&asset_id).unwrap();

	const TOKEN_AMOUNT: u128 = 100_000_000_000;

	let ethereum_sovereign: AccountId =
		GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&Location::new(
			2,
			[GlobalConsensus(EthereumNetwork::get())],
		))
		.unwrap()
		.into();

	AssetHubWestend::fund_accounts(vec![(ethereum_sovereign.clone(), INITIAL_FUND)]);

	BridgeHubWestend::execute_with(|| {
		type Runtime = <BridgeHubWestend as Chain>::Runtime;

		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		// create token
		snowbridge_pallet_system::Tokens::<Runtime>::insert(token_id, asset_id.clone());

		// Send relay token back to AH
		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendNativeToken {
				token_id,
				destination: Destination::AccountId32 { id: AssetHubWestendReceiver::get().into() },
				amount: TOKEN_AMOUNT,
				fee: XCM_FEE,
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::Balances(pallet_balances::Event::Burned{..}) => {},]
		);

		let events = AssetHubWestend::events();

		// Check that the native token burnt from some reserved account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Burned { who, ..})
					if *who == ethereum_sovereign.clone(),
			)),
			"native token burnt from Ethereum sovereign account."
		);

		// Check that the token was minted to beneficiary
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Balances(pallet_balances::Event::Minted { who, amount })
					if *amount >= TOKEN_AMOUNT && *who == AssetHubWestendReceiver::get()
			)),
			"Token minted to beneficiary."
		);
	});
}

#[test]
fn send_ah_token_to_ethereum() {
	let assethub_sovereign = BridgeHubWestend::sovereign_account_id_of(
		BridgeHubWestend::sibling_location_of(AssetHubWestend::para_id()),
	);
	BridgeHubWestend::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);

	AssetHubWestend::force_xcm_version(
		Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]),
		XCM_VERSION,
	);

	let ethereum_sovereign: AccountId =
		GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&Location::new(
			2,
			[GlobalConsensus(EthereumNetwork::get())],
		))
		.unwrap()
		.into();

	AssetHubWestend::fund_accounts(vec![(ethereum_sovereign.clone(), INITIAL_FUND)]);

	const TOKEN_AMOUNT: u128 = 100_000_000_000;

	AssetHubWestend::mint_asset(
		<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendAssetOwner::get()),
		RESERVABLE_ASSET_ID,
		AssetHubWestendSender::get(),
		TOKEN_AMOUNT,
	);

	let asset_id: Location =
		[PalletInstance(ASSETS_PALLET_ID), GeneralIndex(RESERVABLE_ASSET_ID.into())].into();

	let asset_on_bh = Location::new(1, [Junction::Parachain(AssetHubWestend::para_id().into())])
		.appended_with(asset_id.clone().interior)
		.unwrap();

	let token_id = TokenIdOf::convert_location(&asset_on_bh).unwrap();

	// create token
	BridgeHubWestend::execute_with(|| {
		type Runtime = <BridgeHubWestend as Chain>::Runtime;

		snowbridge_pallet_system::Tokens::<Runtime>::insert(token_id, asset_on_bh.clone());
	});

	// Send relay token to Ethereum
	AssetHubWestend::execute_with(|| {
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		let assets =
			vec![Asset { id: AssetId(asset_id.clone()), fun: Fungible(TOKEN_AMOUNT / 10) }];
		let multi_assets = VersionedAssets::V4(Assets::from(assets));

		let destination = VersionedLocation::V4(Location::new(
			2,
			[GlobalConsensus(Ethereum { chain_id: CHAIN_ID })],
		));

		let beneficiary = VersionedLocation::V4(Location::new(
			0,
			[AccountKey20 { network: None, key: ETHEREUM_DESTINATION_ADDRESS.into() }],
		));

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::limited_reserve_transfer_assets(
			RuntimeOrigin::signed(AssetHubWestendSender::get()),
			Box::new(destination),
			Box::new(beneficiary),
			Box::new(multi_assets),
			0,
			Unlimited,
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::Assets(pallet_assets::Event::Transferred{..}) => {}, 		]
		);

		let events = AssetHubWestend::events();
		// Check that the native asset transferred to some reserved account(sovereign of Ethereum)
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Assets(pallet_assets::Event::Transferred { asset_id, to, ..})
					if *asset_id == RESERVABLE_ASSET_ID && *to == ethereum_sovereign.clone()
			)),
			"native token reserved to Ethereum sovereign account."
		);
	});

	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;
		// Check that the transfer token back to Ethereum message was queue in the Ethereum
		// Outbound Queue
		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued{..}) => {}, 		]
		);
	});
}

#[test]
fn send_penpal_token_from_ah_to_ethereum() {
	let assethub_location = BridgeHubWestend::sibling_location_of(AssetHubWestend::para_id());
	let assethub_sovereign = BridgeHubWestend::sovereign_account_id_of(assethub_location);

	AssetHubWestend::force_xcm_version(
		Location::new(2, [GlobalConsensus(Ethereum { chain_id: CHAIN_ID })]),
		XCM_VERSION,
	);

	BridgeHubWestend::fund_accounts(vec![(assethub_sovereign.clone(), INITIAL_FUND)]);

	let penpal_asset_location_on_ah =
		Location::new(1, [Junction::Parachain(PenpalB::para_id().into())])
			.appended_with(PenpalLocalTeleportableToAssetHub::get())
			.unwrap();
	let v3_location: MultiLocation = penpal_asset_location_on_ah.clone().try_into().unwrap();
	const TOKEN_AMOUNT: u128 = 100_000_000_000;

	AssetHubWestend::mint_foreign_asset(
		<AssetHubWestend as Chain>::RuntimeOrigin::signed(PenpalBSiblingSovereignAccount::get()),
		v3_location,
		AssetHubWestendSender::get(),
		TOKEN_AMOUNT,
	);

	let token_id = TokenIdOf::convert_location(&penpal_asset_location_on_ah).unwrap();

	// create token
	BridgeHubWestend::execute_with(|| {
		type Runtime = <BridgeHubWestend as Chain>::Runtime;

		snowbridge_pallet_system::Tokens::<Runtime>::insert(
			token_id,
			penpal_asset_location_on_ah.clone(),
		);
	});

	// Send token to Ethereum
	AssetHubWestend::execute_with(|| {
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		let assets = vec![Asset {
			id: penpal_asset_location_on_ah.clone().into(),
			fun: Fungible(TOKEN_AMOUNT / 10),
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

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::limited_reserve_transfer_assets(
			RuntimeOrigin::signed(AssetHubWestendSender::get()),
			Box::new(destination),
			Box::new(beneficiary),
			Box::new(multi_assets),
			0,
			Unlimited,
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Transferred{..}) => {},]
		);

		let ethereum_sovereign: AccountId =
			GlobalConsensusEthereumConvertsFor::<AccountId>::convert_location(
				&(Parent, Parent, EthereumNetwork::get()).into(),
			)
			.unwrap();

		let events = AssetHubWestend::events();
		// Check that the native asset transferred to some reserved account(sovereign of Ethereum)
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Transferred { amount, to, ..})
					if *amount == TOKEN_AMOUNT/10 && *to == ethereum_sovereign
			)),
			"native token reserved to Ethereum sovereign account."
		);
	});
	//
	BridgeHubWestend::execute_with(|| {
		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;
		// Check that the transfer token back to Ethereum message was queue in the Ethereum
		// Outbound Queue
		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageQueued{..}) => {}, 		]
		);
	});
}

#[test]
fn send_penpal_token_from_ethereum_to_ah() {
	let penpal_asset_location_on_ah =
		Location::new(1, [Parachain(PenpalB::para_id().into()).into()])
			.appended_with(PenpalLocalTeleportableToAssetHub::get())
			.unwrap();

	let v3_location: MultiLocation = penpal_asset_location_on_ah.clone().try_into().unwrap();

	let token_id = TokenIdOf::convert_location(&penpal_asset_location_on_ah).unwrap();

	const TOKEN_AMOUNT: u128 = 100_000_000_000;

	let ethereum_sovereign: AccountId =
		GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&Location::new(
			2,
			[GlobalConsensus(EthereumNetwork::get())],
		))
		.unwrap()
		.into();

	AssetHubWestend::fund_accounts(vec![(ethereum_sovereign.clone(), INITIAL_FUND)]);

	AssetHubWestend::mint_foreign_asset(
		<AssetHubWestend as Chain>::RuntimeOrigin::signed(PenpalBSiblingSovereignAccount::get()),
		v3_location,
		ethereum_sovereign.clone(),
		TOKEN_AMOUNT,
	);

	BridgeHubWestend::execute_with(|| {
		type Runtime = <BridgeHubWestend as Chain>::Runtime;

		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		// create token
		snowbridge_pallet_system::Tokens::<Runtime>::insert(
			token_id,
			penpal_asset_location_on_ah.clone(),
		);

		// Send token back to AH
		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendNativeToken {
				token_id,
				destination: Destination::AccountId32 { id: AssetHubWestendReceiver::get().into() },
				amount: TOKEN_AMOUNT,
				fee: XCM_FEE,
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		// Check that token burnt from some reserved account
		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Burned { .. }) => {},
			]
		);

		let events = AssetHubWestend::events();

		// Check that token issued to destination account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Issued { amount, owner, ..})
					if *amount == TOKEN_AMOUNT && *owner == AssetHubWestendReceiver::get()
			)),
			"Token minted to beneficiary."
		);
	});
}

#[test]
fn send_penpal_token_from_ethereum_to_penpal() {
	let penpal_asset_location_on_ah =
		Location::new(1, [Parachain(PenpalB::para_id().into()).into()])
			.appended_with(PenpalLocalTeleportableToAssetHub::get())
			.unwrap();

	let v3_location: MultiLocation = penpal_asset_location_on_ah.clone().try_into().unwrap();

	let token_id = TokenIdOf::convert_location(&penpal_asset_location_on_ah).unwrap();

	const TOKEN_AMOUNT: u128 = 100_000_000_000;

	let ethereum_sovereign: AccountId =
		GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&Location::new(
			2,
			[GlobalConsensus(EthereumNetwork::get())],
		))
		.unwrap()
		.into();

	AssetHubWestend::fund_accounts(vec![(ethereum_sovereign.clone(), INITIAL_FUND)]);

	AssetHubWestend::mint_foreign_asset(
		<AssetHubWestend as Chain>::RuntimeOrigin::signed(PenpalBSiblingSovereignAccount::get()),
		v3_location.clone(),
		ethereum_sovereign.clone(),
		TOKEN_AMOUNT,
	);

	let ah_sovereign =
		PenpalB::sovereign_account_id_of(PenpalB::sibling_location_of(AssetHubWestend::para_id()));
	PenpalB::fund_accounts(vec![(ah_sovereign.clone(), INITIAL_FUND)]);
	PenpalB::mint_foreign_asset(
		<PenpalB as Chain>::RuntimeOrigin::signed(PenpalAssetOwner::get()),
		RelayLocation::get(),
		ah_sovereign.clone(),
		INITIAL_FUND,
	);
	PenpalB::mint_asset(
		<PenpalB as Chain>::RuntimeOrigin::signed(PenpalAssetOwner::get()),
		TELEPORTABLE_ASSET_ID,
		ah_sovereign.clone(),
		TOKEN_AMOUNT,
	);

	BridgeHubWestend::execute_with(|| {
		type Runtime = <BridgeHubWestend as Chain>::Runtime;

		type RuntimeEvent = <BridgeHubWestend as Chain>::RuntimeEvent;

		// create token
		snowbridge_pallet_system::Tokens::<Runtime>::insert(
			token_id,
			penpal_asset_location_on_ah.clone(),
		);

		// Send token back to AH
		let message_id: H256 = [0; 32].into();
		let message = VersionedMessage::V1(MessageV1 {
			chain_id: CHAIN_ID,
			command: Command::SendNativeToken {
				token_id,
				destination: Destination::ForeignAccountId32 {
					para_id: PenpalB::para_id().into(),
					id: PenpalBReceiver::get().into(),
					fee: XCM_FEE,
				},
				amount: TOKEN_AMOUNT,
				fee: XCM_FEE,
			},
		});
		// Convert the message to XCM
		let (xcm, _) = EthereumInboundQueue::do_convert(message_id, message).unwrap();
		// Send the XCM
		let _ = EthereumInboundQueue::send_xcm(xcm, AssetHubWestend::para_id().into()).unwrap();

		assert_expected_events!(
			BridgeHubWestend,
			vec![
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }) => {},
			]
		);
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		// Check that token burnt from some reserved account
		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Burned { .. }) => {},
			]
		);
	});

	PenpalB::execute_with(|| {
		type RuntimeEvent = <PenpalB as Chain>::RuntimeEvent;

		// Check that token issued to beneficial
		assert_expected_events!(
			PenpalB,
			vec![
				RuntimeEvent::Assets(pallet_assets::Event::Issued { .. }) => {},
			]
		);

		let events = PenpalB::events();

		// Check that token issued to destination account
		assert!(
			events.iter().any(|event| matches!(
				event,
				RuntimeEvent::Assets(pallet_assets::Event::Issued { amount, owner, ..})
					if *amount == TOKEN_AMOUNT && *owner == PenpalBReceiver::get()
			)),
			"Token minted to beneficiary."
		);
	})
}
