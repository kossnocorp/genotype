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
	profile="$1"
	echo -e "🌀 Building for $profile"
	if output=$(cargo run -p genotype_cli --bin gt -- build . --config "genotype.${profile}.toml" 2>&1); then
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
	profile="$1"
	python_version="$2"
	PYTHONPATH="dist/${profile}/py${PYTHONPATH:+:$PYTHONPATH}"

	if output=$(PYTHONPATH="$PYTHONPATH" uv run --python $python_version python test.py "$profile" 2>&1); then
		echo "🟢 Python $python_version: OK"
	else
		echo "🔴 Python $python_version: FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
}

run_profile() {
	profile="$1"
	version_family="$2"

	build_for "$profile"

	if [[ "$version_family" == "latest" ]]; then
		for version in "${LATEST_VERSIONS[@]}"; do
			run_tests_for "$profile" "$version"
		done
	else
		for version in "${LEGACY_VERSIONS[@]}"; do
			run_tests_for "$profile" "$version"
		done
	fi

	echo
}

run_profile "py-latest-uv" "latest"
run_profile "py-legacy-uv" "legacy"
run_profile "py-latest-poetry" "latest"
run_profile "py-legacy-poetry" "legacy"
