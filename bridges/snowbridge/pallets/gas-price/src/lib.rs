// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::WeightInfo;
pub use pallet::*;
use snowbridge_core::{BaseFeePerGas, GasPriceProvider};
use sp_core::U256;
pub const LOG_TARGET: &str = "gas-price";

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
		Updated { value: U256, slot: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {}

	/// Gas price
	#[pallet::storage]
	#[pallet::getter(fn gas_price)]
	pub(super) type GasPrice<T: Config> = StorageValue<_, BaseFeePerGas, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

impl<T: Config> GasPriceProvider for Pallet<T> {
	fn update(value: U256, slot: u64) {
		<GasPrice<T>>::set(BaseFeePerGas { value, slot });

		Self::deposit_event(Event::GasPriceUpdate { value, slot });
	}

	fn get() -> BaseFeePerGas {
		GasPrice::<T>::get()
	}
}
