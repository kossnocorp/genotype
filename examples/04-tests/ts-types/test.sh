#!/usr/bin/env bash

set -euo pipefail

build_for() {
	name="$1"
	config="$2"
	echo "🌀 Building TypeScript ($name)"
	if output=$(cargo run -p genotype_cli --bin gt -- build . --config "$config" 2>&1); then
		echo "🟢 Build ($name): OK"
	else
		echo "🔴 Build ($name): FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
}

build_for "interface" "genotype.interface.toml"
echo
build_for "alias" "genotype.alias.toml"
echo

if output=$(pnpm install 2>&1); then
	echo "🟢 pnpm install: OK"
else
	echo "🔴 pnpm install: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

if output=$(pnpm tsc 2>&1); then
	echo "🟢 Types: OK"
else
	echo "🔴 Types: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

if output=$(pnpm tsx test.ts 2>&1); then
	echo "🟢 Node test: OK"
else
	echo "🔴 Node test: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
