#!/usr/bin/env bash

set -euo pipefail

script_path="${BASH_SOURCE[0]}"
script_dir="$(cd "$(dirname "$script_path")" && pwd)"
cd "$script_dir"

SOME_FAILED=0

UPDATE=0
DEBUG=0
RUN=0
TERM_WIDTH=60

for arg in "$@"; do
	case "$arg" in
	--update | -u)
		UPDATE=1
		;;
	--debug | -d)
		DEBUG=1
		;;
	--run | -r)
		RUN=1
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

if [ $RUN -eq 1 ]; then
	echo "🔵 Run mode: ON"
else
	echo "🔵 Run mode: OFF"
fi

echo

echo "🌀 Building CLI"
if build_output=$(cargo build -p genotype_cli --bin gt 2>&1); then
	echo "🟢 CLI: OK"
else
	echo "🔴 CLI: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$build_output"
	echo "-----------------------------------------------------"
	exit 1
fi

CLI_PATH="$(realpath "../../../target/debug/gt")"
echo "🔵 CLI path: $CLI_PATH"

echo

for project_path in */; do
	if [ ! -f "$project_path/genotype.toml" ]; then
		continue
	fi

	project_path="${project_path%/}"
	project_name="$(basename "$project_path")"
	snap_path="$project_path/output.actual.snap"
	expected_snap_path="$project_path/output.snap"

	rm -rf "$project_path/dist"

	echo "🌀 Checking $project_name build errors"

	if [ $RUN -eq 1 ]; then
		(
			cd "$project_path"
			GT_TERM_WIDTH="$TERM_WIDTH" "$CLI_PATH" build .
		) || SOME_FAILED=1
		echo
		continue
	fi

	if output=$(cd "$project_path" && GT_TERM_WIDTH="$TERM_WIDTH" "$CLI_PATH" build . 2>&1); then
		echo "🔴 $project_name errors: NONE"
		echo "--- Output ------------------------------------------"
		echo "$output"
		echo "-----------------------------------------------------"
		SOME_FAILED=1
	elif [ "$UPDATE" -eq 1 ]; then
		echo "🟠 $project_name errors: Updating snapshot with new output."
		printf '%s\n' "$output" >"$expected_snap_path"
	else
		printf '%s\n' "$output" >"$snap_path"

		if [ ! -f "$expected_snap_path" ]; then
			echo "🟡 $project_name errors: Snapshot missing; run with '--update' to save the output."
			echo "--- Output ------------------------------------------"
			echo "$output"
			echo "-----------------------------------------------------"
			SOME_FAILED=1
		elif diff_output=$(diff -u --color=always "$expected_snap_path" "$snap_path" 2>&1); then
			echo "🟢 $project_name errors: OK"
			if [ $DEBUG -eq 1 ]; then
				echo "--- Output ------------------------------------------"
				echo "$output"
				echo "-----------------------------------------------------"
			fi
		else
			echo "🔴 $project_name errors: Snapshots mismatch; run with '--update' to accept the new snapshot."
			echo "--- Diff --------------------------------------------"
			echo -e "$diff_output"
			echo "-----------------------------------------------------"
			SOME_FAILED=1
		fi

		rm -f "$snap_path"
	fi

	echo
done

echo
if [ $SOME_FAILED -eq 1 ]; then
	echo "🔴 Some tests failed"
	exit 1
else
	echo "🟢 All tests passed"
	exit 0
fi
