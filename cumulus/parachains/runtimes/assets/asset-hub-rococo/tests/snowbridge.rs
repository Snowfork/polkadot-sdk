// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

pub use asset_hub_rococo_runtime::{Runtime, RuntimeCall};
use codec::Encode;
use frame_support::instances::Instance2;
use sp_runtime::MultiAddress;
use xcm::latest::prelude::*;

#[test]
fn test_foreign_create_asset_call_compatibility() {
	assert_eq!(
		RuntimeCall::ForeignAssets(pallet_assets::Call::create {
			id: MultiLocation::default(),
			admin: MultiAddress::Id([0; 32].into()),
			min_balance: 1,
		})
		.encode(),
		snowbridge_router_primitives::inbound::Call::ForeignAssets(
			snowbridge_router_primitives::inbound::ForeignAssetsCall::create {
				id: MultiLocation::default(),
				admin: MultiAddress::Id([0; 32].into()),
				min_balance: 1,
			}
		)
		.encode()
	);
}

#[test]
fn check_foreign_create_asset_call_with_sane_weight() {
	use pallet_assets::WeightInfo;
	let actual = <Runtime as pallet_assets::Config<Instance2>>::WeightInfo::create();
	let max_weight = snowbridge_router_primitives::inbound::FOREIGN_CREATE_ASSET_WEIGHT_AT_MOST;
	assert!(
		actual.all_lte(max_weight),
		"max_weight: {:?} should be adjusted to actual {:?}",
		max_weight,
		actual
	);
}
