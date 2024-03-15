// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use crate as ethereum_beacon_client;
use crate::config;
use frame_support::{derive_impl, parameter_types};
use hex_literal::hex;
use pallet_timestamp;
use primitives::{
	types::deneb, AncestryProof, BeaconHeader, ExecutionProof, Fork, ForkVersions,
	VersionedExecutionPayloadHeader,
};
use snowbridge_core::inbound::{Log, Proof};
use sp_std::default::Default;
use std::{fs::File, path::PathBuf};

type Block = frame_system::mocking::MockBlock<Test>;
use sp_runtime::BuildStorage;

fn load_fixture<T>(basename: String) -> Result<T, serde_json::Error>
where
	T: for<'de> serde::Deserialize<'de>,
{
	let filepath: PathBuf =
		[env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", &basename].iter().collect();
	serde_json::from_reader(File::open(filepath).unwrap())
}

pub fn load_execution_proof_fixture() -> primitives::ExecutionProof {
	load_fixture("execution-proof.json".to_string()).unwrap()
}

pub fn load_checkpoint_update_fixture(
) -> primitives::CheckpointUpdate<{ config::SYNC_COMMITTEE_SIZE }> {
	load_fixture("initial-checkpoint.json".to_string()).unwrap()
}

pub fn load_sync_committee_update_fixture(
) -> primitives::Update<{ config::SYNC_COMMITTEE_SIZE }, { config::SYNC_COMMITTEE_BITS_SIZE }> {
	load_fixture("sync-committee-update.json".to_string()).unwrap()
}

pub fn load_finalized_header_update_fixture(
) -> primitives::Update<{ config::SYNC_COMMITTEE_SIZE }, { config::SYNC_COMMITTEE_BITS_SIZE }> {
	load_fixture("finalized-header-update.json".to_string()).unwrap()
}

pub fn load_next_sync_committee_update_fixture(
) -> primitives::Update<{ config::SYNC_COMMITTEE_SIZE }, { config::SYNC_COMMITTEE_BITS_SIZE }> {
	load_fixture("next-sync-committee-update.json".to_string()).unwrap()
}

pub fn load_next_finalized_header_update_fixture(
) -> primitives::Update<{ config::SYNC_COMMITTEE_SIZE }, { config::SYNC_COMMITTEE_BITS_SIZE }> {
	load_fixture("next-finalized-header-update.json".to_string()).unwrap()
}

pub fn get_message_verification_payload() -> (Log, Proof) {
	(
		Log {
			address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
			topics: vec![
				hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
				hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
				hex!("5f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0").into(),
			],
			data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e00a736aa00000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").into(),
		},
		Proof {
			block_hash: hex!("6c4cbc268b13befb5192ee25580b81c65b5ba53f16a8cff449a2527f1e038ea7").into(),
			tx_index: 0,
			receipt_proof: (vec![
				hex!("7b1f61b9714c080ef0be014e01657a15f45f0304b477beebc7ca5596c8033095").to_vec(),
			], vec![
				hex!("f9028e822080b9028802f90284018301d205b9010000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010f90179f85894eda338e4dc46038493b885327842fd3e301cab39e1a0f78bb28d4b1d7da699e5c0bc2be29c2b04b5aab6aacf6298fe5304f9db9c6d7ea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7df9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a05f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0b8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e00a736aa00000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").to_vec(),
			]),
			execution_proof: ExecutionProof {
				header: BeaconHeader{
					slot: 648,
					proposer_index: 7,
					parent_root: hex!("bd5248653d7d01363ac160de890de774c1981eb13488cdbe804168382e8dfcc9").into(),
					state_root: hex!("49e82d4e3c9f8bfda9b4bf5936b777ebd568ba45c6c9f3ae4cea7d40473ecb8f").into(),
					body_root: hex!("1b040f1609bd23f7ed4816f4e706a1ba001b3a1a2a79c46def42f80623bfd1db").into(),
				},
				ancestry_proof: Some(AncestryProof{
					header_branch: vec![
					hex!("212e1fecbf05fce64dedded61106b04e7fb06fa146536c0741fd32ba525e3bad").into(),
					hex!("ff6347f8d1229e49bc374ed4d2475fa707f72b83c09797c597e8517f6b1e0e93").into(),
					hex!("0e667c8b52af8edd069b822ae4294fcfa971561195645a102b4cff5fe7ec2c09").into(),
					hex!("46099bee90675a08a61ddc5d31eb6a3fc8e01ecffbdc37513dda9898a4f9ddfc").into(),
					hex!("72794001d1b54bec48a3b01bc0bc4b294d9189362de865912074e22ee4438548").into(),
					hex!("472fdf08f4d9e1fbaa6a6901ce2f49f922d5ee823662763f3d72949185883121").into(),
					hex!("3e4aaf8fe909659181989f1157cdf8d1b0176ca53252b12042e69add20e4ecc6").into(),
					hex!("785f8055007000f172f93e5c7e155002230018576774a9f5ee30980fa136c665").into(),
					hex!("713489875f582f1b8b5e9115b45801ca4e138f3ba76addea16c9dbff3464d2fa").into(),
					hex!("47ca5b010188b07de1c718194013f32d93b8068d086ff839974bca682eccd47b").into(),
					hex!("f190f972d47e327338d808217419b69afefa0d2aeb5e2a94b4ad9328a3363ab6").into(),
					hex!("83a01ec0c56356a2aba5a942c9b0e13c38543aa27a150040e854ac10975919bd").into(),
					hex!("63e4a69f56d7a80c14c9987b202f22bb58021d11f514d330a8a45496e4a70d61").into(),
					],
					finalized_block_root: hex!("8ddbf175e0c55643f247b766c91f2bbc31ea401e1e9543b833e4eda78995ce9a").into()
				}),
				execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
					parent_hash: hex!("b085a054359c1a7a97ae4a50e71f564d21238b5197d335cf7d344c5c9e4255af").into(),
					fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
					state_root: hex!("92d6b19aba641c71f71d8a8a7149f79247a35f3428935b047bccb8dc10016fae").into(),
					receipts_root: hex!("7b1f61b9714c080ef0be014e01657a15f45f0304b477beebc7ca5596c8033095").into(),
					logs_bloom: hex!("00000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010").to_vec(),
					prev_randao: hex!("07c462f1c08b6c5310fecdd7988445d64625e056087d7f70cf773250cc7dd51f").into(),
					block_number: 648,
					gas_limit: 42475260,
					gas_used: 119301,
					timestamp: 1710489135,
					extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").to_vec(),
					base_fee_per_gas: 7.into(),
					block_hash: hex!("6c4cbc268b13befb5192ee25580b81c65b5ba53f16a8cff449a2527f1e038ea7").into(),
					transactions_root: hex!("5ebc1347fe3df0611d4f66b19bd8e1c6f4eaed0371d850f14c83b1c77ea234e6").into(),
					withdrawals_root: hex!("792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535").into(),
					blob_gas_used: 0,
					excess_blob_gas: 0,
				}),
				execution_branch: vec![
					hex!("c318d45b48b0b7fdce4b264707bc5ac005a1fde87ed75f67170d1833239ab1d0").into(),
					hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
					hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
					hex!("a33039cf024db93fd798e6b897fc8e668fd06469f2d9bf10679811f750c2611d").into(),
				],
			},
		},
	)
}

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		EthereumBeaconClient: ethereum_beacon_client::{Pallet, Call, Storage, Event<T>},
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const ChainForkVersions: ForkVersions = ForkVersions {
		genesis: Fork {
			version: [0, 0, 0, 0], // 0x00000000
			epoch: 0,
		},
		altair: Fork {
			version: [1, 0, 0, 0], // 0x01000000
			epoch: 0,
		},
		bellatrix: Fork {
			version: [2, 0, 0, 0], // 0x02000000
			epoch: 0,
		},
		capella: Fork {
			version: [3, 0, 0, 0], // 0x03000000
			epoch: 0,
		},
		deneb: Fork {
			version: [4, 0, 0, 0], // 0x90000073
			epoch: 0,
		}
	};
	pub const ExecutionHeadersPruneThreshold: u32 = 8192;
}

impl ethereum_beacon_client::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ForkVersions = ChainForkVersions;
	type MaxExecutionHeadersToKeep = ExecutionHeadersPruneThreshold;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_tester() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	let _ = ext.execute_with(|| Timestamp::set(RuntimeOrigin::signed(1), 30_000));
	ext
}
