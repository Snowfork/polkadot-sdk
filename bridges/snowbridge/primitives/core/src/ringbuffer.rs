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
		let current_index = CurrentIndex::get();
		let mut prev_value_option = None;
		let bound = B::get();

		// Retrieve previous value if it exists
		if Intermediate::contains_key(current_index) {
			let prev_index = if current_index == Index::zero() {
				bound.sub(Index::one())
			} else {
				current_index.sub(Index::one())
			};
			let prev_key = Intermediate::get(prev_index);
			prev_value_option = QueryKind::from_query_to_optional_value(M::get(prev_key.clone()));
		}

		// Decide whether to overwrite or advance index
		if let Some(prev_value) = prev_value_option {
			if OverwriteCondition::can_overwrite(&v, &prev_value) {
				// overwrite the last index
				let current_index = CurrentIndex::get();
				let current_key = Intermediate::get(current_index);
				Intermediate::insert(current_index, k.clone());
				M::insert(k, v);
				M::remove(current_key);
				return;
			}
		}

		// If not overwriting, advance index and insert normally
		let next_index = if current_index + Index::one() >= bound {
			Index::zero()
		} else {
			current_index + Index::one()
		};

		if Intermediate::contains_key(next_index) {
			let older_key = Intermediate::get(next_index);
			M::remove(older_key);
		}

		Intermediate::insert(next_index, k.clone());
		CurrentIndex::set(next_index);
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
