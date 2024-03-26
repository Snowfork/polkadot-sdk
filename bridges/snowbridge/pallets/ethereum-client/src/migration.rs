// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use super::*;
use frame_support::traits::OnRuntimeUpgrade;
use log;
use frame_support::traits::PalletInfoAccess;
use frame_support::migration::clear_storage_prefix;

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

pub mod v1 {
    use frame_support::{pallet_prelude::*, weights::Weight};

    use super::*;

    const LOG_TARGET: &str = "ethereum-client::migration";

    pub struct InitializeOnUpgrade<T>(
        sp_std::marker::PhantomData<T>,
    );
    impl<T> OnRuntimeUpgrade
    for InitializeOnUpgrade<T>
        where
            T: Config,
    {
        fn on_runtime_upgrade() -> Weight {

            // To delete
            // LatestExecutionState
            // ExecutionHeaders
            // ExecutionHeaderIndex
            // ExecutionHeaderIndex
            log::info!(target: LOG_TARGET, "Running migration v1.");

            let res = migration::clear_storage_prefix(
                <Pallet<T>>::name().as_bytes(),
                b"LatestExecutionState",
                b"",
                None,
                None,
            );

            log::info!(
                target: LOG_TARGET,
                "Cleared '{}' entries from 'LatestExecutionState' storage prefix",
                res.unique
            );

            if res.maybe_cursor.is_some() {
                log::error!(
				target: LOG_TARGET,
				"Storage prefix 'LatestExecutionState' is not completely cleared."
			);
            }

            T::DbWeight::get().writes(res.unique.into())
            //T::DbWeight::get().reads_writes(1, 1)
        }
    }
}
