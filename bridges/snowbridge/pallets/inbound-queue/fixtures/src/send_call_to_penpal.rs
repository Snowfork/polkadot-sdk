// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
// Generated, do not edit!
// See ethereum client README.md for instructions to generate

use hex_literal::hex;
use snowbridge_beacon_primitives::{types::deneb, ExecutionProof, VersionedExecutionPayloadHeader};
use snowbridge_core::inbound::{InboundQueueFixture, Log, Message, Proof};
use sp_std::vec;

pub fn make_send_call_to_penpal_message() -> InboundQueueFixture {
	InboundQueueFixture {
        message: Message {
            event_log: 	Log {
                address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
                topics: vec![
                    hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
                    hex!("a69fbbae90bb6096d59b1930bbcfc8a3ef23959d226b1861deb7ad8fb06c6fa3").into(),
                    hex!("c9ad11f6e2d2b770f52e7cbe22ba779eb8e5e000e0db4e5032488898bd3529ec").into(),
                ],
                data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000003e00a736aa00000000000290a987b944cb1dcce5564e5fdecd7a54d3de27fe0100902f5009000000000000000000000002688909017d2000071468656c6c6f0000").into(),
            },
            proof: Proof {
                receipt_proof: (vec![
                    hex!("6bba8f0093461e22cd265bc0de44bf7a596db58ecbf44432e6de8b3490deb077").to_vec(),
                ], vec![
                    hex!("f90234822080b9022e02f9022a0183016e01b9010000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000200000000000000000020000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000010000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002004000000000000000000000200000000000000f9011ff9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0a69fbbae90bb6096d59b1930bbcfc8a3ef23959d226b1861deb7ad8fb06c6fa3a0c9ad11f6e2d2b770f52e7cbe22ba779eb8e5e000e0db4e5032488898bd3529ecb8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000003e00a736aa00000000000290a987b944cb1dcce5564e5fdecd7a54d3de27fe0100902f5009000000000000000000000002688909017d2000071468656c6c6f0000").to_vec(),
                ]),
                execution_proof: ExecutionProof {
                    header: Default::default(),
                    ancestry_proof: None,
                    execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader{
                        parent_hash: Default::default(),
                        fee_recipient: Default::default(),
                        state_root: Default::default(),
                        receipts_root: Default::default(),
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
                },
            },
        },
        finalized_header: Default::default(),
        block_roots_root: Default::default(),
    }
}
