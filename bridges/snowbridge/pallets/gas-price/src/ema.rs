// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
//! Exponential Moving Average
use sp_arithmetic::traits::One;
use sp_runtime::{FixedU128, Saturating};

/// Perform one iteration of the EMA filter defined by:
///   ```text
///   ema_1 = weighting_factor * value + ema_0 * (1 - weighting_factor)
///   ```
pub fn step(weighting_factor: FixedU128, ema: FixedU128, value: FixedU128) -> FixedU128 {
	weighting_factor
		.saturating_mul(value)
		.saturating_add(ema.saturating_mul(FixedU128::one().saturating_sub(weighting_factor)))
}

#[cfg(test)]
mod test {
	use super::*;
	use snowbridge_core::GWEI;
	use sp_arithmetic::traits::{One, Zero};

	#[test]
	pub fn ema_step() {
		let weighting_factor = FixedU128::from_rational(2, 10);
		let ema0 = FixedU128::from_inner(60 * GWEI);
		let value = FixedU128::from_inner(90 * GWEI);
		assert_eq!(step(weighting_factor, ema0, value).into_inner(), 66 * GWEI);
	}

	#[test]
	pub fn ema_step_with_weighting_factor_at_bounds() {
		let weighting_factor = FixedU128::zero();
		let ema0 = FixedU128::from_inner(60 * GWEI);
		let value = FixedU128::from_inner(90 * GWEI);
		assert_eq!(step(weighting_factor, ema0, value).into_inner(), 60 * GWEI);

		let weighting_factor = FixedU128::one();
		let ema0 = FixedU128::from_inner(60 * GWEI);
		let value = FixedU128::from_inner(90 * GWEI);
		assert_eq!(step(weighting_factor, ema0, value).into_inner(), 90 * GWEI);
	}
}
