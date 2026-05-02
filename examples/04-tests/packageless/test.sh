#!/usr/bin/env bash

set -euo pipefail

rm -rf ./py/module/types/
rm -rf ./ts/src/types/
rm -rf ./rs/src/types/

echo -e "🌀 Building Genotype"
if output=$(cargo run -p genotype_cli --bin gt -- build . 2>&1); then
	echo "🟢 Build: OK"
else
	echo "🔴 Build: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
echo

echo "🌀 Checking TypeScript package"
if output=$(pnpm --dir "./ts" install --ignore-workspace 2>&1); then
	echo "🟢 pnpm install: OK"
else
	echo "🔴 pnpm install: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

if output=$(pnpm --dir "./ts" tsc 2>&1); then
	echo "🟢 TypeScript check: OK"
else
	echo "🔴 TypeScript check: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
echo

echo "🌀 Checking Rust package"
if output=$(cargo check --manifest-path ./rs/Cargo.toml 2>&1); then
	echo "🟢 Rust check: OK"
else
	echo "🔴 Rust check: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
echo

echo "🌀 Checking Python package"
if output=$(PYTHONPATH="./py" uv run --python 3.13 python ./py/test.py 2>&1); then
	echo "$output"
else
	echo "🔴 Python check: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
