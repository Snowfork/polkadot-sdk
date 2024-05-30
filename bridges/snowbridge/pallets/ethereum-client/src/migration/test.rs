// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg(all(test, not(feature = "runtime-benchmarks")))]

use frame_support::traits::OnRuntimeUpgrade;
use pallet_migrations::WeightInfo as _;
use crate::mock::new_test_ext;

#[test]
fn ethereum_execution_header_migration_works() {
    new_test_ext().execute_with(|| {
        frame_support::__private::sp_tracing::try_init_simple();
        // Insert some values into the old storage items.


        // Give it enough weight to do exactly 16 iterations:
        let limit = <T as pallet_migrations::Config>::WeightInfo::progress_mbms_none() +
            pallet_migrations::Pallet::<T>::exec_migration_max_weight() +
            weights::SubstrateWeight::<T>::step() * 16;
        MigratorServiceWeight::set(&limit);

        System::set_block_number(1);
        AllPalletsWithSystem::on_runtime_upgrade(); // onboard MBMs

        // Check everything is empty
    });
}
