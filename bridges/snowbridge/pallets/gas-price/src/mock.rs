use frame_support::{derive_impl, parameter_types};
use frame_system as system;
use sp_runtime::{BuildStorage, FixedU128};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		GasPrice: crate::{Pallet, Storage, Event<T>},
	}
);

parameter_types! {
	pub const SS58Prefix: u8 = 42;
	pub const WeightingFactor: FixedU128 = FixedU128::from_rational(2, 10);
	pub const BaseFeeMultiplier: FixedU128 = FixedU128::from_rational(4, 3);
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl system::Config for Test {
	type Block = Block;
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightingFactor = WeightingFactor;
	type BaseFeeMultiplier = BaseFeeMultiplier;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
