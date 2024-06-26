use super::*;
use crate::mock::*;

#[test]
fn update_value_works() {
	new_test_ext().execute_with(|| {
		GasPrice::update(gwei(25), 20);
		let price = GasPrice::get();
		assert_eq!(price, gwei(25));
		GasPrice::update(gwei(25), 30);
		let price = GasPrice::get();
		assert_eq!(price, gwei(25));

		// Update with price increased, the updated value should be more than the previous but less
		// than the updating one
		GasPrice::update(gwei(30), 40);
		let price = GasPrice::get();
		assert_eq!(price, gwei(26));

		// Update with price decreased, the updated value should be less than the previous but more
		// than the updating one
		GasPrice::update(gwei(20), 50);
		let price = GasPrice::get();
		assert_eq!(price, 24800000000_u128.into());

		// Update with a large interval, the new value should dominate the EMA
		GasPrice::update(gwei(3), 50 + 8192);
		let price = GasPrice::get();
		assert_eq!(price, gwei(3).into());

		// Update with an outdated price, the new value won't change
		GasPrice::update(gwei(5), 50 + 8192 - 5);
		let price = GasPrice::get();
		assert_eq!(price, gwei(3).into());
	});
}
