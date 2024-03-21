//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::prelude::*;
use frame_support::storage::types::StorageValue;
use sp_core::U256;
use snowbridge_core::PricingParameters;
use snowbridge_core::inbound::Verifier;
use snowbridge_beacon_primitives::ExecutionProof;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

struct BaseFeeState {
	value: U256,
	slot: u64,
}

#[frame::pallet]
pub mod pallet {
	use frame_system::ensure_signed;

use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type PricingParameters: Get<PricingParameters<Self::Balance>>;
		type Verifier: Verifier;
		type Multiplier: FixedU128;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type BaseFee<T: Config> = StorageValue<_, BaseFeeState, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid Update
		InvalidUpdate,
	}

	impl<T: Config> Pallet<T> {
		#[pallet::call_index(1)]
		#[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
		pub fn update(origin: OriginFor<T>, proof: ExecutionProof) -> DispatchResult {
			ensure_signed(origin)?;

			if BaseFee::<T>::get().slot >= proof.header.slot {
				return Err(Error::<T>::InvalidUpdate.into())
			} 

			Verifier::verify_execution_proof(&proof)?;

			BaseFee::<T>::mutate(|bf| {
				bf.value = proof.execution_header.base_fee_per_gas;
				bf.slot = proof.header.slot;
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn estimate_gas_price() -> U256 {
			let base_fee = BaseFee::<T>::get();
			if base_fee == 0 {
				
			}
		}
	}
}
