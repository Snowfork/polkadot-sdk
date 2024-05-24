// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use crate::{
	functions::compute_period, sync_committee_sum, verify_merkle_branch, BeaconHeader,
	CompactBeaconState, Error, FinalizedBeaconState, LatestFinalizedBlockRoot, NextSyncCommittee,
	SyncCommitteePrepared,
};

use crate::mock::{
	get_message_verification_payload, load_checkpoint_update_fixture,
	load_finalized_header_update_fixture, load_next_finalized_header_update_fixture,
	load_next_sync_committee_update_fixture, load_sync_committee_update_fixture,
};

pub use crate::mock::*;

use crate::config::{EPOCHS_PER_SYNC_COMMITTEE_PERIOD, SLOTS_PER_EPOCH, SLOTS_PER_HISTORICAL_ROOT};
use frame_support::{assert_err, assert_noop, assert_ok};
use hex_literal::hex;
use primitives::{types::deneb, Fork, ForkVersions, NextSyncCommitteeUpdate, VersionedExecutionPayloadHeader, ExecutionProof, AncestryProof};
use snowbridge_core::inbound::{Log, Proof, VerificationError, Verifier};
use snowbridge_core::U256;
use sp_core::H256;
use sp_runtime::DispatchError;

/// Arbitrary hash used for tests and invalid hashes.
const TEST_HASH: [u8; 32] =
	hex!["5f6f02af29218292d21a69b64a794a7c0873b3e0f54611972863706e8cbdf371"];

/* UNIT TESTS */

#[test]
pub fn sum_sync_committee_participation() {
	new_tester().execute_with(|| {
		assert_eq!(sync_committee_sum(&[0, 1, 0, 1, 1, 0, 1, 0, 1]), 5);
	});
}

#[test]
pub fn compute_domain() {
	new_tester().execute_with(|| {
		let domain = EthereumBeaconClient::compute_domain(
			hex!("07000000").into(),
			hex!("00000001"),
			hex!("5dec7ae03261fde20d5b024dfabce8bac3276c9a4908e23d50ba8c9b50b0adff").into(),
		);

		assert_ok!(&domain);
		assert_eq!(
			domain.unwrap(),
			hex!("0700000046324489ceb6ada6d118eacdbe94f49b1fcb49d5481a685979670c7c").into()
		);
	});
}

#[test]
pub fn compute_signing_root_bls() {
	new_tester().execute_with(|| {
		let signing_root = EthereumBeaconClient::compute_signing_root(
			&BeaconHeader {
				slot: 3529537,
				proposer_index: 192549,
				parent_root: hex!(
					"1f8dc05ea427f78e84e2e2666e13c3befb7106fd1d40ef8a3f67cf615f3f2a4c"
				)
				.into(),
				state_root: hex!(
					"0dfb492a83da711996d2d76b64604f9bca9dc08b6c13cf63b3be91742afe724b"
				)
				.into(),
				body_root: hex!("66fba38f7c8c2526f7ddfe09c1a54dd12ff93bdd4d0df6a0950e88e802228bfa")
					.into(),
			},
			hex!("07000000afcaaba0efab1ca832a15152469bb09bb84641c405171dfa2d3fb45f").into(),
		);

		assert_ok!(&signing_root);
		assert_eq!(
			signing_root.unwrap(),
			hex!("3ff6e9807da70b2f65cdd58ea1b25ed441a1d589025d2c4091182026d7af08fb").into()
		);
	});
}

#[test]
pub fn compute_signing_root() {
	new_tester().execute_with(|| {
		let signing_root = EthereumBeaconClient::compute_signing_root(
			&BeaconHeader {
				slot: 222472,
				proposer_index: 10726,
				parent_root: hex!(
					"5d481a9721f0ecce9610eab51d400d223683d599b7fcebca7e4c4d10cdef6ebb"
				)
				.into(),
				state_root: hex!(
					"14eb4575895f996a84528b789ff2e4d5148242e2983f03068353b2c37015507a"
				)
				.into(),
				body_root: hex!("7bb669c75b12e0781d6fa85d7fc2f32d64eafba89f39678815b084c156e46cac")
					.into(),
			},
			hex!("07000000e7acb21061790987fa1c1e745cccfb358370b33e8af2b2c18938e6c2").into(),
		);

		assert_ok!(&signing_root);
		assert_eq!(
			signing_root.unwrap(),
			hex!("da12b6a6d3516bc891e8a49f82fc1925cec40b9327e06457f695035303f55cd8").into()
		);
	});
}

