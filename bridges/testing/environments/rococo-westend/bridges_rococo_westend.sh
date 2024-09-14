#!/bin/bash

# import common functions
source "$FRAMEWORK_PATH/utils/bridges.sh"

# Expected sovereign accounts.
#
# Generated by:
#
#  #[test]
#	fn generate_sovereign_accounts() {
#		use sp_core::crypto::Ss58Codec;
#		use polkadot_parachain_primitives::primitives::Sibling;
#
#		parameter_types! {
#			pub UniversalLocationAHR: InteriorMultiLocation = X2(GlobalConsensus(Rococo), Parachain(1000));
#			pub UniversalLocationAHW: InteriorMultiLocation = X2(GlobalConsensus(Westend), Parachain(1000));
#		}
#
#		// SS58=42
#		println!("GLOBAL_CONSENSUS_ROCOCO_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusConvertsFor::<UniversalLocationAHW, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X1(GlobalConsensus(Rococo)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#		println!("ASSET_HUB_WESTEND_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_WESTEND=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 SiblingParachainConvertsVia::<Sibling, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 1, interior: X1(Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#
#		// SS58=42
#		println!("GLOBAL_CONSENSUS_WESTEND_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusConvertsFor::<UniversalLocationAHR, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X1(GlobalConsensus(Westend)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#		println!("ASSET_HUB_ROCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_ROCOCO=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 SiblingParachainConvertsVia::<Sibling, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 1, interior: X1(Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(42_u16.into())
#		);
#	}
GLOBAL_CONSENSUS_ROCOCO_SOVEREIGN_ACCOUNT="5GxRGwT8bU1JeBPTUXc7LEjZMxNrK8MyL2NJnkWFQJTQ4sii"
ASSET_HUB_WESTEND_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_WESTEND="5Eg2fntNprdN3FgH4sfEaaZhYtddZQSQUqvYJ1f2mLtinVhV"
GLOBAL_CONSENSUS_WESTEND_SOVEREIGN_ACCOUNT="5He2Qdztyxxa4GoagY6q1jaiLMmKy1gXS7PdZkhfj8ZG9hk5"
ASSET_HUB_ROCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_ROCOCO="5Eg2fntNprdN3FgH4sfEaaZhYtddZQSQUqvYJ1f2mLtinVhV"

# Expected sovereign accounts for rewards on BridgeHubs.
#
# Generated by:
#	#[test]
#	fn generate_sovereign_accounts_for_rewards() {
#		use bp_messages::LaneId;
#		use bp_relayers::{PayRewardFromAccount, RewardsAccountOwner, RewardsAccountParams};
#		use sp_core::crypto::Ss58Codec;
#
#		// SS58=42
#		println!(
#			"ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhwd_ThisChain=\"{}\"",
#			frame_support::sp_runtime::AccountId32::new(
#				PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#					LaneId([0, 0, 0, 2]),
#					*b"bhwd",
#					RewardsAccountOwner::ThisChain
#				))
#			)
#				.to_ss58check_with_version(42_u16.into())
#		);
#		// SS58=42
#		println!(
#			"ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhwd_BridgedChain=\"{}\"",
#			frame_support::sp_runtime::AccountId32::new(
#				PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#					LaneId([0, 0, 0, 2]),
#					*b"bhwd",
#					RewardsAccountOwner::BridgedChain
#				))
#			)
#				.to_ss58check_with_version(42_u16.into())
#		);
#
#		// SS58=42
#		println!(
#			"ON_BRIDGE_HUB_WESTEND_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhro_ThisChain=\"{}\"",
#			frame_support::sp_runtime::AccountId32::new(
#				PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#					LaneId([0, 0, 0, 2]),
#					*b"bhro",
#					RewardsAccountOwner::ThisChain
#				))
#			)
#				.to_ss58check_with_version(42_u16.into())
#		);
#		// SS58=42
#		println!(
#			"ON_BRIDGE_HUB_WESTEND_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhro_BridgedChain=\"{}\"",
#			frame_support::sp_runtime::AccountId32::new(
#				PayRewardFromAccount::<[u8; 32], [u8; 32]>::rewards_account(RewardsAccountParams::new(
#					LaneId([0, 0, 0, 2]),
#					*b"bhro",
#					RewardsAccountOwner::BridgedChain
#				))
#			)
#				.to_ss58check_with_version(42_u16.into())
#		);
#	}
ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhwd_ThisChain="5EHnXaT5BhiSGP5hbdsoVGtzi2sQVgpDNToTxLYeQvKoMPEm"
ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhwd_BridgedChain="5EHnXaT5BhiSGP5hbdt5EJSapXYbxEv678jyWHEUskCXcjqo"
ON_BRIDGE_HUB_WESTEND_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhro_ThisChain="5EHnXaT5BhiSGP5h9Rg8sgUJqoLym3iEaWUiboT8S9AT5xFh"
ON_BRIDGE_HUB_WESTEND_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhro_BridgedChain="5EHnXaT5BhiSGP5h9RgQci1txJ2BDbp7KBRE9k8xty3BMUSi"

