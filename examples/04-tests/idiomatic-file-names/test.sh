#!/usr/bin/env bash

set -euo pipefail

script_path="${BASH_SOURCE[0]}"
script_dir="$(cd "$(dirname "$script_path")" && pwd)"
cd "$script_dir"

UPDATE=0
DEBUG=0

for arg in "$@"; do
	case "$arg" in
	--update | -u)
		UPDATE=1
		;;
	--debug | -d)
		DEBUG=1
		;;
	*)
		echo "🔴 Unknown argument: $arg"
		exit 1
		;;
	esac
done

if [ $DEBUG -eq 1 ]; then
	echo "🔵 Debug mode: ON"
else
	echo "🔵 Debug mode: OFF"
fi

if [ $UPDATE -eq 1 ]; then
	echo "🔵 Update mode: ON"
else
	echo "🔵 Update mode: OFF"
fi

echo

rm -rf dist

echo "🌀 Building project with idiomatic target file names"
if output=$(cargo run -p genotype_cli --bin gt -- build . 2>&1); then
	echo "🟢 Build: OK"
	if [ $DEBUG -eq 1 ]; then
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
	fi
else
	echo "🔴 Build: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
echo

actual_snap_path="output.actual.snap"
expected_snap_path="output.snap"
tree dist >"$actual_snap_path"

echo "🌀 Checking generated file tree"
if [ $UPDATE -eq 1 ]; then
	mv "$actual_snap_path" "$expected_snap_path"
	echo "🟠 Generated file tree: Updating snapshot."
elif [ ! -f "$expected_snap_path" ]; then
	echo "🟡 Generated file tree: Snapshot missing; run with '--update' to save the output."
	echo "--- Output ------------------------------------------"
	cat "$actual_snap_path"
	echo "-----------------------------------------------------"
	exit 1
elif diff_output=$(diff -u --color=always "$expected_snap_path" "$actual_snap_path" 2>&1); then
	rm "$actual_snap_path"
	echo "🟢 Generated file tree: OK"
	if [ $DEBUG -eq 1 ]; then
		echo "--- Output ------------------------------------------"
		cat "$expected_snap_path"
		echo "-----------------------------------------------------"
	fi
else
	echo "🔴 Generated file tree: Snapshot mismatch; run with '--update' to accept it."
	echo "--- Diff --------------------------------------------"
	echo -e "$diff_output"
	echo "-----------------------------------------------------"
	exit 1
fi
