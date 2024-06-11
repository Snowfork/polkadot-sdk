// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod test;

use frame_support::pallet_prelude::{ConstU32, ValueQuery};
use frame_system::WeightInfo;
pub use pallet::*;
use snowbridge_core::{BaseFeePerGas, GasPriceProvider, RingBufferMap, RingBufferMapImpl};
use sp_core::{Get, U256};
pub const LOG_TARGET: &str = "gas-price";

pub const BUFFER_SIZE: u32 = 50;
pub const TWAP_INTERVAL: u32 = 3;
pub type CumulatedPriceBuffer<T> = RingBufferMapImpl<
	u32,
	ConstU32<BUFFER_SIZE>,
	PriceIndex<T>,
	PriceStateMapping<T>,
	PriceState<T>,
	ValueQuery,
>;

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
	pub(super) type InstantGasPrice<T: Config> = StorageValue<_, BaseFeePerGas, ValueQuery>;

	#[pallet::storage]
	pub type PriceIndex<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type PriceStateMapping<T: Config> = StorageMap<_, Identity, u32, u64, ValueQuery>;

	#[pallet::storage]
	pub type PriceState<T: Config> = StorageMap<_, Identity, u64, U256, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Get<U256> for Pallet<T> {
		fn get() -> U256 {
			let index = PriceIndex::<T>::get();
			let slot = <PriceStateMapping<T>>::get(index);
			let value = <CumulatedPriceBuffer<T>>::get(slot);

			let previous_index;
			if index >= TWAP_INTERVAL {
				previous_index = index - TWAP_INTERVAL;
			} else {
				previous_index = index + BUFFER_SIZE - TWAP_INTERVAL;
			}
			let previous_slot = <PriceStateMapping<T>>::get(previous_index);
			let previous_value = <CumulatedPriceBuffer<T>>::get(previous_slot);

			let price: U256;
			if previous_slot > 0 && slot > previous_slot {
				price = value
					.saturating_sub(previous_value)
					.checked_div(U256::from(slot - previous_slot))
					.expect("divisor is non-zero; qed")
					.into();
			} else {
				price = InstantGasPrice::<T>::get().value
			}
			price
		}
	}

	impl<T: Config> GasPriceProvider for Pallet<T> {
		fn update(value: U256, slot: u64) {
			let previous_index = PriceIndex::<T>::get();
			let previous_slot = <PriceStateMapping<T>>::get(previous_index);
			let previous_value = <PriceState<T>>::get(previous_slot);
			let cumulated_value: U256;
			if previous_slot > 0 && slot > previous_slot {
				cumulated_value = previous_value
					.saturating_add(value.saturating_mul(U256::from(slot - previous_slot)));
			} else {
				cumulated_value = U256::zero()
			}

			<CumulatedPriceBuffer<T>>::insert(slot, cumulated_value);
			<InstantGasPrice<T>>::set(BaseFeePerGas { value, slot });
			Self::deposit_event(Event::Updated { value, cumulated_value, slot });
		}
	}
}
