use super::*;
use crate::mock::*;

#[test]
fn update_value_works() {
	new_test_ext().execute_with(|| {
		GasPrice::update(U256::from(1000), 10);
		GasPrice::update(U256::from(2000), 20);
		let price = GasPrice::get();
		assert_eq!(price, 560.into());
		GasPrice::update(U256::from(3000), 30);
		let price = GasPrice::get();
		assert_eq!(price, 1048.into());
		GasPrice::update(U256::from(4000), 40);
		let price = GasPrice::get();
		assert_eq!(price, 1638.into());
		GasPrice::update(U256::from(4000), 50);
		let price = GasPrice::get();
		assert_eq!(price, 2110.into());
		GasPrice::update(U256::from(3000), 60);
		let price = GasPrice::get();
		assert_eq!(price, 2288.into());
		GasPrice::update(U256::from(2000), 70);
		let price = GasPrice::get();
		assert_eq!(price, 2231.into());
	});
}
