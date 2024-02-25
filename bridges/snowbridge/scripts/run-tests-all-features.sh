#!/bin/bash

set -eu

for package in $(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name | startswith("snowbridge")) | .name'); do
    echo "Running tests for package: $package"
    cargo test -p "$package" --all-features
done