#[test]
pub fn compute_domain_bls() {
	new_tester().execute_with(|| {
		let domain = EthereumBeaconClient::compute_domain(
			hex!("07000000").into(),
			hex!("01000000"),
			hex!("4b363db94e286120d76eb905340fdd4e54bfe9f06bf33ff6cf5ad27f511bfe95").into(),
		);

		assert_ok!(&domain);
		assert_eq!(
			domain.unwrap(),
			hex!("07000000afcaaba0efab1ca832a15152469bb09bb84641c405171dfa2d3fb45f").into()
		);
	});
}

#[test]
pub fn verify_merkle_branch_for_finalized_root() {
	new_tester().execute_with(|| {
		assert!(verify_merkle_branch(
			hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
			&[
				hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
				hex!("5f6f02af29218292d21a69b64a794a7c0873b3e0f54611972863706e8cbdf371").into(),
				hex!("e7125ff9ab5a840c44bedb4731f440a405b44e15f2d1a89e27341b432fabe13d").into(),
				hex!("002c1fe5bc0bd62db6f299a582f2a80a6d5748ccc82e7ed843eaf0ae0739f74a").into(),
				hex!("d2dc4ba9fd4edff6716984136831e70a6b2e74fca27b8097a820cbbaa5a6e3c3").into(),
				hex!("91f77a19d8afa4a08e81164bb2e570ecd10477b3b65c305566a6d2be88510584").into(),
			],
			crate::config::FINALIZED_ROOT_INDEX,
			crate::config::FINALIZED_ROOT_DEPTH,
			hex!("e46559327592741956f6beaa0f52e49625eb85dce037a0bd2eff333c743b287f").into()
		));
	});
}

#[test]
pub fn verify_merkle_branch_fails_if_depth_and_branch_dont_match() {
	new_tester().execute_with(|| {
		assert!(!verify_merkle_branch(
			hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
			&[
				hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
				hex!("5f6f02af29218292d21a69b64a794a7c0873b3e0f54611972863706e8cbdf371").into(),
				hex!("e7125ff9ab5a840c44bedb4731f440a405b44e15f2d1a89e27341b432fabe13d").into(),
			],
			crate::config::FINALIZED_ROOT_INDEX,
			crate::config::FINALIZED_ROOT_DEPTH,
			hex!("e46559327592741956f6beaa0f52e49625eb85dce037a0bd2eff333c743b287f").into()
		));
	});
}

#[test]
pub fn sync_committee_participation_is_supermajority() {
	let bits =
		hex!("bffffffff7f1ffdfcfeffeffbfdffffbfffffdffffefefffdffff7f7ffff77fffdf7bff77ffdf7fffafffffff77fefffeff7effffffff5f7fedfffdfb6ddff7b"
	);
	let participation = primitives::decompress_sync_committee_bits::<512, 64>(bits);
	assert_ok!(EthereumBeaconClient::sync_committee_participation_is_supermajority(&participation));
}

#[test]
pub fn sync_committee_participation_is_supermajority_errors_when_not_supermajority() {
	new_tester().execute_with(|| {
		let participation: [u8; 512] = [
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
			1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0,
			1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
			1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
			1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1,
			0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1,
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
		];

		assert_err!(
			EthereumBeaconClient::sync_committee_participation_is_supermajority(&participation),
			Error::<Test>::SyncCommitteeParticipantsNotSupermajority
		);
	});
}

