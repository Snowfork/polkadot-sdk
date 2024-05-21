// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
pub use crate::config::{
	SLOTS_PER_HISTORICAL_ROOT, SYNC_COMMITTEE_BITS_SIZE as SC_BITS_SIZE,
	SYNC_COMMITTEE_SIZE as SC_SIZE,
};
use crate::Pallet;
use frame_support::storage::types::OptionQuery;
use snowbridge_core::ringbuffer::RingBufferMapImplWithConditionalOverWrite;
// Specialize types based on configured sync committee size
pub type SyncCommittee = primitives::SyncCommittee<SC_SIZE>;
pub type SyncCommitteePrepared = primitives::SyncCommitteePrepared<SC_SIZE>;
pub type SyncAggregate = primitives::SyncAggregate<SC_SIZE, SC_BITS_SIZE>;
pub type CheckpointUpdate = primitives::CheckpointUpdate<SC_SIZE>;
pub type Update = primitives::Update<SC_SIZE, SC_BITS_SIZE>;
pub type NextSyncCommitteeUpdate = primitives::NextSyncCommitteeUpdate<SC_SIZE>;

pub use primitives::{AncestryProof, ExecutionProof};
