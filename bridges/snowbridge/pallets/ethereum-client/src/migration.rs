// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use super::*;
use frame_support::traits::OnRuntimeUpgrade;
use log;
use frame_support::traits::PalletInfoAccess;
use frame_support::migration::clear_storage_prefix;
use frame_support::migration::storage_iter;

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

pub mod v1 {
    use frame_support::{pallet_prelude::*, weights::Weight};
    use frame_support::storage::storage_prefix;

    use super::*;

    const LOG_TARGET: &str = "ethereum-client::migration";

    pub struct ExecutionHeaderCleanupOnUpgrade<T>(
        sp_std::marker::PhantomData<T>,
    );
    impl<T> OnRuntimeUpgrade
    for ExecutionHeaderCleanupOnUpgrade<T>
        where
            T: Config,
    {
        fn on_runtime_upgrade() -> Weight {
            // To delete
            // LatestExecutionState
            // ExecutionHeaders
            // ExecutionHeaderIndex
            // ExecutionHeaderMapping
            log::info!(target: LOG_TARGET, "Running migration v1.");

            /*let res = clear_storage_prefix(
                <Pallet<T>>::name().as_bytes(),
                b"LatestExecutionState",
                b"",
                None,
                None,
            );

            log::info!(
                target: LOG_TARGET,
                "Storage prefix LatestExecutionState was cleared."
            );

            if res.unique > 0 {
                return T::DbWeight::get().reads_writes(1, 1);
            }*/

            let res = clear_storage_prefix(
                <Pallet<T>>::name().as_bytes(),
                b"ExecutionHeaders",
                b"",
                Some(1),
                None,
            );

            log::info!(
                target: LOG_TARGET,
                "Cleared '{}' entries from 'ExecutionHeaders' storage prefix",
                res.unique
            );

            log::info!(
                target: LOG_TARGET,
                "Loops: {}",
                res.loops
            );

            if res.maybe_cursor.is_some() {
                log::error!(
                    target: LOG_TARGET,
                    "Storage prefix 'ExecutionHeaders' is not completely cleared."
                );
            }

            return T::DbWeight::get().writes(res.unique.into());
        }
    }
}