#[test]
fn compute_fork_version() {
	let mock_fork_versions = ForkVersions {
		genesis: Fork { version: [0, 0, 0, 0], epoch: 0 },
		altair: Fork { version: [0, 0, 0, 1], epoch: 10 },
		bellatrix: Fork { version: [0, 0, 0, 2], epoch: 20 },
		capella: Fork { version: [0, 0, 0, 3], epoch: 30 },
		deneb: Fork { version: [0, 0, 0, 4], epoch: 40 },
	};
	new_tester().execute_with(|| {
		assert_eq!(EthereumBeaconClient::select_fork_version(&mock_fork_versions, 0), [0, 0, 0, 0]);
		assert_eq!(EthereumBeaconClient::select_fork_version(&mock_fork_versions, 1), [0, 0, 0, 0]);
		assert_eq!(
			EthereumBeaconClient::select_fork_version(&mock_fork_versions, 10),
			[0, 0, 0, 1]
		);
		assert_eq!(
			EthereumBeaconClient::select_fork_version(&mock_fork_versions, 21),
			[0, 0, 0, 2]
		);
		assert_eq!(
			EthereumBeaconClient::select_fork_version(&mock_fork_versions, 20),
			[0, 0, 0, 2]
		);
		assert_eq!(
			EthereumBeaconClient::select_fork_version(&mock_fork_versions, 32),
			[0, 0, 0, 3]
		);
	});
}

#[test]
fn find_absent_keys() {
	let participation: [u8; 32] = [
		0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1,
	];
	let update = load_sync_committee_update_fixture();
	let sync_committee_prepared: SyncCommitteePrepared =
		(&update.next_sync_committee_update.unwrap().next_sync_committee)
			.try_into()
			.unwrap();

	new_tester().execute_with(|| {
		let pubkeys = EthereumBeaconClient::find_pubkeys(
			&participation,
			(*sync_committee_prepared.pubkeys).as_ref(),
			false,
		);
		assert_eq!(pubkeys.len(), 2);
		assert_eq!(pubkeys[0], sync_committee_prepared.pubkeys[0]);
		assert_eq!(pubkeys[1], sync_committee_prepared.pubkeys[7]);
	});
}

#[test]
fn find_present_keys() {
	let participation: [u8; 32] = [
		0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
		1, 0,
	];
	let update = load_sync_committee_update_fixture();
	let sync_committee_prepared: SyncCommitteePrepared =
		(&update.next_sync_committee_update.unwrap().next_sync_committee)
			.try_into()
			.unwrap();

	new_tester().execute_with(|| {
		let pubkeys = EthereumBeaconClient::find_pubkeys(
			&participation,
			(*sync_committee_prepared.pubkeys).as_ref(),
			true,
		);
		assert_eq!(pubkeys.len(), 4);
		assert_eq!(pubkeys[0], sync_committee_prepared.pubkeys[1]);
		assert_eq!(pubkeys[1], sync_committee_prepared.pubkeys[8]);
		assert_eq!(pubkeys[2], sync_committee_prepared.pubkeys[26]);
		assert_eq!(pubkeys[3], sync_committee_prepared.pubkeys[30]);
	});
}

/* SYNC PROCESS TESTS */

#[test]
fn process_initial_checkpoint() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::force_checkpoint(
			RuntimeOrigin::root(),
			checkpoint.clone()
		));
		let block_root: H256 = checkpoint.header.hash_tree_root().unwrap();
		assert!(<FinalizedBeaconState<Test>>::contains_key(block_root));
	});
}

#[test]
fn process_initial_checkpoint_with_invalid_sync_committee_proof() {
	let mut checkpoint = Box::new(load_checkpoint_update_fixture());
	checkpoint.current_sync_committee_branch[0] = TEST_HASH.into();

	new_tester().execute_with(|| {
		assert_err!(
			EthereumBeaconClient::force_checkpoint(RuntimeOrigin::root(), checkpoint),
			Error::<Test>::InvalidSyncCommitteeMerkleProof
		);
	});
}

#[test]
fn process_initial_checkpoint_with_invalid_blocks_root_proof() {
	let mut checkpoint = Box::new(load_checkpoint_update_fixture());
	checkpoint.block_roots_branch[0] = TEST_HASH.into();

	new_tester().execute_with(|| {
		assert_err!(
			EthereumBeaconClient::force_checkpoint(RuntimeOrigin::root(), checkpoint),
			Error::<Test>::InvalidBlockRootsRootMerkleProof
		);
	});
}

#[test]
fn submit_update_in_current_period() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_finalized_header_update_fixture());
	let initial_period = compute_period(checkpoint.header.slot);
	let update_period = compute_period(update.finalized_header.slot);
	assert_eq!(initial_period, update_period);

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update.clone()));
		let block_root: H256 = update.finalized_header.hash_tree_root().unwrap();
		assert!(<FinalizedBeaconState<Test>>::contains_key(block_root));
	});
}

