use super::*;
use crate::mock::*;

#[test]
fn update_value_works() {
	new_test_ext().execute_with(|| {
		GasPrice::update(U256::from(1000), 10);
		GasPrice::update(U256::from(2000), 20);
		let price = GasPrice::get();
		assert_eq!(price, 2000.into());
		GasPrice::update(U256::from(3000), 30);
		let price = GasPrice::get();
		assert_eq!(price, 3000.into());
		GasPrice::update(U256::from(4000), 40);
		let price = GasPrice::get();
		assert_eq!(price, 3000.into());
		GasPrice::update(U256::from(4000), 50);
		let price = GasPrice::get();
		assert_eq!(price, 3666.into());
		GasPrice::update(U256::from(4000), 60);
		let price = GasPrice::get();
		assert_eq!(price, 4000.into());
	});
}
