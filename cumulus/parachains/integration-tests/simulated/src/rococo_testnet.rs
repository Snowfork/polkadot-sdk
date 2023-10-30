use crate::setup::*;
use cumulus_primitives_core::ParaId;
use parachains_common::{AccountId, BlockNumber};
use polkadot_runtime_parachains::configuration::HostConfiguration;
use sp_runtime::traits::AccountIdConversion;
use xcm_emulator::polkadot_primitives::{MAX_CODE_SIZE, MAX_POV_SIZE};

use sp_runtime::BuildStorage;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

decl_test_relay_chain! {
	pub struct RococoNet {
		Runtime = rococo_runtime::Runtime,
		RuntimeCall = rococo_runtime::RuntimeCall,
		RuntimeEvent = rococo_runtime::RuntimeEvent,
		XcmConfig = rococo_runtime::xcm_config::XcmConfig,
		MessageQueue = rococo_runtime::MessageQueue,
		System = rococo_runtime::System,
		new_ext = rococo_ext(),
	}
}

decl_test_parachain! {
	pub struct BridgeHub {
		Runtime = bridge_hub_rococo_runtime::Runtime,
		XcmpMessageHandler = bridge_hub_rococo_runtime::XcmpQueue,
		DmpMessageHandler = bridge_hub_rococo_runtime::DmpQueue,
		new_ext = bridge_hub_ext(),
	}
}

decl_test_parachain! {
	pub struct AssetHub {
		Runtime = asset_hub_rococo_runtime::Runtime,
		XcmpMessageHandler = bridge_hub_rococo_runtime::XcmpQueue,
		DmpMessageHandler = asset_hub_rococo_runtime::DmpQueue,
		new_ext = asset_hub_ext(),
	}
}

decl_test_parachain! {
	pub struct Template {
		Runtime = parachain_template_runtime::Runtime,
		XcmpMessageHandler = parachain_template_runtime::XcmpQueue,
		DmpMessageHandler = parachain_template_runtime::DmpQueue,
		new_ext = template_ext(),
	}
}

decl_test_network! {
	pub struct TestNet {
		relay_chain = RococoNet,
		parachains = vec![
			(1000, AssetHub),
			(1013, BridgeHub),
			(1001, Template),
		],
	}
}

fn default_parachains_host_configuration() -> HostConfiguration<BlockNumber> {
	HostConfiguration {
		validation_upgrade_cooldown: 2u32,
		validation_upgrade_delay: 2,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024 * 1024,
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		minimum_validation_upgrade_delay: 5,
		..Default::default()
	}
}

pub fn rococo_ext() -> sp_io::TestExternalities {
	use rococo_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(AccountId::from(ALICE), roc(100)),
			(ParaId::from(1013 as u32).into_account_truncating(), roc(100)),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
		config: default_parachains_host_configuration(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn bridge_hub_ext() -> sp_io::TestExternalities {
	let parachain_id = 1013;
	let ext = ExtBuilder { parachain_id };
	ext.parachain_id(parachain_id).build_bridge_hub()
}

pub fn asset_hub_ext() -> sp_io::TestExternalities {
	let parachain_id = 1000;
	let ext = ExtBuilder { parachain_id };
	ext.parachain_id(parachain_id).build_asset_hub()
}

pub fn template_ext() -> sp_io::TestExternalities {
	let parachain_id = 1001;
	let ext = ExtBuilder { parachain_id };
	ext.parachain_id(parachain_id).build_template()
}