LANE_ID="00000002"
XCM_VERSION=3

function init_ro_wnd() {
    local relayer_path=$(ensure_relayer)

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path init-bridge rococo-to-bridge-hub-westend \
	--source-host localhost \
	--source-port 9942 \
	--source-version-mode Auto \
	--target-host localhost \
	--target-port 8945 \
	--target-version-mode Auto \
	--target-signer //Bob
}

function init_wnd_ro() {
    local relayer_path=$(ensure_relayer)

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path init-bridge westend-to-bridge-hub-rococo \
        --source-host localhost \
        --source-port 9945 \
        --source-version-mode Auto \
        --target-host localhost \
        --target-port 8943 \
        --target-version-mode Auto \
        --target-signer //Bob
}

function run_relay() {
    local relayer_path=$(ensure_relayer)

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-headers-and-messages bridge-hub-rococo-bridge-hub-westend \
        --rococo-host localhost \
        --rococo-port 9942 \
        --rococo-version-mode Auto \
        --bridge-hub-rococo-host localhost \
        --bridge-hub-rococo-port 8943 \
        --bridge-hub-rococo-version-mode Auto \
        --bridge-hub-rococo-signer //Charlie \
        --bridge-hub-rococo-transactions-mortality 4 \
        --westend-host localhost \
        --westend-port 9945 \
        --westend-version-mode Auto \
        --bridge-hub-westend-host localhost \
        --bridge-hub-westend-port 8945 \
        --bridge-hub-westend-version-mode Auto \
        --bridge-hub-westend-signer //Charlie \
        --bridge-hub-westend-transactions-mortality 4 \
        --lane "${LANE_ID}"
}

function run_finality_relay() {
    local relayer_path=$(ensure_relayer)

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-headers rococo-to-bridge-hub-westend \
        --only-free-headers \
        --source-uri ws://localhost:9942 \
        --source-version-mode Auto \
        --target-uri ws://localhost:8945 \
        --target-version-mode Auto \
        --target-signer //Charlie \
        --target-transactions-mortality 4&

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-headers westend-to-bridge-hub-rococo \
        --only-free-headers \
        --source-uri ws://localhost:9945 \
        --source-version-mode Auto \
        --target-uri ws://localhost:8943 \
        --target-version-mode Auto \
        --target-signer //Charlie \
        --target-transactions-mortality 4
}

function run_parachains_relay() {
    local relayer_path=$(ensure_relayer)

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-parachains rococo-to-bridge-hub-westend \
        --only-free-headers \
        --source-uri ws://localhost:9942 \
        --source-version-mode Auto \
        --target-uri ws://localhost:8945 \
        --target-version-mode Auto \
        --target-signer //Dave \
        --target-transactions-mortality 4&

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-parachains westend-to-bridge-hub-rococo \
        --only-free-headers \
        --source-uri ws://localhost:9945 \
        --source-version-mode Auto \
        --target-uri ws://localhost:8943 \
        --target-version-mode Auto \
        --target-signer //Dave \
        --target-transactions-mortality 4
}

