#!/usr/bin/env bash

set -euo pipefail

rm -rf ./dist

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

PYTHON_CHECKS_FAILED=0

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
		# TODO: Implement and enable:
		# exit 1
	fi
	echo
}

run_python_tests_for() {
	version_family="$1"
	python_version="$2"
	PYTHONPATH="dist/py-${version_family}/py${PYTHONPATH:+:$PYTHONPATH}"

	export VIRTUAL_ENV=""

	if output=$(PYTHONPATH="$PYTHONPATH" uv run --python "$python_version" python test.py 2>&1); then
		echo "🟢 Python $python_version: OK"
	else
		echo "🔴 Python $python_version: FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		# TODO: Implement and enable:
		# exit 1
	fi
}

build_for_python "latest"
for version in "${LATEST_VERSIONS[@]}"; do
	run_python_tests_for "latest" "$version"
done

echo

build_for_python "legacy"
for version in "${LEGACY_VERSIONS[@]}"; do
	run_python_tests_for "legacy" "$version"
done

echo

#endregion

#region TypeScript

TS_VARIANTS=(
	"zod"
	"interface"
	"alias"
)

build_for_ts() {
	variant="$1"
	echo "🌀 Building TypeScript for $variant mode"
	if output=$(cargo run -p genotype_cli --bin gt -- build . --config "genotype.ts-$variant.toml" 2>&1); then
		echo "🟢 Build ($variant): OK"
	else
		echo "🔴 Build ($variant): FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
	echo
}

run_ts_tests_for() {
	variant="$1"
	path="./ts/$variant"

	echo "🌀 Running TypeScript tests for $variant mode"

	if ! cd "$path"; then
		echo "🔴 Failed to cd to $path"
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

	cd -
}

build_for_ts "interface"
run_ts_tests_for "interface"

echo

build_for_ts "alias"
run_ts_tests_for "alias"

echo

build_for_ts "zod"
run_ts_tests_for "zod"

#endregion

#region Rust

echo
echo "🌀 Building for Rust"

if output=$(cargo run -p genotype_cli --bin gt -- build . --config genotype.rs.toml 2>&1); then
	echo "🟢 Build: OK"
else
	echo "🔴 Build: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	# TODO: Implement and enable:
	# exit 1
fi

echo
echo "🌀 Running Rust tests"

if output=$(cargo test --manifest-path ./rs/Cargo.toml 2>&1); then
	echo "🟢 Rust tests: OK"
else
	echo "🔴 Rust tests: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	# TODO: Implement and enable:
	# exit 1
fi

#endregion
