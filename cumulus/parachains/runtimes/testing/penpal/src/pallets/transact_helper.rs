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
	use sp_core::{H160, H256};
	use sp_std::{boxed::Box, vec, vec::Vec};
	use xcm::prelude::*;
	use xcm_executor::traits::XcmAssetTransfers;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_xcm::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type XcmRouter: SendXcm;
		type XcmExecutor: ExecuteXcm<<Self as frame_system::Config>::RuntimeCall>
			+ XcmAssetTransfers;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// ExportMessage was sent
		SentExportMessage { message_id: XcmHash, sender_cost: Assets, message: Xcm<()> },
		/// XCM message sent. \[to, message\]
		Sent { to: Location, message: Xcm<()> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidMsg,
		FeesNotMet,
		SendFailure,
		BadVersion,
		Unreachable,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct TransactInfo {
		pub agent_id: H256,
		pub target: H160,
		pub call: Vec<u8>,
		pub gas_limit: u64,
		pub fee: u128,
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
		pub fn send_as_sovereign(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			message: Box<VersionedXcm<()>>,
		) -> DispatchResult {
			ensure_root(origin)?;
			let dest = Location::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
			let message: Xcm<()> = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;

			pallet_xcm::Pallet::<T>::send_xcm(Here, dest.clone(), message.clone()).map_err(
				|e| match e {
					SendError::Unroutable => Error::<T>::Unreachable,
					_ => Error::<T>::SendFailure,
				},
			)?;
			Self::deposit_event(Event::Sent { to: dest, message });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(100_000_000, 0))]
		pub fn transact_to_ethereum(
			origin: OriginFor<T>,
			agent_id: H256,
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

			let transact = TransactInfo { agent_id, target, call, gas_limit, fee };

			let inner_message = Xcm(vec![
				Transact {
					origin_kind: OriginKind::SovereignAccount,
					require_weight_at_most: Weight::default(),
					call: transact.encode().into(),
				},
				SetTopic([0; 32]),
			]);

			let (ticket, price) =
				validate_send::<<T as Config>::XcmRouter>(dest.clone(), inner_message.clone())
					.map_err(|_| Error::<T>::InvalidMsg)?;
			ensure!(price.len() > 0, Error::<T>::FeesNotMet);

			let mut fees: Assets = (Parent, fee).into();
			fees.push(price.get(0).unwrap().clone());

			let origin = Location::from(AccountId32 { network: None, id: who.into() });

			<T as Config>::XcmExecutor::charge_fees(origin, fees.clone().into())
				.map_err(|_| Error::<T>::FeesNotMet)?;

			let message_id =
				<T as Config>::XcmRouter::deliver(ticket).map_err(|_| Error::<T>::SendFailure)?;

			Self::deposit_event(Event::SentExportMessage {
				message_id,
				sender_cost: fees.into(),
				message: inner_message,
			});
			Ok(())
		}
	}
}
