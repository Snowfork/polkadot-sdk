// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::H160;
	use xcm::prelude::*;
	use xcm_executor::traits::XcmAssetTransfers;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type XcmRouter: SendXcm;
		type XcmExecutor: ExecuteXcm<Self::RuntimeCall> + XcmAssetTransfers;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// ExportMessage message was sent
		SentExportMessage { message_id: XcmHash, sender_cost: Assets, message: Xcm<()> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidMsg,
		FeesNotMet,
		SendFailure,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct TransactInfo {
		pub target: H160,
		pub call: Vec<u8>,
		pub gas_limit: u64,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		[u8; 32]: From<<T as frame_system::Config>::AccountId>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(100_000_000, 0))]
		pub fn transact_to_ethereum(
			origin: OriginFor<T>,
			target: H160,
			call: Vec<u8>,
			fee: u128,
			gas_limit: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let dest = Location {
				parents: 2,
				interior: Junctions::from([GlobalConsensus(Ethereum { chain_id: 11155111 })]),
			};
			let transact = TransactInfo { target, call, gas_limit };

			let inner_message = Xcm(vec![
				Transact {
					origin_kind: OriginKind::SovereignAccount,
					require_weight_at_most: Weight::default(),
					call: transact.encode().into(),
				},
				// Optional only for trace
				SetTopic([0; 32]),
			]);

			let (ticket, price) =
				validate_send::<T::XcmRouter>(dest.clone(), inner_message.clone())
					.map_err(|_| Error::<T>::InvalidMsg)?;
			ensure!(price.len() > 0, Error::<T>::FeesNotMet);

			let mut fees: Assets = (Parent, fee).into();
			fees.push(price.get(0).unwrap().clone());

			let origin = Location::from(AccountId32 { network: None, id: who.into() });

			T::XcmExecutor::charge_fees(origin, fees.clone().into())
				.map_err(|_| Error::<T>::FeesNotMet)?;

			let message_id = T::XcmRouter::deliver(ticket).map_err(|_| Error::<T>::SendFailure)?;

			Self::deposit_event(Event::SentExportMessage {
				message_id,
				sender_cost: fees.into(),
				message: inner_message,
			});
			Ok(())
		}
	}
}
