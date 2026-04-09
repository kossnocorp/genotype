#!/usr/bin/env bash

# This script builds and packages the VS Code extension.

set -euo pipefail

PKG_DIR="$(dirname "${BASH_SOURCE[0]}")/.."

cd "$PKG_DIR"

echo -e "🚧 Packaging VS Code extension"

echo -e "\n🌀 Building VS Code extension..."
mise :build
echo "🟢 Built successfully"

echo -e "\n🌀 Packaging vsix..."
cd dist/production
pnpm dlx \
	--allow-build=@vscode/vsce-sign \
	--allow-build=keytar \
	@vscode/vsce pack \
	--no-dependencies \
	--baseContentUrl https://github.com/kossnocorp/genotype/tree/HEAD/pkgs/npm-vscode-genotype \
	--baseImagesUrl https://github.com/kossnocorp/genotype/raw/HEAD/pkgs/npm-vscode-genotype
echo "🟢 Packaged successfully"

echo -e "\n🎉 Extension package is ready!"