#[test]
fn submit_update_with_sync_committee_in_current_period() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_sync_committee_update_fixture());
	let init_period = compute_period(checkpoint.header.slot);
	let update_period = compute_period(update.finalized_header.slot);
	assert_eq!(init_period, update_period);

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert!(!<NextSyncCommittee<Test>>::exists());
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update));
		assert!(<NextSyncCommittee<Test>>::exists());
	});
}

#[test]
fn reject_submit_update_in_next_period() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let sync_committee_update = Box::new(load_sync_committee_update_fixture());
	let update = Box::new(load_next_finalized_header_update_fixture());
	let sync_committee_period = compute_period(sync_committee_update.finalized_header.slot);
	let next_sync_committee_period = compute_period(update.finalized_header.slot);
	assert_eq!(sync_committee_period + 1, next_sync_committee_period);
	let next_sync_committee_update = Box::new(load_next_sync_committee_update_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(
			RuntimeOrigin::signed(1),
			sync_committee_update.clone()
		));
		// check an update in the next period is rejected
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update.clone()),
			Error::<Test>::SyncCommitteeUpdateRequired
		);
		// submit update with next sync committee
		assert_ok!(EthereumBeaconClient::submit(
			RuntimeOrigin::signed(1),
			next_sync_committee_update
		));
		// check same header in the next period can now be submitted successfully
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update.clone()));
	});
}

#[test]
fn submit_update_with_invalid_header_proof() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let mut update = Box::new(load_sync_committee_update_fixture());
	let init_period = compute_period(checkpoint.header.slot);
	let update_period = compute_period(update.finalized_header.slot);
	assert_eq!(init_period, update_period);
	update.finality_branch[0] = TEST_HASH.into();

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert!(!<NextSyncCommittee<Test>>::exists());
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::InvalidHeaderMerkleProof
		);
	});
}

#[test]
fn submit_update_with_invalid_block_roots_proof() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let mut update = Box::new(load_sync_committee_update_fixture());
	let init_period = compute_period(checkpoint.header.slot);
	let update_period = compute_period(update.finalized_header.slot);
	assert_eq!(init_period, update_period);
	update.block_roots_branch[0] = TEST_HASH.into();

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert!(!<NextSyncCommittee<Test>>::exists());
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::InvalidBlockRootsRootMerkleProof
		);
	});
}

#[test]
fn submit_update_with_invalid_next_sync_committee_proof() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let mut update = Box::new(load_sync_committee_update_fixture());
	let init_period = compute_period(checkpoint.header.slot);
	let update_period = compute_period(update.finalized_header.slot);
	assert_eq!(init_period, update_period);
	if let Some(ref mut next_sync_committee_update) = update.next_sync_committee_update {
		next_sync_committee_update.next_sync_committee_branch[0] = TEST_HASH.into();
	}

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert!(!<NextSyncCommittee<Test>>::exists());
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::InvalidSyncCommitteeMerkleProof
		);
	});
}

#[test]
fn submit_update_with_skipped_period() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let sync_committee_update = Box::new(load_sync_committee_update_fixture());
	let mut update = Box::new(load_next_finalized_header_update_fixture());
	update.signature_slot += (EPOCHS_PER_SYNC_COMMITTEE_PERIOD * SLOTS_PER_EPOCH) as u64;
	update.attested_header.slot = update.signature_slot - 1;

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(
			RuntimeOrigin::signed(1),
			sync_committee_update.clone()
		));
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::SkippedSyncCommitteePeriod
		);
	});
}

#[test]
fn submit_update_with_sync_committee_in_next_period() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_sync_committee_update_fixture());
	let next_update = Box::new(load_next_sync_committee_update_fixture());
	let update_period = compute_period(update.finalized_header.slot);
	let next_update_period = compute_period(next_update.finalized_header.slot);
	assert_eq!(update_period + 1, next_update_period);

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert!(!<NextSyncCommittee<Test>>::exists());
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update.clone()));
		assert!(<NextSyncCommittee<Test>>::exists());
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), next_update.clone()));
		let last_finalized_state =
			FinalizedBeaconState::<Test>::get(LatestFinalizedBlockRoot::<Test>::get()).unwrap();
		let last_synced_period = compute_period(last_finalized_state.slot);
		assert_eq!(last_synced_period, next_update_period);
	});
}

