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
fn test_reanchor_pna_on_ah() {
	let asset_id: Location =
		Location { parents: 0, interior: [PalletInstance(50), GeneralIndex(2)].into() };

	let asset: Asset = (asset_id, 1).into();
	let context: InteriorLocation = [GlobalConsensus(Westend), Parachain(1000)].into();
	let dest = Location::new(2, [GlobalConsensus(Ethereum { chain_id: 1 })]);
	let mut reanchored_assets = asset.clone();
	assert_ok!(reanchored_assets.reanchor(&dest, &context));
	assert_eq!(
		reanchored_assets.id.0,
		Location {
			parents: 1,
			interior: [
				GlobalConsensus(Westend),
				Parachain(1000),
				PalletInstance(50),
				GeneralIndex(2)
			]
			.into()
		}
	);
}

#[test]
fn test_reanchor_pna_on_bh() {
	let asset_id: Location = Location {
		parents: 1,
		interior: [GlobalConsensus(Westend), Parachain(1000), PalletInstance(50), GeneralIndex(2)]
			.into(),
	};
	let asset: Asset = (asset_id, 1).into();
	let context: InteriorLocation = [GlobalConsensus(Westend), Parachain(1013)].into();
	let dest = Location::new(1, [GlobalConsensus(Westend), Parachain(1000)]);
	let mut reanchored_assets = asset.clone();
	assert_ok!(reanchored_assets.reanchor(&dest, &context));
	assert_eq!(
		reanchored_assets.id.0,
		Location { parents: 0, interior: [PalletInstance(50), GeneralIndex(2)].into() }
	);
}
