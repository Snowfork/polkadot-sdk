// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use super::*;
mod util;

use crate::{
	migration::{
		v0::{
			CompactExecutionHeader, ExecutionHeaderIndex, ExecutionHeaderMapping,
			ExecutionHeaderState, ExecutionHeaders, LatestExecutionState,
		},
		EthereumExecutionHeaderCleanup,
	},
	Pallet as EthereumBeaconClient,
};
use frame_benchmarking::v2::*;
use frame_support::{migrations::SteppedMigration, weights::WeightMeter};
use frame_system::RawOrigin;
use hex_literal::hex;

use snowbridge_pallet_ethereum_client_fixtures::*;

use snowbridge_beacon_primitives::{
	fast_aggregate_verify, prepare_aggregate_pubkey, prepare_aggregate_signature,
	verify_merkle_branch,
};
use util::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn force_checkpoint() -> Result<(), BenchmarkError> {
		let checkpoint_update = make_checkpoint();
		let block_root: H256 = checkpoint_update.header.hash_tree_root().unwrap();

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(*checkpoint_update));

		assert!(<LatestFinalizedBlockRoot<T>>::get() == block_root);
		assert!(<FinalizedBeaconState<T>>::get(block_root).is_some());

		Ok(())
	}

	#[benchmark]
	fn submit() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
		let checkpoint_update = make_checkpoint();
		let finalized_header_update = make_finalized_header_update();
		let block_root: H256 = finalized_header_update.finalized_header.hash_tree_root().unwrap();
		EthereumBeaconClient::<T>::process_checkpoint_update(&checkpoint_update)?;

		#[extrinsic_call]
		submit(RawOrigin::Signed(caller.clone()), Box::new(*finalized_header_update));

		assert!(<LatestFinalizedBlockRoot<T>>::get() == block_root);
		assert!(<FinalizedBeaconState<T>>::get(block_root).is_some());

		Ok(())
	}

	#[benchmark]
	fn submit_with_sync_committee() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();
		let checkpoint_update = make_checkpoint();
		let sync_committee_update = make_sync_committee_update();
		EthereumBeaconClient::<T>::process_checkpoint_update(&checkpoint_update)?;

		#[extrinsic_call]
		submit(RawOrigin::Signed(caller.clone()), Box::new(*sync_committee_update));

		assert!(<NextSyncCommittee<T>>::exists());

		Ok(())
	}

	#[benchmark(extra)]
	fn bls_fast_aggregate_verify_pre_aggregated() -> Result<(), BenchmarkError> {
		EthereumBeaconClient::<T>::process_checkpoint_update(&make_checkpoint())?;
		let update = make_sync_committee_update();
		let participant_pubkeys = participant_pubkeys::<T>(&update)?;
		let signing_root = signing_root::<T>(&update)?;
		let agg_sig =
			prepare_aggregate_signature(&update.sync_aggregate.sync_committee_signature).unwrap();
		let agg_pub_key = prepare_aggregate_pubkey(&participant_pubkeys).unwrap();

		#[block]
		{
			agg_sig.fast_aggregate_verify_pre_aggregated(signing_root.as_bytes(), &agg_pub_key);
		}

		Ok(())
	}

	#[benchmark(extra)]
	fn bls_fast_aggregate_verify() -> Result<(), BenchmarkError> {
		EthereumBeaconClient::<T>::process_checkpoint_update(&make_checkpoint())?;
		let update = make_sync_committee_update();
		let current_sync_committee = <CurrentSyncCommittee<T>>::get();
		let absent_pubkeys = absent_pubkeys::<T>(&update)?;
		let signing_root = signing_root::<T>(&update)?;

		#[block]
		{
			fast_aggregate_verify(
				&current_sync_committee.aggregate_pubkey,
				&absent_pubkeys,
				signing_root,
				&update.sync_aggregate.sync_committee_signature,
			)
			.unwrap();
		}

		Ok(())
	}

	#[benchmark(extra)]
	fn verify_merkle_proof() -> Result<(), BenchmarkError> {
		EthereumBeaconClient::<T>::process_checkpoint_update(&make_checkpoint())?;
		let update = make_sync_committee_update();
		let block_root: H256 = update.finalized_header.hash_tree_root().unwrap();

		#[block]
		{
			verify_merkle_branch(
				block_root,
				&update.finality_branch,
				config::FINALIZED_ROOT_SUBTREE_INDEX,
				config::FINALIZED_ROOT_DEPTH,
				update.attested_header.state_root,
			);
		}

		Ok(())
	}

	use frame_support::parameter_types;

	parameter_types! {
		pub ExecutionHeaderCount: u32 = 1;
	}

	#[benchmark]
	fn step() {
		let block_root: H256 =
			hex!("4e4ed8c829bf771f94c60caa052dc3b703b24165a2e6459350e3a43a80ab7a8f").into();
		ExecutionHeaders::<T>::insert(
			block_root,
			CompactExecutionHeader {
				parent_hash: hex!(
					"e0a5ca63886dfa16d53347ba347289e0187f7c38320768d094fc48d331ac7a23"
				)
				.into(),
				block_number: 48242,
				state_root: hex!(
					"b3f33b6950fd047b634dcea0d09f002f07431d3e6648213604e54caa822055a6"
				)
				.into(),
				receipts_root: hex!(
					"f744e1ebe846b2961a7daa3c0d9023d8b109cf9e425b9e9973f039180e487b67"
				)
				.into(),
			},
		);
		ExecutionHeaderMapping::<T>::insert(0u32, block_root);
		LatestExecutionState::<T>::set(ExecutionHeaderState {
			beacon_block_root: hex!(
				"b3f33b6950fd047b634dcea0d09f002f07431d3e6648213604e54caa822055a6"
			)
			.into(),
			beacon_slot: 5353,
			block_hash: hex!("e0a5ca63886dfa16d53347ba347289e0187f7c38320768d094fc48d331ac7a23")
				.into(),
			block_number: 5454,
		});
		ExecutionHeaderIndex::<T>::set(0);
		let mut meter = WeightMeter::new();

		#[block]
		{
			EthereumExecutionHeaderCleanup::<T, (), ExecutionHeaderCount>::step(None, &mut meter)
				.unwrap();
		}

		// Check that the header is removed
		assert_eq!(ExecutionHeaderMapping::<T>::get(0u32), H256::zero());
		assert!(ExecutionHeaders::<T>::get(block_root).is_none());
		assert!(LatestExecutionState::<T>::get().beacon_block_root == H256::zero());
		assert!(ExecutionHeaderIndex::<T>::get() == 0);
	}

	impl_benchmark_test_suite!(EthereumBeaconClient, crate::mock::new_tester(), crate::mock::Test);
}
