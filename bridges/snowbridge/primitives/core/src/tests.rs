use crate::{ChannelId, ParaId, TokenIdOf};
use hex_literal::hex;
use xcm::prelude::{
	GeneralIndex, GeneralKey, GlobalConsensus, Location, PalletInstance, Parachain, Rococo,
};
use xcm_executor::traits::ConvertLocation;

const EXPECT_CHANNEL_ID: [u8; 32] =
	hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539");

// The Solidity equivalent code is tested in Gateway.t.sol:testDeriveChannelID
#[test]
fn generate_channel_id() {
	let para_id: ParaId = 1000.into();
	let channel_id: ChannelId = para_id.into();
	assert_eq!(channel_id, EXPECT_CHANNEL_ID.into());
}

#[test]
fn test_describe_relay_token() {
	let asset_location: Location = Location::new(1, [GlobalConsensus(Rococo)]);
	let token_id = TokenIdOf::convert_location(&asset_location).unwrap();
	assert_eq!(
		token_id,
		hex!("fb3d635c7cb573d1b9e9bff4a64ab4f25190d29b6fd8db94c605a218a23fa9ad").into()
	);
}

#[test]
fn test_describe_primary_token_from_parachain() {
	let asset_location: Location = Location::new(1, [GlobalConsensus(Rococo), Parachain(2000)]);
	let token_id = TokenIdOf::convert_location(&asset_location).unwrap();
	assert_eq!(
		token_id,
		hex!("6ee1f706bc329f61dada163071e292d853bcfb6fd66c917f20aa2b975225b482").into()
	);
}

#[test]
fn test_describe_token_with_pallet_instance_prefix() {
	let asset_location: Location =
		Location::new(1, [GlobalConsensus(Rococo), Parachain(2000), PalletInstance(8)]);
	let token_id = TokenIdOf::convert_location(&asset_location).unwrap();
	assert_eq!(
		token_id,
		hex!("53e05099ca310413b4102daa37ad8ae6e0b1f3b65f014529df584cf0132529e1").into()
	);
}

#[test]
fn test_describe_token_with_general_index_prefix() {
	let asset_location: Location =
		Location::new(1, [GlobalConsensus(Rococo), Parachain(2000), GeneralIndex(1)]);
	let token_id = TokenIdOf::convert_location(&asset_location).unwrap();
	assert_eq!(
		token_id,
		hex!("9f08be45307f36434a2dbdbd6093bb3477cdbecf59cc05eebd6b7ffc9af53acc").into()
	);
}

#[test]
fn test_describe_token_with_general_key_prefix() {
	let asset_location: Location = Location::new(
		1,
		[GlobalConsensus(Rococo), Parachain(2000), GeneralKey { length: 32, data: [1; 32] }],
	);
	let token_id = TokenIdOf::convert_location(&asset_location).unwrap();
	assert_eq!(
		token_id,
		hex!("bafbfc63d136de7d2503ce3ee276e5de4353e90c377b2102fcd98a9b9c5eec22").into()
	);
}
