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

extern crate alloc;
use alloc::sync::Arc;
use bp_polkadot_core::Signature;
use bridge_hub_rococo_runtime::{
	bridge_to_bulletin_config::OnBridgeHubRococoRefundRococoBulletinMessages,
	bridge_to_westend_config::OnBridgeHubRococoRefundBridgeHubWestendMessages,
	xcm_config::XcmConfig, AllPalletsWithoutSystem, BridgeRejectObsoleteHeadersAndMessages,
	Executive, MessageQueueServiceWeight, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
	SessionKeys, SignedExtra, UncheckedExtrinsic,
};
use bridge_hub_test_utils::ExtBuilder;
use codec::{Decode, DecodeAll, Encode};
use cumulus_primitives_core::XcmError::{FailedToTransactAsset, NotHoldingFees};
use frame_support::{assert_err, assert_ok, parameter_types, storage::generator::StorageMap};
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Balance};
use snowbridge_core::{inbound::Log, ChannelId, ParaId};
use snowbridge_pallet_ethereum_client::WeightInfo;
use snowbridge_pallet_inbound_queue::{Error, SendError};
use snowbridge_router_primitives::inbound;
use sp_core::H160;
use sp_io::misc::print_hex;
use sp_keyring::AccountKeyring::Alice;
use sp_runtime::{
	generic::{Era, SignedPayload},
	AccountId32 as RuntimeAccountId32,
};
use xcm::prelude::{
	Junctions::{X1, X2},
	*,
};

parameter_types! {
		pub const DefaultBridgeHubEthereumBaseFee: Balance = 2_750_872_500_000;
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
		11155111,
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
		11155111,
		collator_session_keys(),
		1013,
		1000,
		H160::random(),
		H160::random(),
	)
}

#[test]
pub fn transfer_token_to_ethereum_fee_not_enough() {
	snowbridge_runtime_test_common::send_transfer_token_message_failure::<Runtime, XcmConfig>(
		11155111,
		collator_session_keys(),
		1013,
		1000,
		DefaultBridgeHubEthereumBaseFee::get() + 1_000_000_000,
		H160::random(),
		H160::random(),
		// fee not enough
		1_000_000_000,
		NotHoldingFees,
	)
}

