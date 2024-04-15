// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

use snowbridge_core::inbound::{EstimateInboundFeeError, Log};
use xcm::VersionedAssets;

sp_api::decl_runtime_apis! {
	pub trait InboundQueueApi
	{
		/// Calculate the delivery fee for `command`
		fn calculate_fee(log: Log) -> Result<VersionedAssets, EstimateInboundFeeError>;
	}
}
