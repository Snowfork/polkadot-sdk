use crate::{
	rococo_testnet::{AssetHub, BridgeHub},
	setup::{BOB, WETH_ASSET_ID},
};
use parachains_common::AccountId;
use snowbridge_core::ParaId;
use snowbridge_router_primitives::inbound::{Command, MessageV1, VersionedMessage};
use xcm_simulator::TestExt;

#[test]
fn register_from_bridge_hub_to_asset_hub() {
	let _ = env_logger::builder().is_test(true).try_init();
	BridgeHub::execute_with(|| {
		use bridge_hub_rococo_runtime::EthereumInboundQueue;
		let message = MessageV1 {
			chain_id: 15,
			command: Command::RegisterToken {
				gateway: Default::default(),
				token: Default::default(),
			},
		};
		let version_message = VersionedMessage::V1(message);
		EthereumInboundQueue::do_convert_and_send_xcm(version_message, ParaId::from(1000)).unwrap();
	});
	BridgeHub::execute_with(|| {});
	AssetHub::execute_with(|| {
		use asset_hub_rococo_runtime::Assets;
		assert_eq!(Assets::balance(WETH_ASSET_ID, &AccountId::from(BOB)), 0);
	})
}

#[test]
fn transfer_from_bridge_hub_to_asset_hub() {}

#[test]
fn transfer_from_asset_hub_to_bridge_hub() {}
