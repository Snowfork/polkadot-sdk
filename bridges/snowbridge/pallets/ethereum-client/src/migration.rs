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

            let prefix = storage_prefix(<Pallet<T>>::name().as_bytes(), b"LatestExecutionState");
            if sp_io::storage::get(&prefix).is_some() {
                let res = clear_storage_prefix(
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

                return T::DbWeight::get().reads_writes(1, 1);
            } else {
                log::info!(
                    target: LOG_TARGET,
                    "LatestExecutionState was already cleared",
                );
            }

            let prefix = storage_prefix(<Pallet<T>>::name().as_bytes(), b"ExecutionHeaders");
            let next_key = storage_iter::<i32>(<Pallet<T>>::name().as_bytes(), b"ExecutionHeaders").next();
            //let next_key = sp_io::storage::next_key(&prefix)
            if next_key.is_some() {
                let next = next_key.clone().unwrap();

                sp_io::storage::clear(&next.0);
                log::info!(
                    target: LOG_TARGET,
                    "Value is {:?}",
                    next_key.unwrap()
                );


                return T::DbWeight::get().reads_writes(1, 1);
            } else {
                log::info!(
                    target: LOG_TARGET,
                    "ExecutionHeaders was already cleared",
                );
            }



            T::DbWeight::get().reads_writes(1, 1)
        }
    }
}
