// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(test)]

use bridge_hub_rococo_runtime::{
	xcm_config::XcmConfig, MessageQueueServiceWeight, Runtime, RuntimeEvent, SessionKeys,
};
use codec::Decode;
use cumulus_primitives_core::XcmError::{FailedToTransactAsset, FeesNotMet, NotHoldingFees};
use frame_support::{parameter_types, traits::fungible::Inspect};
use parachains_common::{AccountId, AuraId, Balance};
use snowbridge_core::{PricingParameters, Rewards};
use snowbridge_pallet_ethereum_client::WeightInfo;
use sp_core::H160;
use sp_keyring::AccountKeyring::Alice;
use sp_runtime::FixedU128;

type TokenBalanceOf<Runtime> = <<Runtime as snowbridge_pallet_system::Config>::Token as Inspect<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

parameter_types! {
		pub const DefaultBridgeHubEthereumBaseFee: Balance = 2_750_872_500_000;
		pub const InsufficientBridgeHubEthereumBaseFee: Balance = 1_000_000_000;
}

fn collator_session_keys() -> bridge_hub_test_utils::CollatorSessionKeys<Runtime> {
	bridge_hub_test_utils::CollatorSessionKeys::new(
		AccountId::from(Alice),
		AccountId::from(Alice),
		SessionKeys { aura: AuraId::from(Alice.public()) },
	)
}

#[test]
pub fn transfer_token_to_ethereum_works() {
	snowbridge_runtime_test_common::send_transfer_token_message_success::<Runtime, XcmConfig>(
		collator_session_keys(),
		1013,
		1000,
		H160::random(),
		H160::random(),
		DefaultBridgeHubEthereumBaseFee::get(),
		Box::new(|runtime_event_encoded: Vec<u8>| {
			match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
				Ok(RuntimeEvent::EthereumOutboundQueue(event)) => Some(event),
				_ => None,
			}
		}),
	)
}

#[test]
pub fn unpaid_transfer_token_to_ethereum_fails_with_barrier() {
	snowbridge_runtime_test_common::send_unpaid_transfer_token_message::<Runtime, XcmConfig>(
		collator_session_keys(),
		1013,
		1000,
		H160::random(),
		H160::random(),
	)
}

#[test]
pub fn transfer_token_to_ethereum_not_holding_fees() {
	snowbridge_runtime_test_common::send_transfer_token_message_failure::<Runtime, XcmConfig>(
		collator_session_keys(),
		1013,
		1000,
		DefaultBridgeHubEthereumBaseFee::get() + 1_000_000_000,
		H160::random(),
		H160::random(),
		// fee not enough
		InsufficientBridgeHubEthereumBaseFee::get(),
		NotHoldingFees,
	)
}

#[test]
pub fn transfer_token_to_ethereum_failed_to_transact_asset() {
	snowbridge_runtime_test_common::send_transfer_token_message_failure::<Runtime, XcmConfig>(
		collator_session_keys(),
		1013,
		1000,
		// initial fund not enough
		InsufficientBridgeHubEthereumBaseFee::get(),
		H160::random(),
		H160::random(),
		DefaultBridgeHubEthereumBaseFee::get(),
		FailedToTransactAsset("InsufficientBalance"),
	)
}

#[test]
fn max_message_queue_service_weight_is_more_than_beacon_extrinsic_weights() {
	let max_message_queue_weight = MessageQueueServiceWeight::get();
	let force_checkpoint =
		<Runtime as snowbridge_pallet_ethereum_client::Config>::WeightInfo::force_checkpoint();
	let submit_checkpoint =
		<Runtime as snowbridge_pallet_ethereum_client::Config>::WeightInfo::submit();
	max_message_queue_weight.all_gt(force_checkpoint);
	max_message_queue_weight.all_gt(submit_checkpoint);
}

#[test]
pub fn transfer_token_to_ethereum_fees_not_met() {
	let illegal_params: PricingParameters<TokenBalanceOf<Runtime>> = PricingParameters {
		exchange_rate: FixedU128::from_rational(1, 1),
		fee_per_gas: 1_u32.into(),
		rewards: Rewards { local: 1_u32.into(), remote: 1_u32.into() },
	};
	snowbridge_runtime_test_common::send_transfer_token_message_failure_with_invalid_fee_params::<
		Runtime,
		XcmConfig,
	>(
		collator_session_keys(),
		1013,
		1000,
		DefaultBridgeHubEthereumBaseFee::get() + 1_000_000_000,
		H160::random(),
		H160::random(),
		// fee not enough
		InsufficientBridgeHubEthereumBaseFee::get(),
		illegal_params,
		FeesNotMet,
	)
}
