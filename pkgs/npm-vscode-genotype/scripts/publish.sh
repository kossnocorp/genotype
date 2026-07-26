#!/usr/bin/env bash

# This script publishes the VS Code extension to Visual Studio Marketplace
# and Open VSX Registry.

source "$(dirname "$0")/_package_env.sh"

cd "$root_dir"

echo -e "🚧 Publishing VS Code extension v$package_version"

echo
echo "╔═══════════════════════════════════════════════════╗"
./scripts/package.sh
echo "╚═══════════════════════════════════════════════════╝"

echo
echo "🌀 Publishing to Visual Studio Marketplace..."

if ! output=$(fnox exec -- \
	pnpm vsce publish \
	--packagePath "$package_path" \
	--skip-duplicate \
	2>&1); then

	echo "╭─ 🔴 Failed ──────────────────────────────────────╮"
	echo "$output"
	echo "╰───────────────────────────────────────────────────╯"
	exit 1
fi
echo "🟢 Published"

echo

echo "🌀 Publishing to Open VSX Registry..."

if ! output=$(fnox exec -- \
	pnpm ovsx publish "$package_path" 2>&1); then

	echo "╭─ 🔴 Failed ──────────────────────────────────────╮"
	echo "$output"
	echo "╰───────────────────────────────────────────────────╯"
	exit 1
fi
echo "🟢 Published"

echo -e "\n🎉 Extension shipped!"
