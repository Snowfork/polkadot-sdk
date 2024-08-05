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
	use frame_system::{pallet_prelude::*, unique};
	use sp_core::H160;
	use sp_runtime::traits::Dispatchable;
	use sp_std::{boxed::Box, vec};
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

		/// Means of measuring the weight consumed by an XCM message locally.
		type Weigher: WeightBounds<<Self as Config>::RuntimeCall>;

		/// Universal location of this runtime.
		type UniversalLocation: Get<InteriorLocation>;

		/// Ethereum's location of this runtime.
		type Destination: Get<Location>;

		/// DeliveryFee for the execution cost on BH
		type DeliveryFee: Get<Asset>;

		/// The location of BH
		type Forwarder: Get<Location>;
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
		InvalidNetwork,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(4_000_000_000, 0))]
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

			// If fungible asset, ensure non-zero amount.
			let asset: Asset = (*asset).try_into().map_err(|()| Error::<T>::BadVersion)?;
			if let Fungible(x) = asset.fun {
				ensure!(x > 0, Error::<T>::Empty);
			}

			// Find transfer types for fee asset.
			let asset_transfer_type = T::XcmExecutor::determine_for(&asset, &dest)
				.map_err(|_| Error::<T>::CannotDetermine)?;

			let fee: Asset = (*fee).try_into().map_err(|()| Error::<T>::BadVersion)?;
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

			Self::execute_xcm_transfer(origin, T::Forwarder::get(), local_xcm, remote_xcm)
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

		/// Send xcm to bridge hub with designated fee charged
		fn send_xcm(origin: Location, dest: Location, remote_xcm: Xcm<()>) -> DispatchResult {
			let (ticket, delivery_fee) =
				validate_send::<T::XcmRouter>(dest.clone(), remote_xcm.clone())
					.map_err(|_| Error::<T>::InvalidXcm)?;
			Self::charge_fees(origin.clone(), delivery_fee).map_err(|_| Error::<T>::FeesNotMet)?;

			let message_id = T::XcmRouter::deliver(ticket).map_err(|_| Error::<T>::SendFailure)?;
			Self::deposit_event(Event::Sent {
				origin,
				destination: dest,
				message: remote_xcm,
				message_id,
			});
			Ok(())
		}

		/// Execute the transfer including the local xcm
		/// and send the remote xcm to bridge hub
		fn execute_xcm_transfer(
			origin: Location,
			dest: Location,
			mut local_xcm: Xcm<<T as Config>::RuntimeCall>,
			remote_xcm: Xcm<()>,
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

			Self::send_xcm(origin, dest, remote_xcm)?;

			Ok(())
		}

		/// Build the Xcm, a local one and the remote one which will be sent to bridge hub
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

		/// Construct Xcm for Polkadot native asset
		fn local_reserve_transfer_programs(
			origin: Location,
			dest: Location,
			beneficiary: Location,
			asset: &Asset,
			fee: &Asset,
		) -> Result<(Xcm<<T as Config>::RuntimeCall>, Xcm<()>), Error<T>> {
			let assets: Assets = vec![asset.clone()].into();
			let burn_assets: Assets = vec![fee.clone(), T::DeliveryFee::get()].into();

			// XCM instructions to be executed on local chain
			let local_execute_xcm = Xcm(vec![
				// locally move `assets` to `dest`s local sovereign account
				TransferAsset { assets: assets.clone(), beneficiary: dest.clone() },
				// withdraw reserve-based assets
				WithdrawAsset(burn_assets.clone()),
				// burn reserve-based assets
				BurnAsset(burn_assets),
			]);

			let network: NetworkId = match T::Destination::get() {
				Location { parents: 2, interior: Junctions::X1(junction) } =>
					match junction.first() {
						Some(&GlobalConsensus(network_id)) => Ok(network_id),
						_ => Err(Error::<T>::InvalidNetwork),
					},
				_ => Err(Error::<T>::InvalidNetwork),
			}?;

			let mut inner_xcm = Xcm(vec![
				ReserveAssetDeposited(assets),
				ClearOrigin,
				BuyExecution { fees: fee.clone(), weight_limit: Unlimited },
				DepositAsset { assets: Wild(AllCounted(1)), beneficiary },
			]);
			let unique_id = unique(&inner_xcm);
			inner_xcm.0.push(SetTopic(unique_id));

			// XCM instructions to be executed on bridge hub
			let xcm_on_dest = Xcm(vec![
				DescendOrigin(origin.clone().interior),
				ReceiveTeleportedAsset(vec![T::DeliveryFee::get()].into()),
				BuyExecution { fees: T::DeliveryFee::get().into(), weight_limit: Unlimited },
				SetAppendix(Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: origin.clone(),
				}])),
				ExportMessage { network, destination: dest.interior, xcm: inner_xcm },
			]);

			Ok((local_execute_xcm, xcm_on_dest))
		}

		/// Construct Xcm for Ethereum native asset
		fn destination_reserve_transfer_programs(
			origin: Location,
			dest: Location,
			beneficiary: Location,
			asset: &Asset,
			fee: &Asset,
		) -> Result<(Xcm<<T as Config>::RuntimeCall>, Xcm<()>), Error<T>> {
			let assets: Assets = vec![asset.clone(), fee.clone(), T::DeliveryFee::get()].into();

			// XCM instructions to be executed on local chain
			let local_execute_xcm = Xcm(vec![
				// withdraw reserve-based assets
				WithdrawAsset(assets.clone()),
				// burn reserve-based assets
				BurnAsset(assets),
			]);

			let network: NetworkId = match T::Destination::get() {
				Location { parents: 2, interior: Junctions::X1(junction) } =>
					match junction.first() {
						Some(&GlobalConsensus(network_id)) => Ok(network_id),
						_ => Err(Error::<T>::InvalidNetwork),
					},
				_ => Err(Error::<T>::InvalidNetwork),
			}?;

			let mut inner_xcm = Xcm(vec![
				WithdrawAsset(vec![asset.clone()].into()),
				ClearOrigin,
				BuyExecution { fees: fee.clone(), weight_limit: Unlimited },
				DepositAsset { assets: Wild(AllCounted(1)), beneficiary },
			]);
			let unique_id = unique(&inner_xcm);
			inner_xcm.0.push(SetTopic(unique_id));

			// XCM instructions to be executed on bridge hub
			let xcm_on_dest = Xcm(vec![
				DescendOrigin(origin.clone().interior),
				ReceiveTeleportedAsset(vec![T::DeliveryFee::get()].into()),
				BuyExecution { fees: T::DeliveryFee::get().into(), weight_limit: Unlimited },
				SetAppendix(Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: origin.clone(),
				}])),
				ExportMessage { network, destination: dest.interior, xcm: inner_xcm },
			]);

			Ok((local_execute_xcm, xcm_on_dest))
		}
	}
}
