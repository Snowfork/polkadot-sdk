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
use snowbridge_core::{gwei, BaseFeePerGas, GasPriceProvider};
use sp_arithmetic::traits::One;
use sp_core::{Get, U256};
use sp_runtime::{FixedU128, Saturating};

pub const LOG_TARGET: &str = "gas-price";

const BLENDING_FACTOR: FixedU128 = FixedU128::from_rational(20, 100);

#[derive(scale_info::TypeInfo, codec::Encode, codec::Decode, codec::MaxEncodedLen)]
pub struct DefaultFeePerGas;
impl Get<U256> for DefaultFeePerGas {
	fn get() -> U256 {
		gwei(20)
	}
}

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
		Updated { value: U256, accumulated_value: U256, slot: u64 },
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
			let mut accumulated_value: U256 = AccumulatedGasPrice::<T>::get().value;
			if accumulated_value.is_zero() {
				accumulated_value = DefaultFeePerGas::get();
			}
			accumulated_value
		}
	}

	impl<T: Config> GasPriceProvider for Pallet<T> {
		fn update(value: U256, slot: u64) {
			let mut accumulated_value: U256 = <AccumulatedGasPrice<T>>::get().value;
			let last_updated_slot = <AccumulatedGasPrice<T>>::get().slot;
			if accumulated_value.is_zero() {
				accumulated_value = DefaultFeePerGas::get();
			}

			let fixed_value = FixedU128::from_inner(value.low_u128());
			let mut accumulated_fixed_value = FixedU128::from_inner(accumulated_value.low_u128());
			let scaling_factor = sp_std::cmp::max(
				BLENDING_FACTOR,
				sp_std::cmp::min(
					FixedU128::one(),
					FixedU128::from_rational((slot - last_updated_slot).into(), 8192),
				),
			);

			if fixed_value > accumulated_fixed_value {
				accumulated_fixed_value = accumulated_fixed_value.saturating_add(
					fixed_value
						.saturating_sub(accumulated_fixed_value)
						.saturating_mul(scaling_factor),
				);
			} else {
				accumulated_fixed_value = accumulated_fixed_value.saturating_sub(
					accumulated_fixed_value
						.saturating_sub(fixed_value)
						.saturating_mul(scaling_factor),
				);
			}

			let accumulated_value = U256::from(accumulated_fixed_value.into_inner());
			<AccumulatedGasPrice<T>>::set(BaseFeePerGas { value: accumulated_value, slot });

			Self::deposit_event(Event::Updated { value, accumulated_value, slot });
		}
	}
}
