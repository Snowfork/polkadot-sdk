// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use crate::BaseFeePerGas;
use sp_core::U256;
/// A trait for retrieving the base fee per gas.
pub trait GasFeeProvider {
	fn get() -> BaseFeePerGas;
}

pub trait GasFeeStore {
	fn store(value: U256, slot: u64);
}
