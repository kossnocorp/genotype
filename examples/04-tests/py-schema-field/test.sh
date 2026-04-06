#!/usr/bin/env bash

set -euo pipefail

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

build_for() {
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

run_tests_for() {
	version_family="$1"
	python_version="$2"
	PYTHONPATH="dist/py-${version_family}/py${PYTHONPATH:+:$PYTHONPATH}"

	if output=$(PYTHONPATH="$PYTHONPATH" uv run --python $python_version python test.py 2>&1); then
		echo "🟢 Python $python_version: OK"
	else
		echo "🔴 Python $python_version: FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
}

build_for "latest"
for version in "${LATEST_VERSIONS[@]}"; do
	run_tests_for "latest" "$version"
done

echo

build_for "legacy"
for version in "${LEGACY_VERSIONS[@]}"; do
	run_tests_for "legacy" "$version"
done

echo
