// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

use asset_hub_rococo_runtime::Runtime;
pub use asset_hub_rococo_runtime::RuntimeCall;
use codec::Encode;
use frame_support::instances::Instance2;
use parachains_common::rococo::snowbridge::CreateAssetCall;
use snowbridge_router_primitives::inbound::CreateAssetCallInfo;
use sp_runtime::MultiAddress;
use xcm::latest::prelude::*;

#[test]
fn test_foreign_create_asset_call_compatibility() {
	let call = &RuntimeCall::ForeignAssets(pallet_assets::Call::create {
		id: MultiLocation::default(),
		admin: MultiAddress::Id([0; 32].into()),
		min_balance: 1,
	})
	.encode();
	let call_index = &call[..2];
	let snowbridge_call: CreateAssetCallInfo = CreateAssetCall::get();
	assert_eq!(call_index, snowbridge_call.call_index);
}

#[test]
fn check_foreign_create_asset_call_with_sane_weight() {
	use pallet_assets::WeightInfo;
	let actual = <Runtime as pallet_assets::Config<Instance2>>::WeightInfo::create();
	let snowbridge_call: CreateAssetCallInfo = CreateAssetCall::get();
	let max_weight = snowbridge_call.transact_weight_at_most;
	assert!(
		actual.all_lte(max_weight),
		"max_weight: {:?} should be adjusted to actual {:?}",
		max_weight,
		actual
	);
}