#[test]
pub fn transfer_token_to_ethereum_insufficient_fund() {
	snowbridge_runtime_test_common::send_transfer_token_message_failure::<Runtime, XcmConfig>(
		11155111,
		collator_session_keys(),
		1013,
		1000,
		1_000_000_000,
		H160::random(),
		H160::random(),
		DefaultBridgeHubEthereumBaseFee::get(),
		FailedToTransactAsset("Funds are unavailable"),
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
fn ethereum_client_consensus_extrinsics_work() {
	snowbridge_runtime_test_common::ethereum_extrinsic(
		collator_session_keys(),
		1013,
		construct_and_apply_extrinsic,
	);
}

#[test]
fn ethereum_to_polkadot_message_extrinsics_work() {
	snowbridge_runtime_test_common::ethereum_to_polkadot_message_extrinsics_work(
		collator_session_keys(),
		1013,
		construct_and_apply_extrinsic,
	);
}

/// Tests that the digest items are as expected when a Ethereum Outbound message is received.
/// If the MessageQueue pallet is configured before (i.e. the MessageQueue pallet is listed before
/// the EthereumOutboundQueue in the construct_runtime macro) the EthereumOutboundQueue, this test
/// will fail.
#[test]
pub fn ethereum_outbound_queue_processes_messages_before_message_queue_works() {
	snowbridge_runtime_test_common::ethereum_outbound_queue_processes_messages_before_message_queue_works::<
		Runtime,
		XcmConfig,
		AllPalletsWithoutSystem,
	>(
		11155111,
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

fn construct_extrinsic(
	sender: sp_keyring::AccountKeyring,
	call: RuntimeCall,
) -> UncheckedExtrinsic {
	let account_id = RuntimeAccountId32::from(sender.public());
	let extra: SignedExtra = (
		frame_system::CheckNonZeroSender::<Runtime>::new(),
		frame_system::CheckSpecVersion::<Runtime>::new(),
		frame_system::CheckTxVersion::<Runtime>::new(),
		frame_system::CheckGenesis::<Runtime>::new(),
		frame_system::CheckEra::<Runtime>::from(Era::immortal()),
		frame_system::CheckNonce::<Runtime>::from(
			frame_system::Pallet::<Runtime>::account(&account_id).nonce,
		),
		frame_system::CheckWeight::<Runtime>::new(),
		pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
		BridgeRejectObsoleteHeadersAndMessages::default(),
		(
			OnBridgeHubRococoRefundBridgeHubWestendMessages::default(),
			OnBridgeHubRococoRefundRococoBulletinMessages::default(),
		),
	);
	let payload = SignedPayload::new(call.clone(), extra.clone()).unwrap();
	let signature = payload.using_encoded(|e| sender.sign(e));
	UncheckedExtrinsic::new_signed(
		call,
		account_id.into(),
		Signature::Sr25519(signature.clone()),
		extra,
	)
}

fn construct_and_apply_extrinsic(
	origin: sp_keyring::AccountKeyring,
	call: RuntimeCall,
) -> sp_runtime::DispatchOutcome {
	let xt = construct_extrinsic(origin, call);
	let r = Executive::apply_extrinsic(xt);
	r.unwrap()
}

// The event log from https://bridgehub-rococo.stg.subscan.io/extrinsic/2537445-2
#[test]
pub fn send_token_to_foreign_chain() {
	ExtBuilder::<Runtime>::default()
		.with_collators(collator_session_keys().collators())
		.with_session_keys(collator_session_keys().session_keys())
		.with_para_id(1013.into())
		.with_tracing()
		.build()
		.execute_with(|| {
			let _ = <pallet_xcm::Pallet<Runtime>>::force_xcm_version(
				RuntimeOrigin::root(),
				Box::new(Location::new(1, [Parachain(1000)])),
				XCM_VERSION,
			).unwrap();
			let event_log = Log {
				address: hex!("5b4909ce6ca82d2ce23bd46738953c7959e710cd").into(),
				topics: vec![
					hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
					hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
					hex!("e1f34efbe357e26b778c73724e9a64777d95cf5b54e146de8e8ea9a78cf7cda7").into(),
				],
				data: hex!("00000000000000000000000000000000000000000000000000000000000000170000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000005f00a736aa000000000001fff9976782d46cc05630d1f6ebab18b2324d6b1400f4befb32bffac93a1e45336e81dd6498485f9b13935ab108db7afe0bbfa71c6800f2052a01000000000000000000000000e40b5402000000000000000000000000").into(),
			};
			let envelope =
				snowbridge_pallet_inbound_queue::Envelope::try_from(&event_log).map_err(|_| Error::<Runtime>::InvalidEnvelope).unwrap();
			let message = inbound::VersionedMessage::decode_all(&mut envelope.payload.as_ref()).map_err(|_| Error::<Runtime>::InvalidPayload).unwrap();
			let (xcm,fee) = <snowbridge_pallet_inbound_queue::Pallet<Runtime>>::do_convert(envelope.message_id, message).unwrap();
			assert_eq!(xcm.len(),8);
			assert_eq!(fee,10000000000);
			println!("xcm converted as {:?} and fee is {:?}.", xcm, fee);
		});
}

#[test]
pub fn generate_nonce_key() {
	ExtBuilder::<Runtime>::default()
		.with_collators(collator_session_keys().collators())
		.with_session_keys(collator_session_keys().session_keys())
		.with_para_id(1013.into())
		.with_tracing()
		.build()
		.execute_with(|| {
			let para_id: ParaId = 1000.into();
			let channel_id: ChannelId = para_id.into();
			let inbound_queue_nonce_key =
				snowbridge_pallet_inbound_queue::Nonce::<Runtime>::storage_map_final_key(
					channel_id,
				);
			print_hex(inbound_queue_nonce_key.as_slice());
			assert_eq!(inbound_queue_nonce_key,hex!("7d7c8b03a2a182824cfe569187a28faa718368a0ace36e2b1b8b6dbd7f8093c0594aa8a9c557dabac173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539"));
			let inbound_queue_nonce_value = 30_u64.encode();
			print_hex(inbound_queue_nonce_value.as_slice());
			let items = vec![(inbound_queue_nonce_key, inbound_queue_nonce_value)];
			let set_storage_call =
				RuntimeCall::System(frame_system::Call::set_storage { items }).encode();
			print_hex(set_storage_call.as_slice());
			assert_eq!(set_storage_call,hex!("00040421017d7c8b03a2a182824cfe569187a28faa718368a0ace36e2b1b8b6dbd7f8093c0594aa8a9c557dabac173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539201e00000000000000"));
		});
}

#[test]
pub fn trigger_exceeds_max_message_size() {
	ExtBuilder::<Runtime>::default()
		.with_collators(collator_session_keys().collators())
		.with_session_keys(collator_session_keys().session_keys())
		.with_para_id(1013.into())
		.with_tracing()
		.build()
		.execute_with(|| {
			assert_ok!(<pallet_xcm::Pallet<Runtime>>::force_xcm_version(
				RuntimeOrigin::root(),
				Box::new(Location::new(1, [Parachain(1000)])),
				XCM_VERSION,
			));
			let xcm = Xcm::<()>(vec![
				ReceiveTeleportedAsset(Assets::from(vec![Asset {
					id: AssetId(Location { parents: 1, interior: Here }),
					fun: Fungible(10000000000),
				}])),
				BuyExecution {
					fees: Asset {
						id: AssetId(Location { parents: 1, interior: Here }),
						fun: Fungible(10000000000),
					},
					weight_limit: Unlimited,
				},
				DescendOrigin(X1([PalletInstance(80)].into())),
				UniversalOrigin(GlobalConsensus(Ethereum { chain_id: 11155111 })),
				ReserveAssetDeposited(Assets::from(vec![Asset {
					id: AssetId(Location {
						parents: 2,
						interior: X2(Arc::from([
							GlobalConsensus(Ethereum { chain_id: 11155111 }),
							AccountKey20 {
								network: None,
								key: [
									255, 249, 151, 103, 130, 212, 108, 192, 86, 48, 209, 246, 235,
									171, 24, 178, 50, 77, 107, 20,
								],
							},
						])),
					}),
					fun: Fungible(30000000000),
				}])),
				ClearOrigin,
				DepositReserveAsset {
					assets: Definite(Assets::from(vec![
						Asset {
							id: AssetId(Location { parents: 1, interior: Here }),
							fun: Fungible(0),
						},
						Asset {
							id: AssetId(Location {
								parents: 2,
								interior: X2(Arc::from([
									GlobalConsensus(Ethereum { chain_id: 11155111 }),
									AccountKey20 {
										network: None,
										key: [
											255, 249, 151, 103, 130, 212, 108, 192, 86, 48, 209,
											246, 235, 171, 24, 178, 50, 77, 107, 20,
										],
									},
								])),
							}),
							fun: Fungible(30000000000),
						},
					])),
					dest: Location { parents: 1, interior: X1(Arc::from([Parachain(2030)])) },
					xcm: Xcm(vec![
						BuyExecution {
							fees: Asset {
								id: AssetId(Location { parents: 1, interior: Here }),
								fun: Fungible(0),
							},
							weight_limit: Unlimited,
						},
						DepositAsset {
							assets: Definite(Assets::from(vec![Asset {
								id: AssetId(Location {
									parents: 2,
									interior: X2(Arc::from([
										GlobalConsensus(Ethereum { chain_id: 11155111 }),
										AccountKey20 {
											network: None,
											key: [
												255, 249, 151, 103, 130, 212, 108, 192, 86, 48,
												209, 246, 235, 171, 24, 178, 50, 77, 107, 20,
											],
										},
									])),
								}),
								fun: Fungible(30000000000),
							}])),
							beneficiary: Location {
								parents: 0,
								interior: X1(Arc::from([AccountId32 {
									network: None,
									id: [
										144, 68, 17, 180, 104, 211, 111, 197, 154, 116, 64, 250,
										194, 65, 112, 164, 15, 92, 116, 211, 166, 87, 28, 243, 222,
										50, 99, 0, 205, 67, 135, 74,
									],
								}])),
							},
						},
					]),
				},
				SetTopic([
					225, 243, 78, 251, 227, 87, 226, 107, 119, 140, 115, 114, 78, 154, 100, 119,
					125, 149, 207, 91, 84, 225, 70, 222, 142, 142, 169, 167, 140, 247, 205, 167,
				]),
			]);
			let result =
				<snowbridge_pallet_inbound_queue::Pallet<Runtime>>::send_xcm(xcm, 1000.into());
			assert_err!(result, <Error<Runtime>>::Send(SendError::ExceedsMaxMessageSize));
		});
}
