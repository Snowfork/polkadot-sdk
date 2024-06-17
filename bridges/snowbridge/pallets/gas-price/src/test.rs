use super::*;
use crate::mock::*;

#[test]
fn update_value_works() {
	new_test_ext().execute_with(|| {
		GasPrice::update(gwei(25), 20);
		let price = GasPrice::get();
		assert_eq!(price, 21000000000_u128.into());
		GasPrice::update(gwei(25), 30);
		let price = GasPrice::get();
		assert_eq!(price, 21800000000_u128.into());
		GasPrice::update(gwei(30), 40);
		let price = GasPrice::get();
		assert_eq!(price, 23440000000_u128.into());

		// Update with price decreased, the updated value should be less than the previous but more
		// than the new one
		GasPrice::update(gwei(20), 50);
		let price = GasPrice::get();
		assert_eq!(price, 22752000000_u128.into());

		// Update with a large interval, the new value should dominate the EMA
		GasPrice::update(gwei(30), 50 + 8192);
		let price = GasPrice::get();
		assert_eq!(price, gwei(30).into());
	});
}
