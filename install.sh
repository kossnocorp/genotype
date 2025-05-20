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

cargo binstall \
    fnm \
    bacon \
    just \
    cargo-nextest \
    cargo-watch \
    cargo-release \
    toml2json \

fnm install

pnpm install

if ! command -v uv >/dev/null 2>&1; then
    echo "Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
fi

uv python install