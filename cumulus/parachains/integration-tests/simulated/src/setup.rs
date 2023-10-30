use frame_support::traits::GenesisBuild;

pub use frame_support::pallet_prelude::Weight;
use frame_support::traits::Currency;
use parachains_common::{AccountId, Balance};
use polkadot_parachain_primitives::primitives::Sibling;
use sp_runtime::{traits::AccountIdConversion, BuildStorage, MultiAddress};

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];
pub const ROC_DECIMAL: u32 = 12;
pub const WETH_DECIMAL: u8 = 18;
pub const WETH_ASSET_ID: u32 = 1;

pub fn roc(n: u128) -> Balance {
	(n as u128) * 10u128.pow(ROC_DECIMAL)
}

pub fn weth(n: u128) -> Balance {
	(n as u128) * 10u128.pow(WETH_DECIMAL as u32)
}

pub struct ExtBuilder {
	pub parachain_id: u32,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { parachain_id: 1013 }
	}
}

impl ExtBuilder {
	pub fn parachain_id(mut self, parachain_id: u32) -> Self {
		self.parachain_id = parachain_id;
		self
	}

	pub fn build_bridge_hub(self) -> sp_io::TestExternalities {
		use bridge_hub_rococo_runtime::{ParachainSystem, Runtime, System};
		let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

		parachain_info::GenesisConfig::<Runtime> {
			parachain_id: self.parachain_id.into(),
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_xcm::GenesisConfig::<Runtime> { safe_xcm_version: Some(3), ..Default::default() }
			.assimilate_storage(&mut t)
			.unwrap();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: vec![(AccountId::from(ALICE), roc(100)), (AccountId::from(BOB), roc(100))],
		}
		.assimilate_storage(&mut t)
		.unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| {
			System::set_block_number(1);
			ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(1000.into());
		});
		ext
	}

	pub fn build_asset_hub(self) -> sp_io::TestExternalities {
		use asset_hub_rococo_runtime::{
			Assets, Balances, ParachainSystem, Runtime, RuntimeOrigin, System,
		};

		let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

		parachain_info::GenesisConfig::<Runtime> {
			parachain_id: self.parachain_id.into(),
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_xcm::GenesisConfig::<Runtime> { safe_xcm_version: Some(3), ..Default::default() }
			.assimilate_storage(&mut t)
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| {
			System::set_block_number(1);

			ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(1013.into());

			Balances::make_free_balance_be(&ALICE.into(), roc(1));

			// sibling account need to have some ROC to be able to receive non_sufficient assets
			let para_acc: AccountId = Sibling::from(1013).into_account_truncating();
			println!("bridgehub account:{:?}", para_acc);
			Balances::make_free_balance_be(&para_acc, roc(1));

			// prepare for weth
			Assets::force_create(
				RuntimeOrigin::root(),
				WETH_ASSET_ID.into(),
				MultiAddress::Id(AccountId::from(ALICE)),
				true,
				1,
			)
			.unwrap();
			Assets::force_set_metadata(
				RuntimeOrigin::root(),
				WETH_ASSET_ID.into(),
				b"WETH".to_vec(),
				b"WETH".to_vec(),
				WETH_DECIMAL,
				false,
			)
			.unwrap();
			Assets::mint(
				RuntimeOrigin::signed(AccountId::from(ALICE)),
				WETH_ASSET_ID.into(),
				MultiAddress::Id(AccountId::from(ALICE)),
				weth(1),
			)
			.unwrap();
		});
		ext
	}

	pub fn build_template(self) -> sp_io::TestExternalities {
		use parachain_template_runtime::{ParachainSystem, Runtime, System};
		let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

		parachain_info::GenesisConfig::<Runtime> {
			parachain_id: self.parachain_id.into(),
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_xcm::GenesisConfig::<Runtime> { safe_xcm_version: Some(3), ..Default::default() }
			.assimilate_storage(&mut t)
			.unwrap();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: vec![(AccountId::from(ALICE), roc(100)), (AccountId::from(BOB), roc(100))],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| {
			System::set_block_number(1);
			ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(1013.into());
		});
		ext
	}
}
