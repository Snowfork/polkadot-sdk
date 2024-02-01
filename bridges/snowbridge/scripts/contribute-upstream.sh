#!/bin/bash

# A script to remove everything from snowbridge repository/subtree, except:
#
# - parachain
# - readme
# - license

set -eu

# show CLI help
function show_help() {
  set +x
  echo " "
  echo Error: $1
  echo "Usage:"
  echo "  ./bridges/snowbridge/scripts/contribute-upstream.sh          Exit with code 0 if pallets code is well decoupled from the other code in the repo"
  echo "Options:"
  echo "  --no-revert                                Leaves only runtime code on exit"
  echo "  --ignore-git-state                         Ignores git actual state"
  exit 1
}

# parse CLI args
NO_REVERT=
IGNORE_GIT_STATE=
for i in "$@"
do
	case $i in
		--no-revert)
			NO_REVERT=true
			shift
			;;
		--ignore-git-state)
			IGNORE_GIT_STATE=true
			shift
			;;
		*)
			show_help "Unknown option: $i"
			;;
	esac
done

# the script is able to work only on clean git copy, unless we want to ignore this check
[[ ! -z "${IGNORE_GIT_STATE}" ]] || [[ -z "$(git status --porcelain)" ]] || { echo >&2 "The git copy must be clean"; exit 1; }

# let's leave repository/subtree in its original (clean) state if something fails below
function revert_to_clean_state {
	[[ ! -z "${NO_REVERT}" ]] || { echo "Reverting to clean state..."; git checkout .; }
}
trap revert_to_clean_state EXIT

# let's avoid any restrictions on where this script can be called for - snowbridge repo may be
# plugged into any other repo folder. So the script (and other stuff that needs to be removed)
# may be located either in call dir, or one of it subdirs.
SNOWBRIDGE_FOLDER="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )/../.."

# remove everything we think is not required for our needs
rm -rf rust-toolchain.toml
rm -rf $SNOWBRIDGE_FOLDER/.cargo
rm -rf $SNOWBRIDGE_FOLDER/.config
rm -rf $SNOWBRIDGE_FOLDER/.github
rm -rf $SNOWBRIDGE_FOLDER/SECURITY.md
rm -rf $SNOWBRIDGE_FOLDER/.gitignore
rm -rf $SNOWBRIDGE_FOLDER/templates
rm -rf $SNOWBRIDGE_FOLDER/pallets/ethereum-client/fuzz

pushd $SNOWBRIDGE_FOLDER

# let's test if everything we need compiles
cargo check -p snowbridge-pallet-ethereum-client
cargo check -p snowbridge-pallet-ethereum-client --features runtime-benchmarks
cargo check -p snowbridge-pallet-ethereum-client --features try-runtime
cargo check -p snowbridge-pallet-inbound-queue
cargo check -p snowbridge-pallet-inbound-queue --features runtime-benchmarks
cargo check -p snowbridge-pallet-inbound-queue --features try-runtime
cargo check -p snowbridge-pallet-outbound-queue
cargo check -p snowbridge-pallet-outbound-queue --features runtime-benchmarks
cargo check -p snowbridge-pallet-outbound-queue --features try-runtime
cargo check -p snowbridge-pallet-system
cargo check -p snowbridge-pallet-system --features runtime-benchmarks
cargo check -p snowbridge-pallet-system --features try-runtime

# we're removing lock file after all checks are done. Otherwise we may use different
# Substrate/Polkadot/Cumulus commits and our checks will fail
rm -f $SNOWBRIDGE_FOLDER/parachain/Cargo.toml
rm -f $SNOWBRIDGE_FOLDER/parachain/Cargo.lock

# Revert Parity's Github Actions
pushd ../../..

pwd
rm -rf .github
git remote -v | grep -w foo || git remote add parity https://github.com/paritytech/polkadot-sdk
git checkout parity/master -- .github

popd
popd

echo "OK"
