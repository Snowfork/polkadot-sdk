// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

use snowbridge_beacon_primitives::{
	types::deneb, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
};
use snowbridge_core::inbound::Message;
use sp_core::{RuntimeDebug, H256};
use sp_std::vec;

pub mod register_token;
pub mod register_token_with_insufficient_fee;
pub mod send_token;
pub mod send_token_to_penpal;

#[derive(Clone, RuntimeDebug)]
pub struct InboundQueueFixture {
	pub message: Message,
}

pub fn mock_execution_proof(receipts_root: H256) -> ExecutionProof {
	ExecutionProof {
		header: BeaconHeader::default(),
		ancestry_proof: None,
		execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
			parent_hash: Default::default(),
			fee_recipient: Default::default(),
			state_root: Default::default(),
			receipts_root,
			logs_bloom: vec![],
			prev_randao: Default::default(),
			block_number: 0,
			gas_limit: 0,
			gas_used: 0,
			timestamp: 0,
			extra_data: vec![],
			base_fee_per_gas: Default::default(),
			block_hash: Default::default(),
			transactions_root: Default::default(),
			withdrawals_root: Default::default(),
			blob_gas_used: 0,
			excess_blob_gas: 0,
		}),
		execution_branch: vec![],
	}
}
