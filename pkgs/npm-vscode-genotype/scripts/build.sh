#!/usr/bin/env bash

# This script builds the extension for Visual Studio Code.

set -euo pipefail

PKG_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
cd "$PKG_DIR"

DIST_DIR="$PKG_DIR/dist/production"

echo -e "🚧 Building Visual Studio Code extension..."

rm -rf "$DIST_DIR"
mkdir "$DIST_DIR"

echo -e "\n🌀 Building extension bundle..."
if ! output=$(pnpm exec vite build 2>&1); then

	echo "🔴 vite: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

echo "🟢 Bundle built"

echo -e "\n🌀 Copying assets..."

if ! output=$(rsync -av \
	--include='package.json' \
	--include='language-configuration.json' \
	--include='*.md' \
	--exclude='*' \
	. "$DIST_DIR/" 2>&1); then
	echo "🔴 rsync: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

SRC_SYNTAX_PATH="$PKG_DIR/node_modules/genotype-tm-grammar/genotype.json"
DIST_SYNTAX_PATH="$DIST_DIR/syntaxes/genotype.tmLanguage.json"
mkdir -p "$(dirname "$DIST_SYNTAX_PATH")"
if ! output=$(cp "$SRC_SYNTAX_PATH" "$DIST_SYNTAX_PATH" 2>&1); then
	echo "🔴 cp: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

echo "🟢 Assets copied"

echo -e "\n🌀 Patching package.json..."

TMP_PACKAGE_PATH="$DIST_DIR$(mktemp)"
mkdir -p "$(dirname "$TMP_PACKAGE_PATH")"

if ! output=$(
	jaq '
  .main = "./extension.js"
  | .contributes.grammars |= map(
      if .language == "genotype"
      then .path = "./syntaxes/genotype.tmLanguage.json"
      else .
      end
    )
' "$DIST_DIR"/package.json >"$TMP_PACKAGE_PATH" && mv "$TMP_PACKAGE_PATH" "$DIST_DIR"/package.json
	2>&1
); then
	echo "🔴 jaq: FAILED"
	echo "--- Output ------------------------------------------"
	echo "$output"
	echo "-----------------------------------------------------"
	exit 1
fi

echo "🟢 package.json patched"

echo -e "\n🎉 Extension build is ready!"