#[test]
fn submit_update_with_sync_committee_invalid_signature_slot() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let mut update = Box::new(load_sync_committee_update_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));

		// makes a invalid update with signature_slot should be more than attested_slot
		update.signature_slot = update.attested_header.slot;

		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::InvalidUpdateSlot
		);
	});
}

#[test]
fn submit_update_with_skipped_sync_committee_period() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_update = Box::new(load_next_finalized_header_update_fixture());
	let checkpoint_period = compute_period(checkpoint.header.slot);
	let next_sync_committee_period = compute_period(finalized_update.finalized_header.slot);
	assert_eq!(checkpoint_period + 1, next_sync_committee_period);

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_update),
			Error::<Test>::SkippedSyncCommitteePeriod
		);
	});
}

#[test]
fn submit_irrelevant_update() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let mut update = Box::new(load_next_finalized_header_update_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));

		// makes an invalid update where the attested_header slot value should be greater than the
		// checkpoint slot value
		update.finalized_header.slot = checkpoint.header.slot;
		update.attested_header.slot = checkpoint.header.slot;
		update.signature_slot = checkpoint.header.slot + 1;

		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::IrrelevantUpdate
		);
	});
}

#[test]
fn submit_update_with_missing_bootstrap() {
	let update = Box::new(load_next_finalized_header_update_fixture());

	new_tester().execute_with(|| {
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::NotBootstrapped
		);
	});
}

#[test]
fn submit_update_with_invalid_sync_committee_update() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_sync_committee_update_fixture());
	let mut next_update = Box::new(load_next_sync_committee_update_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));

		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update));

		// makes update with invalid next_sync_committee
		<FinalizedBeaconState<Test>>::mutate(<LatestFinalizedBlockRoot<Test>>::get(), |x| {
			let prev = x.unwrap();
			*x = Some(CompactBeaconState { slot: next_update.attested_header.slot, ..prev });
		});
		next_update.attested_header.slot += 1;
		next_update.signature_slot = next_update.attested_header.slot + 1;
		let next_sync_committee = NextSyncCommitteeUpdate::default();
		next_update.next_sync_committee_update = Some(next_sync_committee);

		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), next_update),
			Error::<Test>::InvalidSyncCommitteeUpdate
		);
	});
}

/// Check that a gap of more than 8192 slots between finalized headers is not allowed.
#[test]
fn submit_finalized_header_update_with_too_large_gap() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_sync_committee_update_fixture());
	let mut next_update = Box::new(load_next_sync_committee_update_fixture());

	// Adds 8193 slots, so that the next update is still in the next sync committee, but the
	// gap between the finalized headers is more than 8192 slots.
	let slot_with_large_gap = checkpoint.header.slot + SLOTS_PER_HISTORICAL_ROOT as u64 + 1;

	next_update.finalized_header.slot = slot_with_large_gap;
	// Adding some slots to the attested header and signature slot since they need to be ahead
	// of the finalized header.
	next_update.attested_header.slot = slot_with_large_gap + 33;
	next_update.signature_slot = slot_with_large_gap + 43;

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update.clone()));
		assert!(<NextSyncCommittee<Test>>::exists());
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), next_update.clone()),
			Error::<Test>::InvalidFinalizedHeaderGap
		);
	});
}

/// Check that a gap of 8192 slots between finalized headers is allowed.
#[test]
fn submit_finalized_header_update_with_gap_at_limit() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_sync_committee_update_fixture());
	let mut next_update = Box::new(load_next_sync_committee_update_fixture());

	next_update.finalized_header.slot = checkpoint.header.slot + SLOTS_PER_HISTORICAL_ROOT as u64;
	// Adding some slots to the attested header and signature slot since they need to be ahead
	// of the finalized header.
	next_update.attested_header.slot =
		checkpoint.header.slot + SLOTS_PER_HISTORICAL_ROOT as u64 + 33;
	next_update.signature_slot = checkpoint.header.slot + SLOTS_PER_HISTORICAL_ROOT as u64 + 43;

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update.clone()));
		assert!(<NextSyncCommittee<Test>>::exists());
		assert_err!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), next_update.clone()),
			// The test should pass the InvalidFinalizedHeaderGap check, and will fail at the
			// next check, the merkle proof, because we changed the next_update slots.
			Error::<Test>::InvalidHeaderMerkleProof
		);
	});
}

