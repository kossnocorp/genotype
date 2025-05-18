#!/usr/bin/env bash

set -e

if ! command -v rustup >/dev/null 2>&1; then
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

if ! command -v cargo-binstall >/dev/null 2>&1; then
    echo "Installing cargo-binstall..."
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
fi

cargo binstall just
cargo binstall cargo-nextest
cargo binstall cargo-watch
cargo binstall cargo-release