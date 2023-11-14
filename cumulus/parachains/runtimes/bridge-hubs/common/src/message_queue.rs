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
use cumulus_primitives_core::AggregateMessageOrigin;
use frame_support::{
	traits::{ProcessMessage, ProcessMessageError},
	weights::WeightMeter,
};
use sp_std::marker::PhantomData;

/// Routes messages to either the XCMP or Snowbridge processor.
pub struct BridgeHubMessageRouter<XcmpProcessor, SnowbridgeProcessor>(
	PhantomData<(XcmpProcessor, SnowbridgeProcessor)>,
);

impl<XcmpProcessor, SnowbridgeProcessor> ProcessMessage
	for BridgeHubMessageRouter<XcmpProcessor, SnowbridgeProcessor>
where
	XcmpProcessor: ProcessMessage<Origin = AggregateMessageOrigin>,
	SnowbridgeProcessor: ProcessMessage<Origin = AggregateMessageOrigin>,
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
			Here | Parent | Sibling(_) =>
				XcmpProcessor::process_message(message, origin, meter, id),
			GeneralKey(_) => SnowbridgeProcessor::process_message(message, origin, meter, id),
		}
	}
}
