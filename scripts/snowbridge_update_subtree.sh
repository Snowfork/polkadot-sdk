#!/bin/bash

# A script to udpate bridges repo as subtree to Cumulus
# Usage:
#       ./scripts/update_subtree_snowbridge.sh fetch
#       ./scripts/update_subtree_snowbridge.sh patch

set -e

SNOWBRIDGE_BRANCH="${BRANCH:-main}"
POLKADOT_SDK_BRANCH="${POLKADOT_SDK_BRANCH:-master}"
SNOWBRIDGE_TARGET_DIR="${TARGET_DIR:-bridges/snowbridge}"

function fetch() {
    # the script is able to work only on clean git copy
    [[ -z "$(git status --porcelain)" ]] || {
        echo >&2 "The git copy must be clean (stash all your changes):";
        git status --porcelain
        exit 1;
    }

    local snowbridge_remote=$(git remote -v | grep "snowbridge.git (fetch)" | head -n1 | awk '{print $1;}')
    if [ -z "$snowbridge_remote" ]; then
        echo ""
        echo "Adding new remote: 'snowbridge' repo..."
        echo ""
        git remote add -f snowbridge https://github.com/Snowfork/snowbridge.git
        snowbridge_remote="snowbridge"
    else
        echo ""
        echo "Fetching remote: '${snowbridge_remote}' repo..."
        echo ""
        git fetch https://github.com/Snowfork/snowbridge.git --prune
    fi

    echo ""
    echo "Syncing/updating subtree with remote branch '${snowbridge_remote}/$SNOWBRIDGE_BRANCH' to target directory: '$SNOWBRIDGE_TARGET_DIR'"
    echo ""
    git subtree pull --prefix=$SNOWBRIDGE_TARGET_DIR ${snowbridge_remote} $SNOWBRIDGE_BRANCH --squash
}

function clean() {
    echo ""
    echo "Patching/removing unneeded stuff from subtree in target directory: '$SNOWBRIDGE_TARGET_DIR'"
    remove_parachain_dir
    $SNOWBRIDGE_TARGET_DIR/scripts/verify-pallets-build.sh --ignore-git-state --no-revert
}

function create_patch() {
    [[ -z "$(git status --porcelain)" ]] || {
        echo >&2 "The git copy must be clean (stash all your changes):";
        git status --porcelain
        exit 1;
    }
    echo "Creating diff patch file to apply to snowbridg. No Cargo.toml files will be included in the patch."
    add_parachain_dir
    git diff snowbridge/$SNOWBRIDGE_BRANCH $POLKADOT_SDK_BRANCH:bridges/snowbridge --diff-filter=ACM -- . ':(exclude)*/Cargo.toml' > snowbridge.patch
    remove_parachain_dir
}

function remove_parachain_dir() {
    SOURCE_DIR="bridges/snowbridge/parachain"
    TARGET_DIR="bridges/snowbridge"
    if [[ -e $SOURCE_DIR ]]; then
        echo "Removing parachain dir"
        rm -r $TARGET_DIR/scripts # because there's scripts in both dirs
        mv $SOURCE_DIR/* $TARGET_DIR/

        rm -r $SOURCE_DIR
    else
        echo "Parachain dir already cleared"
    fi
}

function add_parachain_dir() {
    SOURCE_DIR="bridges/snowbridge"
    TARGET_DIR="bridges/snowbridge/parachain"
    if [[ -e $SOURCE_DIR ]]; then
        echo "Parachain dir already added"
    else
        mkdir -p $TARGET_DIR
        mv $SOURCE_DIR/* $TARGET_DIR/
    fi
}

case "$1" in
    fetch)
        fetch
        ;;
    clean)
        clean
        ;;
    create_patch)
        create_patch
        ;;
    update)
        fetch
        clean
        ;;
esac
