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

//! Autogenerated weights for `snowbridge_pallet_inbound_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `macbook pro 14 m2`, CPU: `m2-arm64`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-rococo-dev"), DB CACHE: 1024

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain=bridge-hub-rococo-dev
// --pallet=snowbridge_inbound_queue
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/snowbridge_inbound_queue.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `snowbridge_pallet_inbound_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> snowbridge_pallet_inbound_queue::WeightInfo for WeightInfo<T> {
	/// Storage: EthereumInboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumInboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: EthereumBeaconClient ExecutionHeaders (r:1 w:0)
	/// Proof: EthereumBeaconClient ExecutionHeaders (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: EthereumInboundQueue Nonce (r:1 w:1)
	/// Proof: EthereumInboundQueue Nonce (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `800`
		//  Estimated: `7200`
		// Minimum execution time: 200_000_000 picoseconds.
		Weight::from_parts(200_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7200))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