/* IMPLS */

#[test]
fn verify_message() {
	let (event_log, proof) = get_message_verification_payload();

	new_tester().execute_with(|| {
		assert_ok!(initialize_storage());
		assert_ok!(EthereumBeaconClient::verify(&event_log, &proof));
	});
}

#[test]
fn verify_message_invalid_proof() {
	let (event_log, mut proof) = get_message_verification_payload();
	proof.receipt_proof.1[0] = TEST_HASH.into();

	new_tester().execute_with(|| {
		assert_ok!(initialize_storage());
		assert_err!(
			EthereumBeaconClient::verify(&event_log, &proof),
			VerificationError::InvalidProof
		);
	});
}

#[test]
fn verify_message_invalid_receipts_root() {
	let (event_log, mut proof) = get_message_verification_payload();
	let mut payload = deneb::ExecutionPayloadHeader::default();
	payload.receipts_root = TEST_HASH.into();
	proof.execution_proof.execution_header = VersionedExecutionPayloadHeader::Deneb(payload);

	new_tester().execute_with(|| {
		assert_ok!(initialize_storage());
		assert_err!(
			EthereumBeaconClient::verify(&event_log, &proof),
			VerificationError::InvalidExecutionProof(
				Error::<Test>::BlockBodyHashTreeRootFailed.into()
			)
		);
	});
}

#[test]
fn verify_message_invalid_log() {
	let (mut event_log, proof) = get_message_verification_payload();
	event_log.topics = vec![H256::zero(); 10];
	new_tester().execute_with(|| {
		assert_ok!(initialize_storage());
		assert_err!(
			EthereumBeaconClient::verify(&event_log, &proof),
			VerificationError::InvalidLog
		);
	});
}

#[test]
fn verify_message_receipt_does_not_contain_log() {
	let (mut event_log, proof) = get_message_verification_payload();
	event_log.data = hex!("f9013c94ee9170abfbf9421ad6dd07f6bdec9d89f2b581e0f863a01b11dcf133cc240f682dab2d3a8e4cd35c5da8c9cf99adac4336f8512584c5ada000000000000000000000000000000000000000000000000000000000000003e8a00000000000000000000000000000000000000000000000000000000000000002b8c000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000068000f000000000000000101d184c103f7acc340847eee82a0b909e3358bc28d440edffa1352b13227e8ee646f3ea37456dec70100000101001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0000e8890423c78a0000000000000000000000000000000000000000000000000000000000000000").to_vec();

	new_tester().execute_with(|| {
		assert_ok!(initialize_storage());
		assert_err!(
			EthereumBeaconClient::verify(&event_log, &proof),
			VerificationError::LogNotFound
		);
	});
}

#[test]
fn set_operating_mode() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let update = Box::new(load_finalized_header_update_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));

		assert_ok!(EthereumBeaconClient::set_operating_mode(
			RuntimeOrigin::root(),
			snowbridge_core::BasicOperatingMode::Halted
		));

		assert_noop!(
			EthereumBeaconClient::submit(RuntimeOrigin::signed(1), update),
			Error::<Test>::Halted
		);
	});
}

#[test]
fn set_operating_mode_root_only() {
	new_tester().execute_with(|| {
		assert_noop!(
			EthereumBeaconClient::set_operating_mode(
				RuntimeOrigin::signed(1),
				snowbridge_core::BasicOperatingMode::Halted
			),
			DispatchError::BadOrigin
		);
	});
}

#[test]
fn verify_execution_proof_invalid_ancestry_proof() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_header_update = Box::new(load_finalized_header_update_fixture());
	let mut execution_header_update = Box::new(load_execution_proof_fixture());
	if let Some(ref mut ancestry_proof) = execution_header_update.ancestry_proof {
		ancestry_proof.header_branch[0] = TEST_HASH.into()
	}

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_header_update));
		assert_err!(
			EthereumBeaconClient::verify_execution_proof(&execution_header_update),
			Error::<Test>::InvalidAncestryMerkleProof
		);
	});
}

