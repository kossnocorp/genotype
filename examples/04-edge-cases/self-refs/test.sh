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
	PYTHONPATH="dist/py-${version_family}/py${PYTHONPATH:+:$PYTHONPATH}"

	if output=$(PYTHONPATH="$PYTHONPATH" uv run --python "$python_version" python test.py 2>&1); then
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
for version in "${LATEST_VERSIONS[@]}"; do
	run_python_tests_for "latest" "$version"
done

echo

# TODO: Fix self-refs for Python 3.11 and below
# build_for_python "legacy"
# for version in "${LEGACY_VERSIONS[@]}"; do
# 	run_python_tests_for "legacy" "$version"
# done
echo "🟡 Unimplemented for legacy Python, skipping..."

#endregion

#region TypeScript

echo
echo "🌀 Building TypeScript Zod schemas"
if output=$(cargo run -p genotype_cli --bin gt -- build . --config genotype.ts-zod.toml 2>&1); then
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

#endregion
