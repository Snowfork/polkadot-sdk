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
//! Runtime configuration for MessageQueue pallet
use sp_std::{prelude::*, marker::PhantomData};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{weights::WeightMeter, traits::{ProcessMessage, ProcessMessageError}};
use scale_info::TypeInfo;
use cumulus_primitives_core::ParaId;
use xcm::v3::{MultiLocation, Junction};

/// The aggregate origin of an inbound message.
/// This is specialized for BridgeHub, as the snowbridge-outbound-queue pallet is also using
/// the shared MessageQueue pallet.
#[derive(Encode, Decode, MaxEncodedLen, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub enum AggregateMessageOrigin {
	/// The message came from the para-chain itself.
	Here,
	/// The message came from the relay-chain.
	///
	/// This is used by the DMP queue.
	Parent,
	/// The message came from a sibling para-chain.
	///
	/// This is used by the HRMP queue.
	Sibling(ParaId),
	Snowbridge(SnowbridgeMessageOrigin),
}

/// The origin of an inbound message for Snowbridge.
#[derive(Encode, Decode, MaxEncodedLen, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub enum SnowbridgeMessageOrigin {
	/// The message came from the para-chain itself.
	Here,
	/// The message came from a sibling para-chain.
	Sibling(ParaId),
}

impl From<AggregateMessageOrigin> for MultiLocation {
	fn from(origin: AggregateMessageOrigin) -> Self {
		use AggregateMessageOrigin::*;
		match origin {
			Here => MultiLocation::here(),
			Parent => MultiLocation::parent(),
			Sibling(id) =>
				MultiLocation::new(1, Junction::Parachain(id.into())),
			// NOTE: We don't need this conversion for Snowbridge. However we have to
			// implement it anyway as xcm_builder::ProcessXcmMessage requires it.
			Snowbridge(_) => MultiLocation::default(),
		}
	}
}

#[cfg(feature = "runtime-benchmarks")]
impl From<u32> for AggregateMessageOrigin {
	fn from(x: u32) -> Self {
		match x {
			0 => Self::Here,
			1 => Self::Parent,
			p => Self::Sibling(ParaId::from(p)),
		}
	}
}

/// Routes messages to either the XCMP or Snowbridge processor.
pub struct BridgeHubMessageRouter<XcmpProcessor, SnowbridgeProcessor>(PhantomData<(XcmpProcessor, SnowbridgeProcessor)>)
where
	XcmpProcessor: ProcessMessage<Origin = AggregateMessageOrigin>,
	SnowbridgeProcessor: ProcessMessage<Origin = AggregateMessageOrigin>;

impl<
	XcmpProcessor,
	SnowbridgeProcessor
> ProcessMessage for BridgeHubMessageRouter<
	XcmpProcessor,
	SnowbridgeProcessor
>
where
	XcmpProcessor: ProcessMessage<Origin = AggregateMessageOrigin>,
	SnowbridgeProcessor: ProcessMessage<Origin = AggregateMessageOrigin>
{
	type Origin = AggregateMessageOrigin;

	fn process_message(
		message: &[u8],
		origin: Self::Origin,
		meter: &mut WeightMeter,
		id: &mut [u8; 32],
	) -> Result<bool, ProcessMessageError> {
		use AggregateMessageOrigin::*;
		match origin {
			Here | Parent | Sibling(_) => XcmpProcessor::process_message(message, origin, meter, id),
			Snowbridge(_) => SnowbridgeProcessor::process_message(message, origin, meter, id)
		}
	}
}
