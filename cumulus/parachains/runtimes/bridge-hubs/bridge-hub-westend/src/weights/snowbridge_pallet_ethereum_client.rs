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

//! Autogenerated weights for `snowbridge_pallet_ethereum_client`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Claras-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain=bridge-hub-rococo-dev
// --pallet=snowbridge_pallet_ethereum_client
// --extrinsic
// *
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 20
// --output
// cumulus/parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/snowbridge_pallet_ethereum_client.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `snowbridge_pallet_ethereum_client`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> snowbridge_pallet_ethereum_client::WeightInfo for WeightInfo<T> {
	/// Storage: `EthereumBeaconClient::FinalizedBeaconStateIndex` (r:1 w:1)
	/// Proof: `EthereumBeaconClient::FinalizedBeaconStateIndex` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::FinalizedBeaconStateMapping` (r:1 w:1)
	/// Proof: `EthereumBeaconClient::FinalizedBeaconStateMapping` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::NextSyncCommittee` (r:0 w:1)
	/// Proof: `EthereumBeaconClient::NextSyncCommittee` (`max_values`: Some(1), `max_size`: Some(92372), added: 92867, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::InitialCheckpointRoot` (r:0 w:1)
	/// Proof: `EthereumBeaconClient::InitialCheckpointRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::ValidatorsRoot` (r:0 w:1)
	/// Proof: `EthereumBeaconClient::ValidatorsRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::LatestFinalizedBlockRoot` (r:0 w:1)
	/// Proof: `EthereumBeaconClient::LatestFinalizedBlockRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::CurrentSyncCommittee` (r:0 w:1)
	/// Proof: `EthereumBeaconClient::CurrentSyncCommittee` (`max_values`: Some(1), `max_size`: Some(92372), added: 92867, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::FinalizedBeaconState` (r:0 w:1)
	/// Proof: `EthereumBeaconClient::FinalizedBeaconState` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	fn force_checkpoint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3501`
		// Minimum execution time: 67_553_000_000 picoseconds.
		Weight::from_parts(68_677_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3501))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `EthereumBeaconClient::OperatingMode` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::OperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::LatestFinalizedBlockRoot` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::LatestFinalizedBlockRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::FinalizedBeaconState` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::FinalizedBeaconState` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::NextSyncCommittee` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::NextSyncCommittee` (`max_values`: Some(1), `max_size`: Some(92372), added: 92867, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::CurrentSyncCommittee` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::CurrentSyncCommittee` (`max_values`: Some(1), `max_size`: Some(92372), added: 92867, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::ValidatorsRoot` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::ValidatorsRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `92749`
		//  Estimated: `93857`
		// Minimum execution time: 16_988_000_000 picoseconds.
		Weight::from_parts(17_125_000_000, 0)
			.saturating_add(Weight::from_parts(0, 93857))
			.saturating_add(T::DbWeight::get().reads(6))
	}
	/// Storage: `EthereumBeaconClient::OperatingMode` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::OperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::LatestFinalizedBlockRoot` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::LatestFinalizedBlockRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::FinalizedBeaconState` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::FinalizedBeaconState` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::NextSyncCommittee` (r:1 w:1)
	/// Proof: `EthereumBeaconClient::NextSyncCommittee` (`max_values`: Some(1), `max_size`: Some(92372), added: 92867, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::CurrentSyncCommittee` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::CurrentSyncCommittee` (`max_values`: Some(1), `max_size`: Some(92372), added: 92867, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::ValidatorsRoot` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::ValidatorsRoot` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn submit_with_sync_committee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `92749`
		//  Estimated: `93857`
		// Minimum execution time: 84_553_000_000 picoseconds.
		Weight::from_parts(87_459_000_000, 0)
			.saturating_add(Weight::from_parts(0, 93857))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}

	/// Storage: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e83e6d574e897864a327c716c553f277037` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e83e6d574e897864a327c716c553f277037` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e834209354bdd86a5d7050a9b80004c2d6d` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e834209354bdd86a5d7050a9b80004c2d6d` (r:0 w:1)
	/// Storage: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e83964405908d330d65518e9e60960ba9f1` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e83964405908d330d65518e9e60960ba9f1` (r:0 w:1)
	/// Storage: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e83d98b4ec57d38fb2970ffc6289380767d` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0xada12a87b9ccce83f328569cf9934e83d98b4ec57d38fb2970ffc6289380767d` (r:0 w:1)
	fn step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215`
		//  Estimated: `3680`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3680))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
