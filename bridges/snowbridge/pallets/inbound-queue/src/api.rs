// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

use crate::{envelope::Envelope, Config};
use codec::DecodeAll;
use snowbridge_core::{
	inbound::{EstimateInboundFeeError, Log},
	StaticLookup,
};
use snowbridge_router_primitives::inbound::{ConvertMessage, VersionedMessage};
use sp_core::Get;
use sp_runtime::SaturatedConversion;
use xcm::{
	prelude::{Location, Parachain, SendXcm},
	v4::Asset,
	VersionedAssets,
};

pub fn calculate_fee<T>(event_log: Log) -> Result<VersionedAssets, EstimateInboundFeeError>
where
	T: Config,
{
	let envelope =
		Envelope::try_from(&event_log).map_err(|_| EstimateInboundFeeError::InvalidEnvelope)?;
	let versioned_message = VersionedMessage::decode_all(&mut envelope.payload.as_ref())
		.map_err(|_| EstimateInboundFeeError::InvalidPayload)?;
	let channel = T::ChannelLookup::lookup(envelope.channel_id)
		.ok_or(EstimateInboundFeeError::InvalidChannel)?;
	let (xcm, _) = T::MessageConverter::convert(envelope.message_id, versioned_message)
		.map_err(|_| EstimateInboundFeeError::VersionedConversionFailed)?;

	let dest = Location::new(1, [Parachain(channel.para_id.into())]);
	let (_, mut send_cost) = T::XcmSender::validate(&mut Some(dest), &mut Some(xcm))
		.map_err(|_| EstimateInboundFeeError::Unroutable)?;

	let delivery_amount = crate::Pallet::<T>::calculate_delivery_cost(T::MaxMessageSize::get())
		.saturated_into::<u128>();
	let delivery_cost = Asset::from((Location::parent(), delivery_amount));

	send_cost.push(delivery_cost);

	Ok(VersionedAssets::from(send_cost))
}
