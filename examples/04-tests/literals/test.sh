#!/usr/bin/env bash

set -euo pipefail

#region Python

LATEST_VERSIONS=(
	"3.14"
	"3.13"
	"3.12"
)

LEGACY_VERSIONS=(
	"3.11"
	"3.10"
	"3.9"
	"3.8"
)

build_for_python() {
	version_family="$1"
	echo -e "🌀 Building for $version_family Python"
	if output=$(cargo run -p genotype_cli --bin gt -- build . --config "genotype.py-${version_family}.toml" 2>&1); then
		echo "🟢 Build: OK"
	else
		echo "🔴 Build: FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
	echo
}

run_python_tests_for() {
	version_family="$1"
	python_version="$2"
	pythonpath="dist/py-${version_family}/py${PYTHONPATH:+:$PYTHONPATH}"

	if output=$(PYTHONPATH="$pythonpath" uv run --python "$python_version" python test.py 2>&1); then
		echo "🟢 Python $python_version: OK"
	else
		echo "🔴 Python $python_version: FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
}

build_for_python "latest"

echo "🌀 Running latest Python tests"

for version in "${LATEST_VERSIONS[@]}"; do
	run_python_tests_for "latest" "$version"
done

echo
build_for_python "legacy"

echo "🌀 Running legacy Python tests"

for version in "${LEGACY_VERSIONS[@]}"; do
	run_python_tests_for "legacy" "$version"
done

#endregion

echo
echo "🌀 Building for TypeScript and Rust"
if output=$(cargo run -p genotype_cli --bin gt -- build . --config genotype.toml 2>&1); then
	echo "🟢 Build: OK"
else
	echo "🔴 Build: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

#region TypeScript

echo
echo "🌀 Running TypeScript tests"

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

#endregion

#region Rust

echo
echo "🌀 Running Rust tests"

if output=$(cargo test --manifest-path ./rs/Cargo.toml 2>&1); then
	echo "🟢 Rust tests: OK"
else
	echo "🔴 Rust tests: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	# TODO: Fix literals and enable:
	# exit 1
fi

#endregion
