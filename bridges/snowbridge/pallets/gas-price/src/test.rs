use super::*;
use crate::mock::*;

#[test]
fn estimate_max_fee() {
	new_test_ext().execute_with(|| {
		AverageBaseFee::<Test>::put(60 * GWEI);
		assert_eq!(GasPrice::max_fee(), 79999999999);
	});
}

#[test]
fn update_base_fee() {
	new_test_ext().execute_with(|| {
		AverageBaseFee::<Test>::put(60 * GWEI);

		GasPrice::update(90 * GWEI);
		assert_eq!(GasPrice::base_fee(), 66000000000);

		GasPrice::update(90 * GWEI);
		assert_eq!(GasPrice::base_fee(), 70800000000);
	});
}
