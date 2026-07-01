#!/usr/bin/env bash

set -euo pipefail

variants=(
	"unformatted"
	"formatted"
)

echo "🌀 Building Genotype"
for variant in "${variants[@]}"; do
	if output=$(cargo run -p genotype_cli --bin gt -- build . --config "genotype.${variant}.toml" 2>&1); then
		echo "🟢 Build for ${variant}: OK"
	else
		echo "🔴 Build for ${variant}: FAILED"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		exit 1
	fi
done

failed=0

check_formatter() {
	local variant=$1
	local formatter=$2
	local path=$3
	shift 3

	local output
	local check_passed=0
	if output=$(cd "$path" && "$@" 2>&1); then
		check_passed=1
	fi

	if [[ "$variant" == "unformatted" ]]; then
		if [[ $check_passed -eq 0 ]]; then
			echo "🟢 ${formatter} for ${variant}: OK (failed as expected)"
			return
		fi

		echo "🔴 ${formatter} for ${variant}: FAILED (passed unexpectedly)"
	else
		if [[ $check_passed -eq 1 ]]; then
			echo "🟢 ${formatter} for ${variant}: OK"
			return
		fi

		echo "🔴 ${formatter} for ${variant}: FAILED"
	fi

	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	failed=1
}

echo
echo "🌀 Running oxfmt checks"
for variant in "${variants[@]}"; do
	check_formatter "$variant" "oxfmt" "dist/${variant}" pnpm oxfmt --check
done

echo
echo "🌀 Running cargo fmt checks"
for variant in "${variants[@]}"; do
	check_formatter "$variant" "cargo fmt" "dist/${variant}/rs" cargo fmt --all --check
done

echo
echo "🌀 Running ruff checks"
for variant in "${variants[@]}"; do
	check_formatter "$variant" "ruff" "dist/${variant}/py" uv run ruff format --check
done

echo

if [[ $failed -eq 0 ]]; then
	echo "🟢 All formatter checks passed"
	exit 0
else
	echo "🔴 Some formatter checks failed"
	exit 1
fi
