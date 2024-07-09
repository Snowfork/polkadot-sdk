// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

mod ema;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod test;

pub use pallet::*;
use snowbridge_core::{GasPriceEstimator, GWEI};
use sp_arithmetic::traits::One;
use sp_core::Get;
use sp_runtime::{FixedU128, Saturating};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The weighting factor used to compute the EMA of the EIP-1559 `BaseFeePerGas` variable.
		#[pallet::constant]
		type WeightingFactor: Get<FixedU128>;

		/// The multiplier used to compute an estimate of the EIP-1559 `MaxFeePerGas` variable
		#[pallet::constant]
		type BaseFeeMultiplier: Get<FixedU128>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The average base fee has been updated
		Updated { average_base_fee: u128 },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::type_value]
	pub fn InitialBaseFee() -> u128 {
		20.saturating_mul(GWEI)
	}

	#[pallet::storage]
	pub(super) type AverageBaseFee<T: Config> = StorageValue<_, u128, ValueQuery, InitialBaseFee>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn integrity_test() {
			assert!(T::WeightingFactor::get() <= FixedU128::one());
			assert!(T::BaseFeeMultiplier::get() >= FixedU128::one());
		}
	}

	impl<T: Config> Pallet<T> {
		/// Estimate the EIP-1559 `MaxFeePerGas` variable
		fn estimate_max_fee() -> u128 {
			let average_base_fee = FixedU128::from_inner(AverageBaseFee::<T>::get());
			T::BaseFeeMultiplier::get().saturating_mul(average_base_fee).into_inner()
		}

		/// Update the EMA of the EIP-1559 `BaseFeePerGas` variable
		fn do_update(base_fee: u128) {
			let average_base_fee = ema::step(
				T::WeightingFactor::get(),
				FixedU128::from_inner(AverageBaseFee::<T>::get()),
				FixedU128::from_inner(base_fee),
			)
			.into_inner();

			AverageBaseFee::<T>::put(average_base_fee);

			Self::deposit_event(Event::Updated { average_base_fee });
		}
	}

	impl<T: Config> GasPriceEstimator for Pallet<T> {
		fn update(base_fee_per_gas: u128) {
			Self::do_update(base_fee_per_gas)
		}

		fn max_fee() -> u128 {
			Self::estimate_max_fee()
		}

		fn base_fee() -> u128 {
			AverageBaseFee::<T>::get()
		}
	}
}
