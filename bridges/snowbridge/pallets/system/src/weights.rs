
//! Autogenerated weights for `snowbridge_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-09, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `crake.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain
// bridge-hub-rococo-dev
// --pallet=snowbridge_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template
// ../parachain/templates/module-weight-template.hbs
// --output
// ../parachain/pallets/control/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `snowbridge_system`.
pub trait WeightInfo {
	fn upgrade() -> Weight;
	fn create_agent() -> Weight;
	fn create_channel() -> Weight;
	fn update_channel() -> Weight;
	fn force_update_channel() -> Weight;
	fn set_operating_mode() -> Weight;
	fn transfer_native_from_agent() -> Weight;
	fn force_transfer_native_from_agent() -> Weight;
	fn set_token_transfer_fees() -> Weight;
	fn set_pricing_parameters() -> Weight;
	fn register_token() -> Weight;
	fn force_register_token() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 44_000_000 picoseconds.
		Weight::from_parts(44_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: EthereumSystem Agents (r:1 w:1)
	/// Proof: EthereumSystem Agents (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn create_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187`
		//  Estimated: `6196`
		// Minimum execution time: 85_000_000 picoseconds.
		Weight::from_parts(85_000_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthereumSystem Agents (r:1 w:0)
	/// Proof: EthereumSystem Agents (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: EthereumSystem Channels (r:1 w:1)
	/// Proof: EthereumSystem Channels (max_values: None, max_size: Some(12), added: 2487, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn create_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `602`
		//  Estimated: `69050`
		// Minimum execution time: 83_000_000 picoseconds.
		Weight::from_parts(83_000_000, 69050)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: EthereumSystem Channels (r:1 w:0)
	/// Proof: EthereumSystem Channels (max_values: None, max_size: Some(12), added: 2487, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn update_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `6044`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(40_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: EthereumSystem Channels (r:1 w:0)
	/// Proof: EthereumSystem Channels (max_values: None, max_size: Some(12), added: 2487, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn force_update_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `6044`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(41_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn set_operating_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(31_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: EthereumSystem Agents (r:1 w:0)
	/// Proof: EthereumSystem Agents (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn transfer_native_from_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252`
		//  Estimated: `6044`
		// Minimum execution time: 45_000_000 picoseconds.
		Weight::from_parts(45_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: EthereumSystem Agents (r:1 w:0)
	/// Proof: EthereumSystem Agents (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn force_transfer_native_from_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252`
		//  Estimated: `6044`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(42_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn set_token_transfer_fees() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(42_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EthereumOutboundQueue PalletOperatingMode (r:1 w:0)
	/// Proof: EthereumOutboundQueue PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn set_pricing_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(42_000_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	fn register_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `6044`
		// Minimum execution time: 45_000_000 picoseconds.
		Weight::from_parts(45_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	fn force_register_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `7044`
		// Minimum execution time: 46_000_000 picoseconds.
		Weight::from_parts(46_000_000, 7044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
