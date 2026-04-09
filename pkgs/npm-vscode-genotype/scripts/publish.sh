#!/usr/bin/env bash

# This script publishes the VS Code extension to Visual Studio Marketplace
# and Open VSX Registry.

set -euo pipefail

PKG_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
VERSION="$(cat "$PKG_DIR/package.json" | jaq -r .version)"
PACKAGE_PATH="$PKG_DIR/dist/production/genotype-$VERSION.vsix"

cd "$PKG_DIR"

echo -e "🚧 Publishing VS Code extension $VERSION"

mise :build/package

echo -e "\n🌀 Publishing to Visual Studio Marketplace..."

if ! output=$(fnox exec -- \
	pnpm vsce publish --packagePath "$PACKAGE_PATH" 2>&1); then
	echo "🔴 vsce: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi
echo "🟢 Published successfully"

# TODO: Claim Open VSX namespace and configure token

# echo "🌀 Publishing to Open VSX Registry..."

# if ! output=$(fnox exec -- \
# 	pnpm ovsx publish "$PACKAGE_PATH" 2>&1); then
# 	echo "🔴 ovsx: FAILED"
# 	echo "--- Output ------------------------------------------"
# 	echo "$output"
# 	echo "-----------------------------------------------------"
# 	exit 1
# fi

echo -e "\n🎉 Extension shipped!"
