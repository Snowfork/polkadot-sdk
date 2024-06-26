// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-unxyhko3-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_balances
// --chain=rococo-dev
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: `NisCounterpartBalances::Account` (r:2 w:2)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `6164`
		// Minimum execution time: 42_331_000 picoseconds.
		Weight::from_parts(43_215_000, 0)
			.saturating_add(Weight::from_parts(0, 6164))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:2 w:2)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `6164`
		// Minimum execution time: 32_674_000 picoseconds.
		Weight::from_parts(33_564_000, 0)
			.saturating_add(Weight::from_parts(0, 6164))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3577`
		// Minimum execution time: 9_813_000 picoseconds.
		Weight::from_parts(10_111_000, 0)
			.saturating_add(Weight::from_parts(0, 3577))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3593`
		// Minimum execution time: 16_467_000 picoseconds.
		Weight::from_parts(17_088_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:2 w:2)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `6196`
		// Minimum execution time: 43_846_000 picoseconds.
		Weight::from_parts(45_059_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:2 w:2)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `6164`
		// Minimum execution time: 41_260_000 picoseconds.
		Weight::from_parts(42_367_000, 0)
			.saturating_add(Weight::from_parts(0, 6164))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3593`
		// Minimum execution time: 14_914_000 picoseconds.
		Weight::from_parts(15_631_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:999 w:999)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:999 w:999)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[1, 1000]`.
	/// The range of component `u` is `[1, 1000]`.
	fn upgrade_accounts(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + u * (256 ±0)`
		//  Estimated: `990 + u * (2603 ±0)`
		// Minimum execution time: 14_630_000 picoseconds.
		Weight::from_parts(14_924_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 15_311
			.saturating_add(Weight::from_parts(14_920_201, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(u.into()))
	}
	fn force_adjust_total_issuance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_193_000 picoseconds.
		Weight::from_parts(5_403_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn burn_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3577`
		// Minimum execution time: 27_002_000 picoseconds.
		Weight::from_parts(27_785_000, 0)
			.saturating_add(Weight::from_parts(0, 3577))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn burn_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3577`
		// Minimum execution time: 17_533_000 picoseconds.
		Weight::from_parts(18_338_000, 0)
			.saturating_add(Weight::from_parts(0, 3577))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
