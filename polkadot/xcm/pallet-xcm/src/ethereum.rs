use crate::{Box, Config, Error, Event, FeesHandling, Instruction::BurnAsset, Pallet, Vec, Xcm};
use codec::Encode;
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{Contains, EnsureOrigin},
};
use frame_system::pallet_prelude::OriginFor;
use sp_runtime::traits::Zero;
use xcm::{
	latest::{validate_send, Asset, Assets, Location, WeightLimit},
	prelude::{Fungible, Here},
	v4::{ExecuteXcm, SendXcm},
	VersionedAssets, VersionedLocation,
};
use xcm_executor::traits::{TransferType, WeightBounds, XcmAssetTransfers};

impl<T: Config> Pallet<T> {
	pub fn do_reserve_transfer_ethereum_assets(
		origin: OriginFor<T>,
		dest: Box<VersionedLocation>,
		beneficiary: Box<VersionedLocation>,
		assets: Box<VersionedAssets>,
		fees: Box<VersionedAssets>,
		weight_limit: WeightLimit,
	) -> DispatchResult {
		let origin_location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		let dest = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let beneficiary: Location =
			(*beneficiary).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let assets: Assets = (*assets).try_into().map_err(|()| Error::<T>::BadVersion)?;
		log::debug!(
			target: "xcm::pallet_xcm::do_reserve_transfer_assets",
			"origin {:?}, dest {:?}, beneficiary {:?}, assets {:?}, fees {:?}",
			origin_location, dest, beneficiary, assets, fees,
		);

		let value = (origin_location, assets.into_inner());
		ensure!(T::XcmReserveTransferFilter::contains(&value), Error::<T>::Filtered);
		let (origin, assets) = value;

		let fees: Assets = (*fees).try_into().map_err(|()| Error::<T>::BadVersion)?;
		ensure!(fees.len() == 2, Error::<T>::FeesNotMet);
		let fee_on_ethereum = fees.get(1).unwrap();
		// Find transfer types for fee and non-fee assets.
		let (fees_transfer_type, assets_transfer_type) =
			Self::find_ethereum_assets_transfer_types(&assets, fee_on_ethereum, &dest)?;
		// Ensure assets (and fees according to check below) are not teleportable to `dest`.
		ensure!(assets_transfer_type != TransferType::Teleport, Error::<T>::Filtered);
		// Ensure all assets (including fees) have same reserve location.
		ensure!(assets_transfer_type == fees_transfer_type, Error::<T>::TooManyReserves);
		let (local_xcm, remote_xcm) = Self::build_ethereum_xcm_transfer(
			origin.clone(),
			dest.clone(),
			beneficiary,
			assets,
			assets_transfer_type,
			fees.clone().into_inner(),
			weight_limit,
		)?;

		Self::execute_ethereum_xcm_transfer(origin, dest, fees, local_xcm, remote_xcm)
	}

	fn find_ethereum_assets_transfer_types(
		assets: &[Asset],
		fee: &Asset,
		dest: &Location,
	) -> Result<(TransferType, TransferType), Error<T>> {
		let mut assets_transfer_type = None;
		let fee_transfer_type =
			T::XcmExecutor::determine_for(fee, dest).map_err(Error::<T>::from)?;

		for (_, asset) in assets.iter().enumerate() {
			if let Fungible(x) = asset.fun {
				// If fungible asset, ensure non-zero amount.
				ensure!(!x.is_zero(), Error::<T>::Empty);
			}
			let transfer_type =
				T::XcmExecutor::determine_for(&asset, dest).map_err(Error::<T>::from)?;
			if let Some(existing) = assets_transfer_type.as_ref() {
				// Ensure transfer for multiple assets uses same transfer type (only fee may
				// have different transfer type/path)
				ensure!(existing == &transfer_type, Error::<T>::TooManyReserves);
			} else {
				// asset reserve identified
				assets_transfer_type = Some(transfer_type);
			}
		}
		Ok((fee_transfer_type, assets_transfer_type.ok_or(Error::<T>::Empty)?))
	}

	fn build_ethereum_xcm_transfer(
		origin: Location,
		dest: Location,
		beneficiary: Location,
		assets: Vec<Asset>,
		transfer_type: TransferType,
		fees: Vec<Asset>,
		weight_limit: WeightLimit,
	) -> Result<(Xcm<<T as Config>::RuntimeCall>, Option<Xcm<()>>), Error<T>> {
		log::debug!(
			target: "xcm::pallet_xcm::build_xcm_transfer_type",
			"origin {:?}, dest {:?}, beneficiary {:?}, assets {:?}, transfer_type {:?}, \
			fees_handling {:?}, weight_limit: {:?}",
			origin, dest, beneficiary, assets, transfer_type, fees, weight_limit,
		);
		let fee_on_bridgehub = &fees[0];
		let fee_on_ethereum = &fees[1];
		let (local, mut remote) = match transfer_type {
			TransferType::LocalReserve => {
				let (local, remote) = Self::local_reserve_transfer_programs(
					origin.clone(),
					dest.clone(),
					beneficiary,
					assets,
					FeesHandling::Batched { fees: fee_on_ethereum.clone() },
					weight_limit,
				)?;
				Some((local, remote))
			},
			TransferType::DestinationReserve => {
				let (local, remote) = Self::destination_reserve_transfer_programs(
					origin.clone(),
					dest.clone(),
					beneficiary,
					assets,
					FeesHandling::Batched { fees: fee_on_ethereum.clone() },
					weight_limit,
				)?;
				Some((local, remote))
			},
			_ => None,
		}
		.ok_or(Error::InvalidAssetUnsupportedReserve)?;
		remote.inner_mut().push(BurnAsset(fee_on_bridgehub.clone().into()));
		Ok((local, Some(remote)))
	}

	fn execute_ethereum_xcm_transfer(
		origin: Location,
		dest: Location,
		fees: Assets,
		mut local_xcm: Xcm<<T as Config>::RuntimeCall>,
		remote_xcm: Option<Xcm<()>>,
	) -> DispatchResult {
		log::debug!(
			target: "xcm::pallet_xcm::execute_xcm_transfer",
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
		outcome.ensure_complete().map_err(|error| {
			log::error!(
				target: "xcm::pallet_xcm::execute_xcm_transfer",
				"XCM execution failed with error {:?}", error
			);
			Error::<T>::LocalExecutionIncomplete
		})?;

		if let Some(remote_xcm) = remote_xcm {
			let (ticket, _) = validate_send::<T::XcmRouter>(dest.clone(), remote_xcm.clone())
				.map_err(Error::<T>::from)?;
			if origin != Here.into_location() {
				Self::charge_fees(origin.clone(), fees).map_err(|error| {
					log::error!(
						target: "xcm::pallet_xcm::execute_xcm_transfer",
						"Unable to charge fee with error {:?}", error
					);
					Error::<T>::FeesNotMet
				})?;
			}
			let message_id = T::XcmRouter::deliver(ticket).map_err(Error::<T>::from)?;

			let e = Event::Sent { origin, destination: dest, message: remote_xcm, message_id };
			Self::deposit_event(e);
		}
		Ok(())
	}
}
