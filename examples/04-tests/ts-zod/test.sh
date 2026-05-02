#!/usr/bin/env bash

set -euo pipefail

echo "🌀 Building TypeScript Zod schemas"
if output=$(cargo run -p genotype_cli --bin gt -- build . 2>&1); then
	echo "🟢 Build: OK"
else
	echo "🔴 Build: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

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