function run_messages_relay() {
    local relayer_path=$(ensure_relayer)

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-messages bridge-hub-rococo-to-bridge-hub-westend \
        --source-uri ws://localhost:8943 \
        --source-version-mode Auto \
        --source-signer //Eve \
        --source-transactions-mortality 4 \
        --target-uri ws://localhost:8945 \
        --target-version-mode Auto \
        --target-signer //Eve \
        --target-transactions-mortality 4 \
        --lane $LANE_ID&

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        $relayer_path relay-messages bridge-hub-westend-to-bridge-hub-rococo \
        --source-uri ws://localhost:8945 \
        --source-version-mode Auto \
        --source-signer //Ferdie \
        --source-transactions-mortality 4 \
        --target-uri ws://localhost:8943 \
        --target-version-mode Auto \
        --target-signer //Ferdie \
        --target-transactions-mortality 4 \
        --lane $LANE_ID
}

case "$1" in
  run-relay)
    init_wnd_ro
    init_ro_wnd
    run_relay
    ;;
  run-finality-relay)
    init_wnd_ro
    init_ro_wnd
    run_finality_relay
    ;;
  run-parachains-relay)
    run_parachains_relay
    ;;
  run-messages-relay)
    run_messages_relay
    ;;
  init-asset-hub-rococo-local)
      ensure_polkadot_js_api
      # create foreign assets for native Westend token (governance call on Rococo)
      force_create_foreign_asset \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9910" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X1": [{ "GlobalConsensus": "Westend" }] } }')" \
          "$GLOBAL_CONSENSUS_WESTEND_SOVEREIGN_ACCOUNT" \
          10000000000 \
          true
      # HRMP
      open_hrmp_channels \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1000 1013 4 524288
      open_hrmp_channels \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1013 1000 4 524288
      # set XCM version of remote AssetHubWestend
      force_xcm_version \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9910" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Westend" }, { "Parachain": 1000 } ] } }')" \
          $XCM_VERSION
      ;;
  init-bridge-hub-rococo-local)
      ensure_polkadot_js_api
      # SA of sibling asset hub pays for the execution
      transfer_balance \
          "ws://127.0.0.1:8943" \
          "//Alice" \
          "$ASSET_HUB_ROCOCO_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_ROCOCO" \
          100000000000000
      # drip SA of lane dedicated to asset hub for paying rewards for delivery
      transfer_balance \
          "ws://127.0.0.1:8943" \
          "//Alice" \
          "$ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhwd_ThisChain" \
          100000000000000
      # drip SA of lane dedicated to asset hub for paying rewards for delivery confirmation
      transfer_balance \
          "ws://127.0.0.1:8943" \
          "//Alice" \
          "$ON_BRIDGE_HUB_ROCOCO_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhwd_BridgedChain" \
          100000000000000
      # set XCM version of remote BridgeHubWestend
      force_xcm_version \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1013 \
          "ws://127.0.0.1:8943" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Westend" }, { "Parachain": 1002 } ] } }')" \
          $XCM_VERSION
      ;;
  init-asset-hub-westend-local)
      ensure_polkadot_js_api
      # create foreign assets for native Rococo token (governance call on Westend)
      force_create_foreign_asset \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9010" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X1": [{ "GlobalConsensus": "Rococo" }] } }')" \
          "$GLOBAL_CONSENSUS_ROCOCO_SOVEREIGN_ACCOUNT" \
          10000000000 \
          true
      # HRMP
      open_hrmp_channels \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1000 1002 4 524288
      open_hrmp_channels \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1002 1000 4 524288
      # set XCM version of remote AssetHubRococo
      force_xcm_version \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9010" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Rococo" }, { "Parachain": 1000 } ] } }')" \
          $XCM_VERSION
      ;;
  init-bridge-hub-westend-local)
      # SA of sibling asset hub pays for the execution
      transfer_balance \
          "ws://127.0.0.1:8945" \
          "//Alice" \
          "$ASSET_HUB_WESTEND_SOVEREIGN_ACCOUNT_AT_BRIDGE_HUB_WESTEND" \
          100000000000000
      # drip SA of lane dedicated to asset hub for paying rewards for delivery
      transfer_balance \
          "ws://127.0.0.1:8945" \
          "//Alice" \
          "$ON_BRIDGE_HUB_WESTEND_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhro_ThisChain" \
          100000000000000
      # drip SA of lane dedicated to asset hub for paying rewards for delivery confirmation
      transfer_balance \
          "ws://127.0.0.1:8945" \
          "//Alice" \
          "$ON_BRIDGE_HUB_WESTEND_SOVEREIGN_ACCOUNT_FOR_LANE_00000002_bhro_BridgedChain" \
          100000000000000
      # set XCM version of remote BridgeHubRococo
      force_xcm_version \
          "ws://127.0.0.1:9945" \
          "//Alice" \
          1002 \
          "ws://127.0.0.1:8945" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Rococo" }, { "Parachain": 1013 } ] } }')" \
          $XCM_VERSION
      ;;
  reserve-transfer-assets-from-asset-hub-rococo-local)
      amount=$2
      ensure_polkadot_js_api
      # send ROCs to Alice account on AHW
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9910" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Westend" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 1, "interior": "Here" } }, "fun": { "Fungible": '$amount' } } ] }')" \
          0 \
          "Unlimited"
      ;;
  withdraw-reserve-assets-from-asset-hub-rococo-local)
      amount=$2
      ensure_polkadot_js_api
      # send back only 100000000000 wrappedWNDs to Alice account on AHW
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9910" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Westend" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 2, "interior": { "X1": { "GlobalConsensus": "Westend" } } } }, "fun": { "Fungible": '$amount' } } ] }')" \
          0 \
          "Unlimited"
      ;;
  reserve-transfer-assets-from-asset-hub-westend-local)
      amount=$2
      ensure_polkadot_js_api
      # send WNDs to Alice account on AHR
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9010" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Rococo" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 1, "interior": "Here" } }, "fun": { "Fungible": '$amount' } } ] }')" \
          0 \
          "Unlimited"
      ;;
  withdraw-reserve-assets-from-asset-hub-westend-local)
      amount=$2
      ensure_polkadot_js_api
      # send back only 100000000000 wrappedROCs to Alice account on AHR
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9010" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Rococo" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 2, "interior": { "X1": { "GlobalConsensus": "Rococo" } } } }, "fun": { "Fungible": '$amount' } } ] }')" \
          0 \
          "Unlimited"
      ;;
  claim-rewards-bridge-hub-rococo-local)
      ensure_polkadot_js_api
      # bhwd -> [62, 68, 77, 64] -> 0x62687764
      claim_rewards \
          "ws://127.0.0.1:8943" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x62687764" \
          "ThisChain"
      claim_rewards \
          "ws://127.0.0.1:8943" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x62687764" \
          "BridgedChain"
      ;;
  claim-rewards-bridge-hub-westend-local)
      # bhro -> [62, 68, 72, 6f] -> 0x6268726f
      claim_rewards \
          "ws://127.0.0.1:8945" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x6268726f" \
          "ThisChain"
      claim_rewards \
          "ws://127.0.0.1:8945" \
          "//Charlie" \
          "0x${LANE_ID}" \
          "0x6268726f" \
          "BridgedChain"
      ;;
  stop)
    pkill -f polkadot
    pkill -f parachain
    ;;
  import)
    # to avoid trigger anything here
    ;;
  *)
    echo "A command is require. Supported commands for:
    Local (zombienet) run:
          - run-relay
          - run-finality-relay
          - run-parachains-relay
          - run-messages-relay
          - init-asset-hub-rococo-local
          - init-bridge-hub-rococo-local
          - init-asset-hub-westend-local
          - init-bridge-hub-westend-local
          - reserve-transfer-assets-from-asset-hub-rococo-local
          - withdraw-reserve-assets-from-asset-hub-rococo-local
          - reserve-transfer-assets-from-asset-hub-westend-local
          - withdraw-reserve-assets-from-asset-hub-westend-local
          - claim-rewards-bridge-hub-rococo-local
          - claim-rewards-bridge-hub-westend-local";
    exit 1
    ;;
esac
