use super::GlobalConsensusEthereumConvertsFor;
use crate::inbound::CallIndex;
use frame_support::{assert_ok, parameter_types};
use hex_literal::hex;
use xcm::prelude::*;
use xcm_executor::traits::ConvertLocation;

const NETWORK: NetworkId = Ethereum { chain_id: 11155111 };

parameter_types! {
	pub EthereumNetwork: NetworkId = NETWORK;

	pub const CreateAssetCall: CallIndex = [1, 1];
	pub const CreateAssetExecutionFee: u128 = 123;
	pub const CreateAssetDeposit: u128 = 891;
	pub const SendTokenExecutionFee: u128 = 592;
}

#[test]
fn test_contract_location_with_network_converts_successfully() {
	let expected_account: [u8; 32] =
		hex!("ce796ae65569a670d0c1cc1ac12515a3ce21b5fbf729d63d7b289baad070139d");
	let contract_location = Location::new(2, [GlobalConsensus(NETWORK)]);

	let account =
		GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&contract_location)
			.unwrap();

	assert_eq!(account, expected_account);
}

#[test]
fn test_contract_location_with_incorrect_location_fails_convert() {
	let contract_location = Location::new(2, [GlobalConsensus(Polkadot), Parachain(1000)]);

	assert_eq!(
		GlobalConsensusEthereumConvertsFor::<[u8; 32]>::convert_location(&contract_location),
		None,
	);
}

#[test]
fn test_reanchor_all_assets() {
	let ethereum_context: InteriorLocation = [GlobalConsensus(Ethereum { chain_id: 1 })].into();
	let ethereum = Location::new(2, ethereum_context.clone());
	let ah_context: InteriorLocation = [GlobalConsensus(Polkadot), Parachain(1000)].into();
	let global_ah = Location::new(1, ah_context.clone());
	let bh_context: InteriorLocation = [GlobalConsensus(Polkadot), Parachain(1002)].into();
	let assets = vec![
		// DOT
		Location::new(1, []),
		// GLMR (Some Polkadot parachain currency)
		Location::new(1, [Parachain(2004)]),
		// AH asset
		Location::new(0, [PalletInstance(50), GeneralIndex(42)]),
		// KSM
		Location::new(2, [GlobalConsensus(Kusama)]),
		// KAR (Some Kusama parachain currency)
		Location::new(2, [GlobalConsensus(Kusama), Parachain(2000)]),
	];
	for asset in assets.iter() {
		// reanchor logic in pallet_xcm on AH
		let mut reanchored_asset = asset.clone();
		assert_ok!(reanchored_asset.reanchor(&ethereum, &ah_context));
		// reanchor back to original location in context of Ethereum
		let mut reanchored_asset_with_ethereum_context = reanchored_asset.clone();
		assert_ok!(reanchored_asset_with_ethereum_context.reanchor(&global_ah, &ethereum_context));
		assert_eq!(reanchored_asset_with_ethereum_context, asset.clone());
		// reanchor back to original location in context of BH
		let mut reanchored_asset_with_bh_context = reanchored_asset.clone();
		assert_ok!(reanchored_asset_with_bh_context.reanchor(&global_ah, &bh_context));
		assert_eq!(reanchored_asset_with_bh_context, asset.clone());
	}
}
