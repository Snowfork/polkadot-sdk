// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
//! # Location
//!
//! Location helpers for dealing with Tokens and Agents

pub use polkadot_parachain_primitives::primitives::{
	Id as ParaId, IsSystem, Sibling as SiblingParaId,
};
pub use sp_core::U256;

use codec::Encode;
use sp_core::H256;
use sp_std::prelude::*;
use xcm::prelude::{
	AccountId32, AccountKey20, GeneralIndex, GeneralKey, GlobalConsensus, Location, PalletInstance,
};
use xcm_builder::{DescribeAllTerminal, DescribeFamily, DescribeLocation, HashedDescription};

pub type AgentId = H256;

/// Creates an AgentId from a Location. An AgentId is a unique mapping to a Agent contract on
/// Ethereum which acts as the sovereign account for the Location.
pub type AgentIdOf =
	HashedDescription<AgentId, (DescribeHere, DescribeFamily<DescribeAllTerminal>)>;

pub type TokenId = H256;

/// Convert a token location to a stable ID that can be used on the Ethereum side
pub type TokenIdOf = HashedDescription<
	TokenId,
	DescribeGlobalPrefix<(DescribeHere, DescribeFamily<DescribeTokenTerminal>)>,
>;

pub struct DescribeHere;
impl DescribeLocation for DescribeHere {
	fn describe_location(l: &Location) -> Option<Vec<u8>> {
		match l.unpack() {
			(0, []) => Some(Vec::<u8>::new().encode()),
			_ => None,
		}
	}
}
pub struct DescribeGlobalPrefix<DescribeInterior>(sp_std::marker::PhantomData<DescribeInterior>);
impl<Suffix: DescribeLocation> DescribeLocation for DescribeGlobalPrefix<Suffix> {
	fn describe_location(l: &Location) -> Option<Vec<u8>> {
		match (l.parent_count(), l.first_interior()) {
			(_, Some(GlobalConsensus(network))) => {
				let mut tail = l.clone().split_first_interior().0;
				tail.dec_parent();
				let interior = Suffix::describe_location(&tail)?;
				Some((b"GlobalConsensus", network, interior).encode())
			},
			_ => None,
		}
	}
}

pub struct DescribeTokenTerminal;
impl DescribeLocation for DescribeTokenTerminal {
	fn describe_location(l: &Location) -> Option<Vec<u8>> {
		match l.unpack().1 {
			[] => Some(Vec::<u8>::new().encode()),
			[GeneralIndex(index)] => Some((b"Index", *index).encode()),
			[GeneralKey { data, .. }] => Some((b"Key", *data).encode()),
			[AccountKey20 { key, .. }] => Some((b"AccountKey", *key).encode()),
			[AccountId32 { id, .. }] => Some((b"AccountId", *id).encode()),

			// Pallet
			[PalletInstance(instance)] => Some((b"Pallet", *instance).encode()),
			[PalletInstance(instance), GeneralIndex(index)] =>
				Some((b"Pallet", *instance, "Index", *index).encode()),
			[PalletInstance(instance), GeneralKey { data, .. }] =>
				Some((b"Pallet", *instance, b"Key", *data).encode()),

			[PalletInstance(instance), AccountKey20 { key, .. }] =>
				Some((b"Pallet", *instance, b"AccountKey", *key).encode()),
			[PalletInstance(instance), AccountId32 { id, .. }] =>
				Some((b"Pallet", *instance, b"AccountId", *id).encode()),

			// Reject all other locations
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::TokenIdOf;
	use xcm::prelude::{
		GeneralIndex, GeneralKey, GlobalConsensus, Junction::*, Location, NetworkId::*,
		PalletInstance, Parachain,
	};
	use xcm_executor::traits::ConvertLocation;

	#[test]
	fn test_token_of_id() {
		let token_locations = [
			// Relay Chain cases
			// Relay Chain relative to Ethereum
			Location::new(1, [GlobalConsensus(Westend)]),
			// Relay Chain relative to another polkadot chain.
			Location::new(2, [GlobalConsensus(Kusama)]),
			// Parachain cases
			// Parachain relative to Ethereum
			Location::new(1, [GlobalConsensus(Westend), Parachain(2000)]),
			// Parachain relative to another polkadot chain.
			Location::new(2, [GlobalConsensus(Kusama), Parachain(2000)]),
			// Parachain general index
			Location::new(1, [GlobalConsensus(Westend), Parachain(2000), GeneralIndex(1)]),
			// Parachain general key
			Location::new(
				1,
				[
					GlobalConsensus(Westend),
					Parachain(2000),
					GeneralKey { length: 32, data: [0; 32] },
				],
			),
			// Parachain account key 20
			Location::new(
				1,
				[
					GlobalConsensus(Westend),
					Parachain(2000),
					AccountKey20 { network: None, key: [0; 20] },
				],
			),
			// Parachain account id 32
			Location::new(
				1,
				[
					GlobalConsensus(Westend),
					Parachain(2000),
					AccountId32 { network: None, id: [0; 32] },
				],
			),
			// Parchain Pallet instance cases
			// Parachain pallet instance
			Location::new(1, [GlobalConsensus(Westend), Parachain(2000), PalletInstance(8)]),
			// Parachain Pallet general index
			Location::new(
				1,
				[GlobalConsensus(Westend), Parachain(2000), PalletInstance(8), GeneralIndex(1)],
			),
			// Parachain Pallet general key
			Location::new(
				1,
				[
					GlobalConsensus(Westend),
					Parachain(2000),
					PalletInstance(8),
					GeneralKey { length: 32, data: [0; 32] },
				],
			),
			// Parachain Pallet account key 20
			Location::new(
				1,
				[
					GlobalConsensus(Westend),
					Parachain(2000),
					PalletInstance(8),
					AccountKey20 { network: None, key: [0; 20] },
				],
			),
			// Parachain Pallet account id 32
			Location::new(
				1,
				[
					GlobalConsensus(Westend),
					Parachain(2000),
					PalletInstance(8),
					AccountId32 { network: None, id: [0; 32] },
				],
			),
			// Ethereum cases
			// Ethereum location relative to Polkadot
			Location::new(2, [GlobalConsensus(Ethereum { chain_id: 1 })]),
			// Ethereum location relative to Ethereum
			Location::new(1, [GlobalConsensus(Ethereum { chain_id: 2 })]),
			// Ethereum ERC20 location relative to Polkadot
			Location::new(
				2,
				[
					GlobalConsensus(Ethereum { chain_id: 1 }),
					AccountKey20 { network: None, key: [0; 20] },
				],
			),
		];

		for token in token_locations {
			assert!(
				TokenIdOf::convert_location(&token).is_some(),
				"Valid token = {token:?} yeilds no TokenId."
			);
		}

		let non_token_locations = [
			// Relative location for a token should fail.
			Location::new(1, []),
			// Relative location for a token should fail.
			Location::new(1, [Parachain(1000)]),
		];

		for token in non_token_locations {
			assert!(
				TokenIdOf::convert_location(&token).is_none(),
				"Invalid token = {token:?} yeilds a TokenId."
			);
		}
	}
}
