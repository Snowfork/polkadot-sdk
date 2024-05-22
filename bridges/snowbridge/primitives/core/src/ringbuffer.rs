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

/// Trait to check if a previous value can be overwritten by the provided one.
pub trait CheckOverwrite<Value> {
	/// Determines whether to overwrite an existing value based on the previous value.
	fn can_overwrite(new_value: &Value, prev_value: &Value) -> bool;
}

pub struct RingBufferMapImplWithConditionalOverWrite<
	Index,
	B,
	CurrentIndex,
	Intermediate,
	M,
	QueryKind,
	OverwriteCondition,
>(PhantomData<(Index, B, CurrentIndex, Intermediate, M, QueryKind, OverwriteCondition)>);

/// Ringbuffer implementation based on `RingBufferTransient`
impl<Key, Value, Index, B, CurrentIndex, Intermediate, M, QueryKind, OverwriteCondition>
	RingBufferMap<Key, Value, QueryKind>
	for RingBufferMapImplWithConditionalOverWrite<
		Index,
		B,
		CurrentIndex,
		Intermediate,
		M,
		QueryKind,
		OverwriteCondition,
	> where
	Key: FullCodec + Clone,
	Value: FullCodec,
	Index: Ord + One + Zero + Add<Output = Index> + Sub<Output = Index> + Copy + FullCodec + Eq,
	B: Get<Index>,
	CurrentIndex: StorageValue<Index, Query = Index>,
	Intermediate: StorageMap<Index, Key, Query = Key>,
	M: StorageMap<Key, Value, Query = QueryKind::Query>,
	QueryKind: QueryKindTrait<Value, GetDefault>,
	OverwriteCondition: CheckOverwrite<Value>,
{
	/// Insert a map entry.
	fn insert(k: Key, v: Value) {
		let bound = B::get();
		let mut current_index = CurrentIndex::get();
		let mut prev_value_option = None;

		// Retrieve the last inserted value, if it exists.
		if Intermediate::contains_key(current_index) {
			// If the index of the last inserted item is 0, we need to look for the
			// previous value in the array at the end of the ringbuffer (to wrap around).
			let prev_index = if current_index == Index::zero() {
				bound.sub(Index::one())
			} else {
				// Else the index is just current_index - 1
				current_index.sub(Index::one())
			};
			// Get the previous item's key
			let prev_key = Intermediate::get(prev_index);
			// Get the previous item's value, using the key
			prev_value_option = QueryKind::from_query_to_optional_value(M::get(prev_key.clone()));
		}

		// Decide whether to overwrite or insert the next item. If a value
		// at the previous index exists, use it to
		if let Some(prev_value) = prev_value_option {
			if OverwriteCondition::can_overwrite(&v, &prev_value) {
				// Store the last item's key so we can delete the old value.
				let current_key = Intermediate::get(current_index);
				// Insert the item at the same index as the last item,
				// effectively overwriting it.
				Intermediate::insert(current_index, k.clone());
				// Insert the new item in the item map.
				M::insert(k, v);
				// Delete the old, now overwritten item using the current we
				// stored in current_key.
				M::remove(current_key);
				return;
			}
		}

		// At this point, we have determined the new value should just
		// be inserted normally.
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
