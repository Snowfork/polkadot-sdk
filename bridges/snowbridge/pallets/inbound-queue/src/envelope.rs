// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use snowbridge_core::{inbound::Log, ChannelId};

use sp_core::{RuntimeDebug, H160, H256};
use sp_std::{convert::TryFrom, prelude::*};

use alloy_primitives::{B256, hex};
use alloy_sol_types::{sol, SolEvent};
use snowbridge_router_primitives::inbound::{MessageV1, VersionedMessage};
use codec::{Encode, DecodeAll};
use snowbridge_beacon_primitives::verify_receipt_proof;
use snowbridge_router_primitives::inbound::Command::{RegisterToken, SendToken};
use snowbridge_router_primitives::inbound::VersionedMessage::V1;
use snowbridge_core::inbound::Proof;

sol! {
	event OutboundMessageAccepted(bytes32 indexed channel_id, uint64 nonce, bytes32 indexed message_id, bytes payload);
}

/// An inbound message that has had its outer envelope decoded.
#[derive(Clone, RuntimeDebug)]
pub struct Envelope {
	/// The address of the outbound queue on Ethereum that emitted this message as an event log
	pub gateway: H160,
	/// The message Channel
	pub channel_id: ChannelId,
	/// A nonce for enforcing replay protection and ordering.
	pub nonce: u64,
	/// An id for tracing the message on its route (has no role in bridge consensus)
	pub message_id: H256,
	/// The inner payload generated from the source application.
	pub payload: Vec<u8>,
}

#[derive(Copy, Clone, RuntimeDebug)]
pub struct EnvelopeDecodeError;

impl TryFrom<&Log> for Envelope {
	type Error = EnvelopeDecodeError;

	fn try_from(log: &Log) -> Result<Self, Self::Error> {
		let topics: Vec<B256> = log.topics.iter().map(|x| B256::from_slice(x.as_ref())).collect();

		let event = OutboundMessageAccepted::decode_log(topics, &log.data, true)
			.map_err(|_| EnvelopeDecodeError)?;

		Ok(Self {
			gateway: log.address,
			channel_id: ChannelId::from(event.channel_id.as_ref()),
			nonce: event.nonce,
			message_id: H256::from(event.message_id.as_ref()),
			payload: event.payload,
		})
	}
}

#[test]
fn test_log() {
	use hex_literal::hex;
	let log = Log {
		address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
		topics: vec![
			hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
			hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
			hex!("c8eaf22f2cb07bac4679df0a660e7115ed87fcfd4e32ac269f6540265bbbd26f").into(),
		],
		data: hex!("00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000005f0001000000000000000187d1f7fdfee7f651fabc8bfcb6e086c278b77a7d008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48000064a7b3b6e00d000000000000000000e40b5402000000000000000000000000").into(),
	};
	let proof = &Proof {
		block_hash: hex!("d3c155f123c3cbff22f3d7869283e02179edea9ffa7a5e9a4d8414c2a6b8991f").into(),
		tx_index: 0,
		data: (vec![
			hex!("9f3340b57eddc1f86de30776db57faeca80269a3dd459031741988dec240ce34").to_vec(),
		], vec![
			hex!("f90451822080b9044b02f90447018301bcb9b9010000800000000000000000000020000000000000000000004000000000000000000400000000000000000000001000000010000000000000000000000008000000200000000000000001000008000000000000000000000000000000008000080000000000200000000000000000000000000100000000000000000011000000000000020200000000000000000000000000003000000040080008000000000000000000040044000021000000002000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200800000000000f9033cf89b9487d1f7fdfee7f651fabc8bfcb6e086c278b77a7df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000090a987b944cb1dcce5564e5fdecd7a54d3de27fea000000000000000000000000057a2d4ff0c3866d96556884bf09fecdd7ccd530ca00000000000000000000000000000000000000000000000000de0b6b3a7640000f9015d94eda338e4dc46038493b885327842fd3e301cab39f884a024c5d2de620c6e25186ae16f6919eba93b6e2c1a33857cc419d9f3a00d6967e9a000000000000000000000000090a987b944cb1dcce5564e5fdecd7a54d3de27fea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7da000000000000000000000000000000000000000000000000000000000000003e8b8c000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48f9013c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a0c8eaf22f2cb07bac4679df0a660e7115ed87fcfd4e32ac269f6540265bbbd26fb8c000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000005f0001000000000000000187d1f7fdfee7f651fabc8bfcb6e086c278b77a7d008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48000064a7b3b6e00d000000000000000000e40b5402000000000000000000000000").to_vec(),
		]),
	};

	let topics: Vec<B256> = log.topics.iter().map(|x| B256::from_slice(x.as_ref())).collect();

	let event = OutboundMessageAccepted::decode_log(topics, &log.data, true)
		.map_err(|_| EnvelopeDecodeError).unwrap();

	println!("event: {:?}", hex::encode(event.payload.clone()));

	match VersionedMessage::decode_all(&mut event.payload.as_ref()) {
		Ok(message) => {
			match message {
				V1(MessageV1 { chain_id, command }) => {
					println!("chain_id: {}", chain_id);
					let message_v1 = MessageV1 {
						chain_id: 1, // Example chain_id
						command, // Example command
					};

					let versioned_message = VersionedMessage::V1(message_v1.clone());
					let payload2 = versioned_message.encode();

					println!("payload2: {:?}", hex::encode(payload2));

					verify_receipt_proof(H256::default(),&proof.data.1);
				},
			}
		},
		Err(_) => panic!("cannot decode"),
	};


}
