// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_asset_conversion`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-anb7yjbi-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_asset_conversion
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/asset-conversion/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_asset_conversion`.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn swap_exact_tokens_for_tokens(n: u32, ) -> Weight;
	fn swap_tokens_for_exact_tokens(n: u32, ) -> Weight;
}

/// Weights for `pallet_asset_conversion` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `AssetConversion::Pools` (r:1 w:1)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetConversion::NextPoolAssetId` (r:1 w:1)
	/// Proof: `AssetConversion::NextPoolAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `910`
		//  Estimated: `6360`
		// Minimum execution time: 86_709_000 picoseconds.
		Weight::from_parts(88_841_000, 6360)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:2 w:2)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1507`
		//  Estimated: `11426`
		// Minimum execution time: 148_672_000 picoseconds.
		Weight::from_parts(151_824_000, 11426)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1650`
		//  Estimated: `11426`
		// Minimum execution time: 130_743_000 picoseconds.
		Weight::from_parts(132_793_000, 11426)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `Assets::Asset` (r:4 w:4)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:8 w:8)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 4]`.
	fn swap_exact_tokens_for_tokens(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + n * (419 ±0)`
		//  Estimated: `990 + n * (5218 ±0)`
		// Minimum execution time: 81_173_000 picoseconds.
		Weight::from_parts(82_574_000, 990)
			// Standard Error: 335_929
			.saturating_add(Weight::from_parts(11_607_291, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 5218).saturating_mul(n.into()))
	}
	/// Storage: `Assets::Asset` (r:4 w:4)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:8 w:8)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 4]`.
	fn swap_tokens_for_exact_tokens(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + n * (419 ±0)`
		//  Estimated: `990 + n * (5218 ±0)`
		// Minimum execution time: 80_562_000 picoseconds.
		Weight::from_parts(82_501_000, 990)
			// Standard Error: 329_460
			.saturating_add(Weight::from_parts(11_295_339, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 5218).saturating_mul(n.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `AssetConversion::Pools` (r:1 w:1)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetConversion::NextPoolAssetId` (r:1 w:1)
	/// Proof: `AssetConversion::NextPoolAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `910`
		//  Estimated: `6360`
		// Minimum execution time: 86_709_000 picoseconds.
		Weight::from_parts(88_841_000, 6360)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:2 w:2)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1507`
		//  Estimated: `11426`
		// Minimum execution time: 148_672_000 picoseconds.
		Weight::from_parts(151_824_000, 11426)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1650`
		//  Estimated: `11426`
		// Minimum execution time: 130_743_000 picoseconds.
		Weight::from_parts(132_793_000, 11426)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: `Assets::Asset` (r:4 w:4)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:8 w:8)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 4]`.
	fn swap_exact_tokens_for_tokens(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + n * (419 ±0)`
		//  Estimated: `990 + n * (5218 ±0)`
		// Minimum execution time: 81_173_000 picoseconds.
		Weight::from_parts(82_574_000, 990)
			// Standard Error: 335_929
			.saturating_add(Weight::from_parts(11_607_291, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 5218).saturating_mul(n.into()))
	}
	/// Storage: `Assets::Asset` (r:4 w:4)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:8 w:8)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 4]`.
	fn swap_tokens_for_exact_tokens(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + n * (419 ±0)`
		//  Estimated: `990 + n * (5218 ±0)`
		// Minimum execution time: 80_562_000 picoseconds.
		Weight::from_parts(82_501_000, 990)
			// Standard Error: 329_460
			.saturating_add(Weight::from_parts(11_295_339, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 5218).saturating_mul(n.into()))
	}
}
