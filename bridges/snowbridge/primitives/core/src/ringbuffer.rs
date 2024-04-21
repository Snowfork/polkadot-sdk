// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use codec::FullCodec;
use core::{
	cmp::Ord,
	marker::PhantomData,
	ops::{Add, Sub},
};
use frame_support::storage::{types::QueryKindTrait, StorageMap, StorageValue};
use sp_core::{Get, GetDefault};
use sp_runtime::traits::{One, Zero};
use sp_std::{clone::Clone, default::Default};

/// Trait object presenting the ringbuffer interface.
pub trait RingBufferMap<Key, Value, QueryKind>
where
	Key: FullCodec,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, GetDefault>,
{
	/// Insert a map entry.
	fn insert(k: Key, v: Value);

	/// Check if map contains a key
	fn contains_key(k: Key) -> bool;

	/// Get the value of the key
	fn get(k: Key) -> QueryKind::Query;
}

pub struct RingBufferMapImpl<Index, B, CurrentIndex, Intermediate, M, QueryKind>(
	PhantomData<(Index, B, CurrentIndex, Intermediate, M, QueryKind)>,
);

/// Ringbuffer implementation based on `RingBufferTransient`
impl<Key, Value, Index, B, CurrentIndex, Intermediate, M, QueryKind>
	RingBufferMap<Key, Value, QueryKind>
	for RingBufferMapImpl<Index, B, CurrentIndex, Intermediate, M, QueryKind>
where
	Key: FullCodec + Clone,
	Value: FullCodec,
	Index: Ord + One + Zero + Add<Output = Index> + Copy + FullCodec + Eq,
	B: Get<Index>,
	CurrentIndex: StorageValue<Index, Query = Index>,
	Intermediate: StorageMap<Index, Key, Query = Key>,
	M: StorageMap<Key, Value, Query = QueryKind::Query>,
	QueryKind: QueryKindTrait<Value, GetDefault>,
{
	/// Insert a map entry.
	fn insert(k: Key, v: Value) {
		let bound = B::get();
		let mut current_index = CurrentIndex::get();

		// Adding one here as bound denotes number of items but our index starts with zero.
		if (current_index + Index::one()) >= bound {
			current_index = Index::zero();
		} else {
			current_index = current_index + Index::one();
		}

		// Deleting earlier entry if it exists
		if Intermediate::contains_key(current_index) {
			let older_key = Intermediate::get(current_index);
			M::remove(older_key);
		}

		Intermediate::insert(current_index, k.clone());
		CurrentIndex::set(current_index);
		M::insert(k, v);
	}

	/// Check if map contains a key
	fn contains_key(k: Key) -> bool {
		M::contains_key(k)
	}

	/// Get the value associated with key
	fn get(k: Key) -> M::Query {
		M::get(k)
	}
}

pub trait CheckOverWriteTrait<Value> {
	fn can_over_write(value: Value, prev_value: Value) -> bool;
}

pub struct RingBufferMapImplWithConditionalOverWrite<
	Index,
	B,
	CurrentIndex,
	Intermediate,
	M,
	Checker,
	QueryKind,
>(PhantomData<(Index, B, CurrentIndex, Intermediate, M, Checker, QueryKind)>);

/// Ringbuffer implementation based on `RingBufferTransient`
impl<Key, Value, Index, B, CurrentIndex, Intermediate, M, Checker, QueryKind>
	RingBufferMap<Key, Value, QueryKind>
	for RingBufferMapImplWithConditionalOverWrite<
		Index,
		B,
		CurrentIndex,
		Intermediate,
		M,
		Checker,
		QueryKind,
	> where
	Key: FullCodec + Clone,
	Value: FullCodec + Default + Clone,
	Index: Ord + One + Zero + Add<Output = Index> + Sub<Output = Index> + Copy + FullCodec + Eq,
	B: Get<Index>,
	CurrentIndex: StorageValue<Index, Query = Index>,
	Intermediate: StorageMap<Index, Key, Query = Key>,
	M: StorageMap<Key, Value, Query = QueryKind::Query>,
	Checker: CheckOverWriteTrait<Value>,
	QueryKind: QueryKindTrait<Value, GetDefault>,
{
	/// Insert a map entry.
	fn insert(k: Key, v: Value) {
		let bound = B::get();
		let mut current_index = CurrentIndex::get();

		// If the ringbuffer is at the first slot, then the previous head is the last slot
		// because the ringbuffer wraps around
		let second_last_index = if current_index == Index::zero() {
			bound - Index::one()
		} else {
			current_index - Index::one()
		};

		let prev_key = Intermediate::get(second_last_index);
		let prev_value_query = M::get(prev_key);
		let prev_value =
			QueryKind::from_query_to_optional_value(prev_value_query).unwrap_or_default();

		// If the last finalized header and the one provided to be added to the store
		// is less than a sync committee apart, overwrite the last finalized header since
		// we only need one finalized update per SLOTS_PER_HISTORICAL_ROOT. If the slots
		// are SLOTS_PER_HISTORICAL_ROOT apart, store the update in a new index in the
		// ringbuffer.
		// Do not overwrite for the first index (initial beacon checkpoint)
		// if prev_value.slot != 0 && slot - prev_value.slot < SLOTS_PER_HISTORICAL_ROOT as u64 {
		if Checker::can_over_write(v.clone(), prev_value) {
			let current_key = Intermediate::get(current_index);
			Intermediate::insert(current_index, k.clone());
			M::insert(k, v);
			M::remove(current_key);
		} else {
			// Adding one here as bound denotes number of items but our index starts with zero.
			if (current_index + Index::one()) == bound {
				current_index = Index::zero();
			} else {
				current_index = current_index + Index::one();
			}
			// Deleting earlier entry if it exists
			if Intermediate::contains_key(current_index) {
				let older_key = Intermediate::get(current_index);
				M::remove(older_key);
			}

			Intermediate::insert(current_index, k.clone());
			CurrentIndex::set(current_index);
			M::insert(k, v);
		}
	}

	/// Check if map contains a key
	fn contains_key(k: Key) -> bool {
		M::contains_key(k)
	}

	/// Get the value associated with key
	fn get(k: Key) -> M::Query {
		M::get(k)
	}
}
