#!/usr/bin/env bash

# This script builds and packages the VS Code extension.

source "$(dirname "$0")/_package_env.sh"

echo -e "🚧️ Packaging VS Code extension\n"

cd "$root_dir"

echo "🌀 Building VS Code extension source code..."
if ! output=$(mise build 2>&1); then
	echo "╭─ 🔴 Failed ──────────────────────────────────────╮"
	echo "$output"
	echo "╰───────────────────────────────────────────────────╯"
	exit 1
fi
echo "🟢 Built"

echo
echo "🌀 Packaging vsix..."

# TODO: Figure out how to call `pnpm exec` on a different directory or pass
# dist path to vsce
vsce_path="$(pwd)/$(pnpm -c exec which vsce)"
cd $package_dist_dir
if ! output=$($vsce_path pack \
	--no-dependencies \
	--out $package_filename \
	2>&1); then

	echo "╭─ 🔴 Failed ──────────────────────────────────────╮"
	echo "$output"
	echo "╰───────────────────────────────────────────────────╯"
	exit 1
fi
cd -
echo "🟢 Packaged: $package_path"

echo -e "\n🎉 Extension package is ready!"
