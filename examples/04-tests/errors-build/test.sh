#!/usr/bin/env bash

set -euo pipefail

SNAP_PATH="./output.actual.snap"
EXPECTED_SNAP_PATH="./output.snap"
SOME_FAILED=0

UPDATE=0
DEBUG=0
RUN=0

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

cd modules

rm -rf ./dist

if [ $RUN -eq 1 ]; then
	echo "🌀 Running \`gt build\`"
	echo "-----------------------------------------------------"
	"$CLI_PATH" build .
	exit 0
fi

echo "🌀 Checking module build errors"

if output=$("$CLI_PATH" build . 2>&1); then
	echo "🔴 Module errors: NONE"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
else
	if [ "$UPDATE" -eq 1 ]; then
		echo "🟠 Module errors: Updating snapshot with new output."
		printf '%s\n' "$output" >"$EXPECTED_SNAP_PATH"
		exit 0
	else
		printf '%s\n' "$output" >"$SNAP_PATH"

		if [ ! -f "$EXPECTED_SNAP_PATH" ]; then
			echo "🟡 Module errors: Snapshot missing; run with '--update' to save the output."
			echo "--- Output ------------------------------------------"
			echo "$output"
			echo "-----------------------------------------------------"
			SOME_FAILED=1
		else
			if diff_output=$(diff -u --color=always "$EXPECTED_SNAP_PATH" "$SNAP_PATH" 2>&1); then
				echo "🟢 Module errors: OK"
				if [ $DEBUG -eq 1 ]; then
					echo "--- Output ------------------------------------------"
					echo "$output"
					echo "-----------------------------------------------------"
				fi
			else
				echo "🔴 Module errors: Snapshots mismatch; run with '--update' to accept the new snapshot."
				echo "--- Diff --------------------------------------------"
				echo -e "$diff_output"
				echo "-----------------------------------------------------"
				SOME_FAILED=1
			fi
		fi

		rm -f "$SNAP_PATH"
	fi
fi
cd - >/dev/null

echo
if [ $SOME_FAILED -eq 1 ]; then
	echo "🔴 Some tests failed"
	exit 1
else
	echo "🟢 All tests passed"
	exit 0
fi