#[test]
fn verify_execution_proof_invalid_execution_header_proof() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_header_update = Box::new(load_finalized_header_update_fixture());
	let mut execution_header_update = Box::new(load_execution_proof_fixture());
	execution_header_update.execution_branch[0] = TEST_HASH.into();

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_header_update));
		assert_err!(
			EthereumBeaconClient::verify_execution_proof(&execution_header_update),
			Error::<Test>::InvalidExecutionHeaderProof
		);
	});
}

#[test]
fn verify_execution_proof_that_is_also_finalized_header_which_is_not_stored() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_header_update = Box::new(load_finalized_header_update_fixture());
	let mut execution_header_update = Box::new(load_execution_proof_fixture());
	execution_header_update.ancestry_proof = None;

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_header_update));
		assert_err!(
			EthereumBeaconClient::verify_execution_proof(&execution_header_update),
			Error::<Test>::ExpectedFinalizedHeaderNotStored
		);
	});
}

#[test]
fn submit_execution_proof_that_is_also_finalized_header_which_is_stored_but_slots_dont_match() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_header_update = Box::new(load_finalized_header_update_fixture());
	let mut execution_header_update = Box::new(load_execution_proof_fixture());
	execution_header_update.ancestry_proof = None;

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_header_update));

		let block_root: H256 = execution_header_update.header.hash_tree_root().unwrap();

		<FinalizedBeaconState<Test>>::insert(
			block_root,
			CompactBeaconState {
				slot: execution_header_update.header.slot + 1,
				block_roots_root: Default::default(),
			},
		);
		LatestFinalizedBlockRoot::<Test>::set(block_root);

		assert_err!(
			EthereumBeaconClient::verify_execution_proof(&execution_header_update),
			Error::<Test>::ExpectedFinalizedHeaderNotStored
		);
	});
}

#[test]
fn verify_execution_proof_not_finalized() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_header_update = Box::new(load_finalized_header_update_fixture());
	let update = Box::new(load_execution_proof_fixture());

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_header_update));

		<FinalizedBeaconState<Test>>::mutate(<LatestFinalizedBlockRoot<Test>>::get(), |x| {
			let prev = x.unwrap();
			*x = Some(CompactBeaconState { slot: update.header.slot - 1, ..prev });
		});

		assert_err!(
			EthereumBeaconClient::verify_execution_proof(&update),
			Error::<Test>::HeaderNotFinalized
		);
	});
}

