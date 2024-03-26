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
            if sp_io::storage::get(&LatestExecutionState::<T>::hashed_key()).is_some() {
                LatestExecutionState::<T>::kill();

                return T::DbWeight::get().reads_writes(1, 1);
            } else {
                log::info!(
                    target: LOG_TARGET,
                    "LatestExecutionState was already cleared",
                );
            }

            let mut iter = ExecutionHeaders::<T>::iter();

            let next_item = iter.next();
            if next_item.is_some() {
                let key = next_item.unwrap().0;
                ExecutionHeaders::<T>::remove(&key);
                log::info!(
                    target: LOG_TARGET,
                    "Cleared execution header: {}", key
                );
            }

            T::DbWeight::get().reads_writes(1, 1)
        }
    }
}
