#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EDGE_CASES_DIR="$ROOT_DIR/examples/04-edge-cases"

shopt -s nullglob
example_dirs=("$EDGE_CASES_DIR"/*)

passed_examples=0
failed_examples=0

for example_dir in "${example_dirs[@]}"; do
	if [[ ! -d "$example_dir" || ! -f "$example_dir/test.sh" ]]; then
		continue
	fi
	rel_example_dir="${example_dir#"$ROOT_DIR"/}"

	echo -e "=== 🚧 Running $rel_example_dir ===\n"

	example_failed=0

	if (cd "$example_dir" && ./test.sh); then
		echo "🟢 Tests: OK"
		passed_examples=$((passed_examples + 1))
	else
		echo "🔴 Tests: FAILED"
		failed_examples=$((failed_examples + 1))
	fi

	echo -e "\n=====================================================\n"
done

status_emoji="✅"
if [[ "$failed_examples" -gt 0 ]]; then
	status_emoji="🚨"
fi

echo "$status_emoji $passed_examples passed, $failed_examples failed"

if [[ "$failed_examples" -gt 0 ]]; then
	exit 1
fi