#[test]
fn verify_execution_proof_mainnet() {
	let checkpoint = Box::new(load_checkpoint_update_fixture());
	let finalized_header_update = Box::new(load_finalized_header_update_fixture());
	let log = Log{
		address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
		topics: vec![
			hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
			hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
			hex!("5f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0").into(),
		],
		data: hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e0001000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").into(),
	};
	let proof = &Proof{
		receipt_proof: (vec![
			hex!("4a98e45a319168b0fc6005ce6b744ee9bf54338e2c0784b976a8578d241ced0f").to_vec(),
		], vec![
			hex!("f9028c30b9028802f90284018301d205b9010000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000000000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000040004000000000000002000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000200000000000010f90179f85894eda338e4dc46038493b885327842fd3e301cab39e1a0f78bb28d4b1d7da699e5c0bc2be29c2b04b5aab6aacf6298fe5304f9db9c6d7ea000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7df9011c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a05f7060e971b0dc81e63f0aa41831091847d97c1a4693ac450cc128c7214e65e0b8a000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000002e0001000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7d00e40b54020000000000000000000000000000000000000000000000000000000000").to_vec(),
		]),
		execution_proof: ExecutionProof {
			header: BeaconHeader {
				slot: 393,
				proposer_index: 4,
				parent_root: hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef").into(),
				state_root: hex!("b62ac34a8cb82497be9542fe2114410c9f6021855b766015406101a1f3d86434").into(),
				body_root: hex!("308e4c20194c0c77155c65a2d2c7dcd0ec6a7b20bdeb002c065932149fe0aa1b").into(),
			},
			ancestry_proof: Some(AncestryProof {
				header_branch: vec![
					hex!("6545b47a614a1dd4cad042a0cdbbf5be347e8ffcdc02c6c64540d5153acebeef").into(),
					hex!("fa84cc88ca53a72181599ff4eb07d8b444bce023fe2347c3b4f51004c43439d3").into(),
					hex!("cadc8ae211c6f2221c9138e829249adf902419c78eb4727a150baa4d9a02cc9d").into(),
					hex!("33a89962df08a35c52bd7e1d887cd71fa7803e68787d05c714036f6edf75947c").into(),
					hex!("2c9760fce5c2829ef3f25595a703c21eb22d0186ce223295556ed5da663a82cf").into(),
					hex!("e1aa87654db79c8a0ecd6c89726bb662fcb1684badaef5cd5256f479e3c622e1").into(),
					hex!("aa70d5f314e4a1fbb9c362f3db79b21bf68b328887248651fbd29fc501d0ca97").into(),
					hex!("160b6c235b3a1ed4ef5f80b03ee1c76f7bf3f591c92fca9d8663e9221b9f9f0f").into(),
					hex!("f68d7dcd6a07a18e9de7b5d2aa1980eb962e11d7dcb584c96e81a7635c8d2535").into(),
					hex!("1d5f912dfd6697110dd1ecb5cb8e77952eef57d85deb373572572df62bb157fc").into(),
					hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b").into(),
					hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220").into(),
					hex!("b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f").into(),
				],
				finalized_block_root: hex!("751414cd97c0624f922b3e80285e9f776b08fa22fd5f87391f2ed7ef571a8d46").into(),
			}),
			execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
				parent_hash: hex!("8092290aa21b7751576440f77edd02a94058429ce50e63a92d620951fb25eda2").into(),
				fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
				state_root: hex!("96a83e9ddf745346fafcb0b03d57314623df669ed543c110662b21302a0fae8b").into(),
				receipts_root: hex!("62d13e9a073dc7cf609005b5531bb208c8686f18f7c8ae02d76232d83ae41a21").into(),
				logs_bloom: hex!("00000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000080000000400000000000000000000004000000000080000000000000000000000000000000000010100000000000000000000000000000000020000000000000000000000000000000000080000000000000000000000000000040004000000000000002002002000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000080000000000000000000000000000000000100000000000000000200000200000010").into(),
				prev_randao: hex!("62e309d4f5119d1f5c783abc20fc1a549efbab546d8d0b25ff1cfd58be524e67").into(),
				block_number: 393,
				gas_limit: 54492273,
				gas_used: 199644,
				timestamp: 1710552813,
				extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").into(),
				base_fee_per_gas: U256::from(7u64),
				block_hash: hex!("6a9810efb9581d30c1a5c9074f27c68ea779a8c1ae31c213241df16225f4e131").into(),
				transactions_root: hex!("2cfa6ed7327e8807c7973516c5c32a68ef2459e586e8067e113d081c3bd8c07d").into(),
				withdrawals_root: hex!("792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535").into(),
				blob_gas_used: 0,
				excess_blob_gas: 0,
			}),
			execution_branch: vec![
				hex!("a6833fa629f3286b6916c6e50b8bf089fc9126bee6f64d0413b4e59c1265834d").into(),
				hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
				hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
				hex!("d3af7c05c516726be7505239e0b9c7cb53d24abce6b91cdb3b3995f0164a75da").into(),
			],
		}
	};
	let finalized_header = BeaconHeader{
		slot: 864,
		proposer_index: 4,
		parent_root: hex!("614e7672f991ac268cd841055973f55e1e42228831a211adef207bb7329be614").into(),
		state_root: hex!("5fa8dfca3d760e4242ab46d529144627aa85348a19173b6e081172c701197a4a").into(),
		body_root: hex!("0f34c083b1803666bb1ac5e73fa71582731a2cf37d279ff0a3b0cad5a2ff371e").into(),
	};
	let block_roots_root: H256 = hex!("3adb5c78afd49ef17160ca7fc38b47228cbb13a317709c86bb6f51d799ba9ab6").into();

	new_tester().execute_with(|| {
		assert_ok!(EthereumBeaconClient::process_checkpoint_update(&checkpoint));
		assert_ok!(EthereumBeaconClient::submit(RuntimeOrigin::signed(1), finalized_header_update.clone()));

		<FinalizedBeaconState<Test>>::mutate(<LatestFinalizedBlockRoot<Test>>::get(), |x| {
			let prev = x.unwrap();
			*x = Some(CompactBeaconState { slot: finalized_header_update.finalized_header.slot, block_roots_root });
		});

		EthereumBeaconClient::verify(&log, &proof).unwrap();
	});
}
