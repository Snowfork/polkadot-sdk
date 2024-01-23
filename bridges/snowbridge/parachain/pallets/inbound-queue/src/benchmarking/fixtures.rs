// Generated, do not edit!
// See ethereum client README.md for instructions to generate
use hex_literal::hex;
use snowbridge_beacon_primitives::CompactExecutionHeader;
use snowbridge_core::inbound::{Log, Message, Proof};
use sp_std::vec;

pub struct InboundQueueTest {
	pub execution_header: CompactExecutionHeader,
	pub message: Message,
}

pub fn make_create_message() -> InboundQueueTest {
	InboundQueueTest {
        execution_header: CompactExecutionHeader{
            parent_hash: hex!("d82ec63f5c5e6ba61d62f09c188f158e6449b94bdcc31941e68639eec3c4cf7a").into(),
            block_number: 215,
            state_root: hex!("8b65545fe5f3216b47b6339b9c91ca2b7f1032a970b04246d9e9fb4460ee34c3").into(),
            receipts_root: hex!("7b1f61b9714c080ef0be014e01657a15f45f0304b477beebc7ca5596c8033095").into(),
        },
        message: Message {
            event_log: 	Log {
                address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
                topics: vec![
                    hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
                    hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
                    hex!("5f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0").into(),
                ],
                data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e00a736aa00000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").into(),
            },
            proof: Proof {
                block_hash: hex!("48498dbfbcfae53a7f4c289ee00747aceea925f6260c50ead5a33e1c55c40f98").into(),
                tx_index: 0,
                data: (vec![
                    hex!("7b1f61b9714c080ef0be014e01657a15f45f0304b477beebc7ca5596c8033095").to_vec(),
                ], vec![
                    hex!("f9028e822080b9028802f90284018301d205b9010000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010f90179f85894eda338e4dc46038493b885327842fd3e301cab39e1a0f78bb28d4b1d7da699e5c0bc2be29c2b04b5aab6aacf6298fe5304f9db9c6d7ea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7df9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a05f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0b8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e00a736aa00000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").to_vec(),
                ]),
            },
        },
    }
}
