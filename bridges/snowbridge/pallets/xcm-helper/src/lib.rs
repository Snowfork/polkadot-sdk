// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{GetDispatchInfo, PostDispatchInfo},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use sp_core::H160;
	use sp_runtime::traits::Dispatchable;
	use sp_std::{boxed::Box, vec, vec::Vec};
	use xcm::prelude::*;
	use xcm_executor::traits::{TransferType, WeightBounds, XcmAssetTransfers};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The ExecuteXcmOrigin type.
		type ExecuteXcmOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = Location,
		>;

		/// The XcmRouter type.
		type XcmRouter: SendXcm;

		/// The XcmExecutor type.
		type XcmExecutor: ExecuteXcm<<Self as Config>::RuntimeCall> + XcmAssetTransfers;

		/// The runtime `Origin` type.
		type RuntimeOrigin: From<Origin> + From<<Self as frame_system::Config>::RuntimeOrigin>;

		/// The runtime `Call` type.
		type RuntimeCall: Parameter
			+ GetDispatchInfo
			+ Dispatchable<
				RuntimeOrigin = <Self as Config>::RuntimeOrigin,
				PostInfo = PostDispatchInfo,
			>;

		type Weigher: WeightBounds<<Self as Config>::RuntimeCall>;

		type UniversalLocation: Get<InteriorLocation>;

		type Destination: Get<Location>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Fees were paid from a location for an operation (often for using `SendXcm`).
		FeesPaid { paying: Location, fees: Assets },
		/// Execution of an XCM message was attempted.
		Attempted { outcome: Outcome },
		/// A XCM message was sent.
		Sent { origin: Location, destination: Location, message: Xcm<()>, message_id: XcmHash },
	}

	#[pallet::origin]
	#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Origin {
		/// It comes from somewhere in the XCM space wanting to transact.
		Xcm(Location),
	}
	impl From<Location> for Origin {
		fn from(location: Location) -> Origin {
			Origin::Xcm(location)
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidXcm,
		SendFailure,
		BadVersion,
		Empty,
		CannotReanchor,
		CannotDetermine,
		InvalidAsset,
		FeesNotMet,
		UnweighableMessage,
		LocalExecutionIncomplete,
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
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(100_000_000, 0))]
		pub fn transact_to_ethereum(
			origin: OriginFor<T>,
			target: H160,
			call: Vec<u8>,
			gas_limit: u64,
		) -> DispatchResult {
			let origin = T::ExecuteXcmOrigin::ensure_origin(origin)?;

			let dest = T::Destination::get();

			// construct the inner xcm of ExportMessage
			let transact = TransactInfo { target, call, gas_limit };
			let mut message = Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::default(),
				call: transact.encode().into(),
			}]);
			let message_clone = message.clone();
			// Add SetTopic for tracing
			let _ = &message
				.inner_mut()
				.push(SetTopic(message_clone.using_encoded(sp_io::hashing::blake2_256)));

			Self::send_xcm(origin, dest, message, None)?;

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(100_000_000, 0))]
		pub fn transfer_to_ethereum(
			origin: OriginFor<T>,
			beneficiary: H160,
			asset: Box<VersionedAsset>,
			fee: Box<VersionedAsset>,
		) -> DispatchResult {
			let origin = T::ExecuteXcmOrigin::ensure_origin(origin)?;

			let beneficiary: Location = Location::new(
				0,
				[Junction::AccountKey20 { network: None, key: beneficiary.into() }],
			);

			let dest = T::Destination::get();

			let asset: Asset = (*asset).try_into().map_err(|()| Error::<T>::BadVersion)?;
			let fee: Asset = (*fee).try_into().map_err(|()| Error::<T>::BadVersion)?;

			if let Fungible(x) = asset.fun {
				// If fungible asset, ensure non-zero amount.
				ensure!(x > 0, Error::<T>::Empty);
			}

			// Find transfer types for fee asset.
			let asset_transfer_type = T::XcmExecutor::determine_for(&asset, &dest)
				.map_err(|_| Error::<T>::CannotDetermine)?;

			log::debug!(
				target: "xcm::transfer_to_ethereum",
				"origin {:?}, dest {:?}, beneficiary {:?}, asset {:?}, fee {:?}, transfer_type {:?}",
				origin, dest, beneficiary, asset, fee, asset_transfer_type
			);

			let (local_xcm, remote_xcm) = Self::build_xcm_transfer(
				origin.clone(),
				dest.clone(),
				beneficiary,
				&asset,
				asset_transfer_type,
				&fee,
			)?;

			Self::execute_xcm_transfer(origin, dest, local_xcm, remote_xcm, fee)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Withdraw given `assets` from the given `location` and pay as XCM fees.
		///
		/// Fails if:
		/// - the `assets` are not known on this chain;
		/// - the `assets` cannot be withdrawn with that location as the Origin.
		fn charge_fees(location: Location, assets: Assets) -> DispatchResult {
			T::XcmExecutor::charge_fees(location.clone(), assets.clone())
				.map_err(|_| Error::<T>::FeesNotMet)?;
			Self::deposit_event(Event::FeesPaid { paying: location, fees: assets });
			Ok(())
		}

		fn send_xcm(
			origin: Location,
			dest: Location,
			remote_xcm: Xcm<()>,
			fee: Option<Asset>,
		) -> DispatchResult {
			let (ticket, delivery_fee) =
				validate_send::<T::XcmRouter>(dest.clone(), remote_xcm.clone())
					.map_err(|_| Error::<T>::InvalidXcm)?;
			Self::charge_fees(origin.clone(), delivery_fee).map_err(|_| Error::<T>::FeesNotMet)?;

			if let Some(execution_fee) = fee {
				Self::charge_fees(origin.clone(), execution_fee.into())
					.map_err(|_| Error::<T>::FeesNotMet)?;
			}

			let message_id = T::XcmRouter::deliver(ticket).map_err(|_| Error::<T>::SendFailure)?;
			Self::deposit_event(Event::Sent {
				origin,
				destination: dest,
				message: remote_xcm,
				message_id,
			});
			Ok(())
		}

		fn execute_xcm_transfer(
			origin: Location,
			dest: Location,
			mut local_xcm: Xcm<<T as Config>::RuntimeCall>,
			remote_xcm: Xcm<()>,
			fee: Asset,
		) -> DispatchResult {
			log::debug!(
				target: "xcm::transfer_to_ethereum",
				"origin {:?}, dest {:?}, local_xcm {:?}, remote_xcm {:?}",
				origin, dest, local_xcm, remote_xcm,
			);

			let weight =
				T::Weigher::weight(&mut local_xcm).map_err(|()| Error::<T>::UnweighableMessage)?;
			let mut hash = local_xcm.using_encoded(sp_io::hashing::blake2_256);
			let outcome = T::XcmExecutor::prepare_and_execute(
				origin.clone(),
				local_xcm,
				&mut hash,
				weight,
				weight,
			);
			Self::deposit_event(Event::Attempted { outcome: outcome.clone() });
			outcome.ensure_complete().map_err(|_| Error::<T>::LocalExecutionIncomplete)?;

			Self::send_xcm(origin, dest, remote_xcm, Some(fee))?;

			Ok(())
		}

		fn build_xcm_transfer(
			origin: Location,
			dest: Location,
			beneficiary: Location,
			asset: &Asset,
			transfer_type: TransferType,
			fee: &Asset,
		) -> Result<(Xcm<<T as Config>::RuntimeCall>, Xcm<()>), Error<T>> {
			let (local, remote) = match transfer_type {
				TransferType::LocalReserve => {
					let (local, remote) = Self::local_reserve_transfer_programs(
						origin.clone(),
						dest.clone(),
						beneficiary,
						asset,
						fee,
					)?;
					Some((local, remote))
				},
				TransferType::DestinationReserve => {
					let (local, remote) = Self::destination_reserve_transfer_programs(
						origin.clone(),
						dest.clone(),
						beneficiary,
						asset,
						fee,
					)?;
					Some((local, remote))
				},
				_ => None,
			}
			.ok_or(Error::InvalidAsset)?;
			Ok((local, remote))
		}

		fn local_reserve_transfer_programs(
			_origin: Location,
			dest: Location,
			beneficiary: Location,
			asset: &Asset,
			fee: &Asset,
		) -> Result<(Xcm<<T as Config>::RuntimeCall>, Xcm<()>), Error<T>> {
			let assets: Assets = asset.clone().into();
			let context = T::UniversalLocation::get();

			let mut reanchored_assets = assets.clone();
			reanchored_assets
				.reanchor(&dest, &context)
				.map_err(|_| Error::<T>::CannotReanchor)?;

			let mut reanchored_fee = fee.clone();
			reanchored_fee = reanchored_fee
				.reanchored(&dest, &context)
				.map_err(|_| Error::<T>::CannotReanchor)?;

			// XCM instructions to be executed on local chain
			let local_execute_xcm = Xcm(vec![
				// locally move `assets` to `dest`s local sovereign account
				TransferAsset { assets, beneficiary: dest.clone() },
			]);
			// XCM instructions to be executed on bridge hub
			let xcm_on_dest = Xcm(vec![
				// let (dest) chain know assets are in its SA on reserve
				ReserveAssetDeposited(reanchored_assets),
				// following instructions are not exec'ed on behalf of origin chain anymore
				ClearOrigin,
				BuyExecution { fees: reanchored_fee, weight_limit: Unlimited },
				DepositAsset { assets: Wild(AllCounted(1)), beneficiary },
			]);

			Ok((local_execute_xcm, xcm_on_dest))
		}

		fn destination_reserve_transfer_programs(
			_origin: Location,
			dest: Location,
			beneficiary: Location,
			asset: &Asset,
			fee: &Asset,
		) -> Result<(Xcm<<T as Config>::RuntimeCall>, Xcm<()>), Error<T>> {
			let assets: Assets = asset.clone().into();
			let context = T::UniversalLocation::get();

			let mut reanchored_assets = assets.clone();
			reanchored_assets
				.reanchor(&dest, &context)
				.map_err(|_| Error::<T>::CannotReanchor)?;

			let mut reanchored_fee = fee.clone();
			reanchored_fee = reanchored_fee
				.reanchored(&dest, &context)
				.map_err(|_| Error::<T>::CannotReanchor)?;

			// XCM instructions to be executed on local chain
			let local_execute_xcm = Xcm(vec![
				// withdraw reserve-based assets
				WithdrawAsset(assets.clone()),
				// burn reserve-based assets
				BurnAsset(assets),
			]);
			// XCM instructions to be executed on bridge hub
			let xcm_on_dest = Xcm(vec![
				// withdraw `assets` from origin chain's sovereign account
				WithdrawAsset(reanchored_assets),
				// following instructions are not exec'ed on behalf of origin chain anymore
				ClearOrigin,
				BuyExecution { fees: reanchored_fee, weight_limit: Unlimited },
				DepositAsset { assets: Wild(AllCounted(1)), beneficiary },
			]);

			Ok((local_execute_xcm, xcm_on_dest))
		}
	}
}
