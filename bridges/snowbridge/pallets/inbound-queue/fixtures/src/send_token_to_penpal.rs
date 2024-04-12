// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
// Generated, do not edit!
// See ethereum client README.md for instructions to generate

use hex_literal::hex;
use snowbridge_beacon_primitives::{
	types::deneb, AncestryProof, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
};
use snowbridge_core::inbound::{InboundQueueFixture, Log, Message, Proof};
use sp_core::U256;
use sp_std::vec;

pub fn make_send_token_to_penpal_message() -> InboundQueueFixture {
	InboundQueueFixture {
        message: Message {
            event_log: 	Log {
                address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
                topics: vec![
                    hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
                    hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
                    hex!("be323bced46a1a49c8da2ab62ad5e974fd50f1dabaeed70b23ca5bcf14bfe4aa").into(),
                ],
                data: hex!("00000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300a736aa00000000000187d1f7fdfee7f651fabc8bfcb6e086c278b77a7d01d00700002848d4dce4a7387c0f38f932a82f1b4ed783633e12ff73f0416cc982b5d8293000902f50090000000000000000000000000064a7b3b6e00d000000000000000000e40b5402000000000000000000000000000000000000000000000000").into(),
            },
            proof: Proof {
                receipt_proof: (vec![
                    hex!("ef58c752b95a54c9fda2fb84321032e324388fe6979d5effabfc817f6bc5c404").to_vec(),
                    hex!("f2cb7deabd80a854ec1b71700bf72951c1746e8f1ca99d82fde52dcd3594a59c").to_vec(),
                ], vec![
                    hex!("f851a0f6344ee4424a0969377674a96525d555851fde52f0243e341f95c64a117b7a1f80808080808080a0f2cb7deabd80a854ec1b71700bf72951c1746e8f1ca99d82fde52dcd3594a59c8080808080808080").to_vec(),
                    hex!("f9046f30b9046b02f904670183018678b9010000800000000000008000000000000000000000000000004000000000000000000400000000004000000000001000000010000000000000000000001008000000000000000000000001000008000040000000000000000000000000008000080000000000200000000000000000000000000100000000000000000010000000000000020000000000000000000000000000003000000000080018000000000000000000040004000021000000002000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200820000000000f9035cf89b9487d1f7fdfee7f651fabc8bfcb6e086c278b77a7df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000090a987b944cb1dcce5564e5fdecd7a54d3de27fea000000000000000000000000057a2d4ff0c3866d96556884bf09fecdd7ccd530ca00000000000000000000000000000000000000000000000000de0b6b3a7640000f9015d94eda338e4dc46038493b885327842fd3e301cab39f884a024c5d2de620c6e25186ae16f6919eba93b6e2c1a33857cc419d9f3a00d6967e9a000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7da000000000000000000000000090a987b944cb1dcce5564e5fdecd7a54d3de27fea000000000000000000000000000000000000000000000000000000000000007d0b8c000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000202848d4dce4a7387c0f38f932a82f1b4ed783633e12ff73f0416cc982b5d82930f9015c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a0be323bced46a1a49c8da2ab62ad5e974fd50f1dabaeed70b23ca5bcf14bfe4aab8e000000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300a736aa00000000000187d1f7fdfee7f651fabc8bfcb6e086c278b77a7d01d00700002848d4dce4a7387c0f38f932a82f1b4ed783633e12ff73f0416cc982b5d8293000902f50090000000000000000000000000064a7b3b6e00d000000000000000000e40b5402000000000000000000000000000000000000000000000000").to_vec(),
                ]),
                execution_proof: ExecutionProof {
                    header: BeaconHeader {
                        slot: 656,
                        proposer_index: 4,
                        parent_root: hex!("4c8623cadda798eb37fc2532ffdbed5063a6918cdb852ed767ac031e16fc7454").into(),
                        state_root: hex!("5c2b7450d708d7285be567c114789ed13c8cf3acc18966f3dd7cd403b2e8846d").into(),
                        body_root: hex!("a62ad47ddac154d25cb11970d2c7e6bf6415aa715998c467ffb363b6611eef75").into(),
                    },
                        ancestry_proof: Some(AncestryProof {
                        header_branch: vec![
                            hex!("1d033b2148faccaf3f3374599436c89a09e5bffc83920e1bde5a7b6a39c97eb6").into(),
                            hex!("aa0eb5609ddf2cb220b409b99240964870df2645baf58add8c246ca75bd44ced").into(),
                            hex!("d68779c403a986fc2a948e9342d3e3cb55bed335669b00aa106c59b84bbdf616").into(),
                            hex!("e7524457f2b258409bb1966cf787a2d9730b254dd5fd8552e5b2f38c60e8bcf2").into(),
                            hex!("eef41eeedb32358c666e42d94341f9f28851157179bce25e57ad651ff4c4e874").into(),
                            hex!("c859788034c88ccb1a3897346801d18bcd6c03f13de86dcad3b1a874a0712dfb").into(),
                            hex!("fb04a72d4f852cb2ea66fce3cc33262c1275c5c4bbb0b4e9ff12c020f74dd219").into(),
                            hex!("f40ba6b04766b5a9fbc9aa3152a70515de6ddd6dea6a57f591a42dbeea64e41a").into(),
                            hex!("f13382a9ad115d0afa374b16c9fe0c884f2d0331f26e70f80212ee4e2020531c").into(),
                            hex!("7fc8d0ae882d8da2d2684c79469a66bb3fa32a12e6b8a804f59549ef40abf626").into(),
                            hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b").into(),
                            hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220").into(),
                            hex!("b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f").into(),
                        ],
                        finalized_block_root: hex!("996e49aa50ee8a10bdc68c6438af1941a3416cb42a514769e4276acabe06fb6d").into(),
                        }),
                    execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
                        parent_hash: hex!("6445865151b2b7d6ed86c528b4d4a891affaab9311ae9f49892bb1b5e72931fe").into(),
                        fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
                        state_root: hex!("77ff810321768b52853212284636d59753e7d338f6d7b1a36e0ef1a01648e089").into(),
                        receipts_root: hex!("ef58c752b95a54c9fda2fb84321032e324388fe6979d5effabfc817f6bc5c404").into(),
                        logs_bloom: hex!("00800000000000008000000000000000000000000000004000000000000000000400000000004000000000001000000010000000000000000000001008000000000000000000000001000008000040000000000000000000000000008000080000000000200000000000000000000000000100000000000000000010000000000000020000000000000000000000000000003000000000080018000000000000000000040004000021000000002000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200820000000000").into(),
                        prev_randao: hex!("8e8fae508a070280553a96b9a258348cd955bec4a35e573f56c58ae7b757eca2").into(),
                        block_number: 656,
                        gas_limit: 42144566,
                        gas_used: 146598,
                        timestamp: 1712921255,
                        extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").into(),
                        base_fee_per_gas: U256::from(7u64),
                        block_hash: hex!("372506e1624df810a9177861beac8c800ad3e699aeb11a38530272d5cf855f06").into(),
                        transactions_root: hex!("d0228932698005cd96a6d237270630146078da381a142c3414a9c12de4803ed1").into(),
                        withdrawals_root: hex!("792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535").into(),
                        blob_gas_used: 0,
                        excess_blob_gas: 0,
                    }),
                    execution_branch: vec![
                            hex!("1995d26e55d94c44dfba7dd6c2994b0626f85d31b42e6289f6079aac342542d0").into(),
                            hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
                            hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
                            hex!("9a3ad932c32fd1b71c85ff3833c48d02fe9b322ab74ad16bec2182effa621a83").into(),
                    ],
                }
            },
        },
        finalized_header: BeaconHeader {
            slot: 896,
            proposer_index: 5,
            parent_root: hex!("d2aa51a51789f4d3e3bbe335a603f9a0ce7b80e9203bdb1a56b3dba53ce48941").into(),
            state_root: hex!("a7ba9d3ae073b11d055ced30dbbe915d6ce66df4f21c511200c4e6ee6970c0a4").into(),
            body_root: hex!("f92b843758729c91c612f38e2fcfc7ebebe936abc26af2f2896572a3fd20e61a").into(),
        },
        block_roots_root: hex!("577f8305f63ff2dc2b1ebd6363004d853824ccd23af1a09b99bb9f83ab365061").into(),
    }
}
