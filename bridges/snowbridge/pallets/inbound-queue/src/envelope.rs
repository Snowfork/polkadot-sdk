// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

use sp_core::{RuntimeDebug, H160, H256};
use sp_std::{convert::TryFrom, prelude::*};

use alloy_primitives::{B256, hex};
use alloy_sol_types::{sol, SolEvent};
use snowbridge_router_primitives::inbound::{MessageV1, VersionedMessage};
use codec::{Encode, DecodeAll};
use snowbridge_beacon_primitives::verify_receipt_proof;
use snowbridge_router_primitives::inbound::VersionedMessage::V1;
use snowbridge_beacon_primitives::{
	types::deneb, AncestryProof, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
};
use snowbridge_core::inbound::{InboundQueueFixture, Log, Message, Proof};
use snowbridge_core::ChannelId;
use snowbridge_core::inbound::VerificationError::InvalidProof;
use sp_core::U256;
use sp_std::vec;

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
	let log = Log{
		address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
		topics: vec![
			hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
			hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
			hex!("5f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0").into(),
		],
		data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e0001000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").into(),
	};
	let proof = &Proof{
		receipt_proof: (vec![
			//hex!("dccdfceea05036f7b61dcdabadc937945d31e68a8d3dfd4dc85684457988c284").to_vec(),
			hex!("4a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f").to_vec(),
		], vec![
			//hex!("f851a09c01dd6d2d8de951c45af23d3ad00829ce021c04d6c8acbe1612d456ee320d4980808080808080a04a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f8080808080808080").to_vec(),
			hex!("f9028c30b9028802f90284018301d205b9010000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010f90179f85894eda338e4dc46038493b885327842fd3e301cab39e1a0f78bb28d4b1d7da699e5c0bc2be29c2b04b5aab6aacf6298fe5304f9db9c6d7ea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7df9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a05f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0b8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e0001000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").to_vec(),
		]),
		execution_proof: ExecutionProof {
			header: BeaconHeader {
				slot: 393,
				proposer_index: 4,
				parent_root: hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef").into(),
				state_root: hex!("b62ac34a8cb82497be9542fe2114410c9f6021855b766015406101a1f3d86434").into(),
				body_root: hex!("04005fe231e11a5b7b1580cb73b177ae8b338bedd745497e6bb7122126a806db").into(),
			},
			ancestry_proof: Some(AncestryProof {
				header_branch: vec![
					hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef").into(),
					hex!("fa84cc88ca53a72181599ff4eb07d8b444bce023fe2347c3b4f51004c43439d3").into(),
					hex!("cadc8ae211c6f2221c9138e829249adf902419c78eb4727a150baa4d9a02cc9d").into(),
					hex!("33a89962df08a35c52bd7e1d887cd71fa7803e68787d05c714036f6edf75947c").into(),
					hex!("2c9760fce5c2829ef3f25595a703c21eb22d0186ce223295556ed5da663a82cf").into(),
					hex!("e1aa87654db79c8a0ecd6c89726bb662fcb1684badaef5cd5256f479e3c622e1").into(),
					hex!("aa70d5f314e4a1fbb9c362f3db79b21bf68b328887248651fbd29fc501d0ca97").into(),
					hex!("160b6c235b3a1ed4ef5f80b03ee1c76f7bf3f591c92fca9d8663e9221b9f9f0f").into(),
					hex!("f68d7dcd6a07a18e9de7b5d2aa1980eb962e11d7dcb584c96e81a7635c8d2535").into(),
					hex!("1d5f912dfd6697110dd1ecb5cb8e77952eef57d85deb373572572df62bb157fc").into(),
					hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b").into(),
					hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220").into(),
					hex!("b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f").into(),
				],
				finalized_block_root: hex!("751414cd97c0624f922b3e80285e9f776b08fa22fd5f87391f2ed7ef571a8d46").into(),
			}),
			execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
				parent_hash: hex!("8092290aa21b7751576440f77edd02a94058429ce50e63a92d620951fb25eda2").into(),
				fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
				state_root: hex!("96a83e9ddf745346fafcb0b03d57314623df669ed543c110662b21302a0fae8b").into(),
				receipts_root: hex!("62d13e9a073dc7cf609005b5531bb208c8686f18f7c8ae02d76232d83ae41a21").into(),
				logs_bloom: hex!("00000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000400000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000080000000000000000000000000000040004000000000000002002002000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000080000000000000000000000000000000000100000000000000000200000200000010").into(),
				prev_randao: hex!("62e309d4f5119d1f5c783abc20fc1a549efbab546d8d0b25ff1cfd58be524e67").into(),
				block_number: 393,
				gas_limit: 54492273,
				gas_used: 199644,
				timestamp: 1710552813,
				extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").into(),
				base_fee_per_gas: U256::from(7u64),
				block_hash: hex!("6a9810efb9581d30c1a5c9074f27c68ea779a8c1ae31c213241df16225f4e131").into(),
				transactions_root: hex!("2cfa6ed7327e8807c7973516c5c32a68ef2459e586e8067e113d081c3bd8c07d").into(),
				withdrawals_root: hex!("792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535").into(),
				blob_gas_used: 0,
				excess_blob_gas: 0,
			}),
			execution_branch: vec![
				hex!("a6833fa629f3286b6916c6e50b8bf089fc9126bee6f64d0413b4e59c1265834d").into(),
				hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
				hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
				hex!("d3af7c05c516726be7505239e0b9c7cb53d24abce6b91cdb3b3995f0164a75da").into(),
			],
		}
	};
	let finalized_header = BeaconHeader{
		slot: 864,
		proposer_index: 4,
		parent_root: hex!("614e7672f991ac268cd841055973f55e1e42228831a211adef207bb7329be614").into(),
		state_root: hex!("5fa8dfca3d760e4242ab46d529144627aa85348a19173b6e081172c701197a4a").into(),
		body_root: hex!("0f34c083b1803666bb1ac5e73fa71582731a2cf37d279ff0a3b0cad5a2ff371e").into(),
	};
	let block_roots_root: H256 = hex!("b9aab9c388c4e4fcd899b71f62c498fc73406e38e8eb14aa440e9affa06f2a10").into();

	let topics: Vec<B256> = log.topics.iter().map(|x| B256::from_slice(x.as_ref())).collect();

	let event = OutboundMessageAccepted::decode_log(topics, &log.data, true)
		.map_err(|_| EnvelopeDecodeError).unwrap();

	println!("event: {:?}", hex::encode(event.payload.clone()));

	match VersionedMessage::decode_all(&mut event.payload.as_ref()) {
		Ok(message) => {
			println!("message: {:?}", message);
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

					let result = verify_receipt_proof(proof.execution_proof.execution_header.receipts_root(),&proof.receipt_proof.1);

					match result {
						None => { panic!("no result") }
						Some(value) => {
							let res = value.unwrap();
						}
					}
				},
			}
		},
		Err(_) => panic!("cannot decode"),
	};
}
