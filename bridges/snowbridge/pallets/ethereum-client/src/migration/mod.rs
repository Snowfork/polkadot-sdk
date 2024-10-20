// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use crate::{pallet::Config, WeightInfo};
use frame_support::{
	migrations::{MigrationId, SteppedMigration, SteppedMigrationError},
	pallet_prelude::PhantomData,
	weights::WeightMeter,
};
use sp_core::Get;

mod test;

pub const PALLET_MIGRATIONS_ID: &[u8; 26] = b"ethereum-execution-headers";
pub const LOG_TARGET: &str = "ethereum-client-migration";

/// Module containing the old Ethereum execution headers that should be cleaned up.
pub mod v0 {
	use crate::pallet::{Config, Pallet};
	use frame_support::{
		pallet_prelude::{Decode, Encode, MaxEncodedLen, OptionQuery, TypeInfo, ValueQuery},
		storage_alias, CloneNoBound, Identity, PartialEqNoBound, RuntimeDebugNoBound,
	};
	use sp_core::H256;

	#[storage_alias]
	pub type LatestExecutionState<T: Config> =
		StorageValue<Pallet<T>, ExecutionHeaderState, ValueQuery>;

	#[storage_alias]
	pub type ExecutionHeaders<T: Config> =
		StorageMap<Pallet<T>, Identity, H256, CompactExecutionHeader, OptionQuery>;

	#[storage_alias]
	pub type ExecutionHeaderIndex<T: Config> = StorageValue<Pallet<T>, u32, ValueQuery>;

	#[storage_alias]
	pub type ExecutionHeaderMapping<T: Config> =
		StorageMap<Pallet<T>, Identity, u32, H256, ValueQuery>;

	#[derive(Copy, Clone, Default, Encode, Decode, Debug, TypeInfo, MaxEncodedLen, PartialEq)]
	pub struct ExecutionHeaderState {
		pub beacon_block_root: H256,
		pub beacon_slot: u64,
		pub block_hash: H256,
		pub block_number: u64,
	}

	#[derive(
		Default,
		Encode,
		Decode,
		CloneNoBound,
		PartialEqNoBound,
		RuntimeDebugNoBound,
		TypeInfo,
		MaxEncodedLen,
	)]
	pub struct CompactExecutionHeader {
		pub parent_hash: H256,
		#[codec(compact)]
		pub block_number: u64,
		pub state_root: H256,
		pub receipts_root: H256,
	}
}

pub struct EthereumExecutionHeaderCleanup<T: Config, W: WeightInfo, M: Get<u32>>(
	PhantomData<(T, W, M)>,
);
impl<T: Config, W: WeightInfo, M: Get<u32>> SteppedMigration
	for EthereumExecutionHeaderCleanup<T, W, M>
{
	type Cursor = u32;
	type Identifier = MigrationId<26>; // Length of the migration ID PALLET_MIGRATIONS_ID

	fn id() -> Self::Identifier {
		MigrationId { pallet_id: *PALLET_MIGRATIONS_ID, version_from: 0, version_to: 1 }
	}

	fn step(
		mut cursor: Option<Self::Cursor>,
		meter: &mut WeightMeter,
	) -> Result<Option<Self::Cursor>, SteppedMigrationError> {
		log::info!(target: LOG_TARGET, "Starting step iteration for Ethereum execution header cleanup.");
		let required = W::step();
		if meter.remaining().any_lt(required) {
			return Err(SteppedMigrationError::InsufficientWeight { required });
		}

		// We loop here to do as much progress as possible per step.
		loop {
			if meter.try_consume(required).is_err() {
				break;
			}

			let index = if let Some(last_key) = cursor {
				last_key.saturating_add(1)
			} else {
				log::info!(target: LOG_TARGET, "Cursor is 0, starting migration.");
				// If no cursor is provided, start iterating from the beginning.
				0
			};

			if index >= M::get() {
				v0::LatestExecutionState::<T>::kill();
				v0::ExecutionHeaderIndex::<T>::kill();
				// We are at the end of the migration, signal complete.
				cursor = None;
				log::info!(target: LOG_TARGET, "Ethereum execution header cleanup migration is complete. Index = {}.", index);
				break
			} else {
				let execution_hash = v0::ExecutionHeaderMapping::<T>::get(index);
				v0::ExecutionHeaders::<T>::remove(execution_hash);
				v0::ExecutionHeaderMapping::<T>::remove(index);
				cursor = Some(index);
				log::info!(target: LOG_TARGET, "Migration is in progress. Index = {}.", index);
			}
		}
		Ok(cursor)
	}
}
