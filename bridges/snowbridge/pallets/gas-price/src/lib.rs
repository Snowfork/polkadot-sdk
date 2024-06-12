// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod test;

use frame_support::pallet_prelude::ValueQuery;
use frame_system::WeightInfo;
pub use pallet::*;
use snowbridge_core::{BaseFeePerGas, GasPriceProvider};
use sp_core::{Get, U256};
use sp_runtime::{FixedU128, Saturating};

pub const LOG_TARGET: &str = "gas-price";

const BLENDING_FACTOR: FixedU128 = FixedU128::from_rational(20, 100);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Updated { value: U256, cumulated_value: U256, slot: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {}

	/// Gas price
	#[pallet::storage]
	#[pallet::getter(fn gas_price)]
	pub(super) type AccumulatedGasPrice<T: Config> = StorageValue<_, BaseFeePerGas, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Get<U256> for Pallet<T> {
		fn get() -> U256 {
			AccumulatedGasPrice::<T>::get().value
		}
	}

	impl<T: Config> GasPriceProvider for Pallet<T> {
		fn update(value: U256, slot: u64) {
			let cumulated_value: U256 = <AccumulatedGasPrice<T>>::get().value;

			let fixed_value = FixedU128::from_inner(value.low_u128());
			let cumulated_fixed_value = FixedU128::from_inner(cumulated_value.low_u128());

			let cumulated_fixed_value_updated: FixedU128;
			if fixed_value > cumulated_fixed_value {
				cumulated_fixed_value_updated = cumulated_fixed_value.saturating_add(
					fixed_value
						.saturating_sub(cumulated_fixed_value)
						.saturating_mul(BLENDING_FACTOR),
				);
			} else {
				cumulated_fixed_value_updated = cumulated_fixed_value.saturating_sub(
					cumulated_fixed_value
						.saturating_sub(fixed_value)
						.saturating_mul(BLENDING_FACTOR),
				);
			}

			let cumulated_value = U256::from(cumulated_fixed_value_updated.into_inner());
			<AccumulatedGasPrice<T>>::set(BaseFeePerGas { value: cumulated_value, slot });

			Self::deposit_event(Event::Updated { value, cumulated_value, slot });
		}
	}
}
