#!/usr/bin/env bash

# This script links the syntax files from root examples to local tests/syntax dir.

set -euo pipefail

PKG_DIR="$(dirname "${BASH_SOURCE[0]}")/.."

cd "$PKG_DIR/tests/syntax"

# Clean up existing links
find . -type l -delete

# Link syntax files from root examples
ls -d ../../../../examples/02-syntax/*.type | while read -r SYNTAX_FILE_PATH; do
	SYNTAX_FILE_NAME="$(basename "$SYNTAX_FILE_PATH")"
	echo "$SYNTAX_FILE_PATH -> $SYNTAX_FILE_NAME"
	ln -sf "$SYNTAX_FILE_PATH" "$SYNTAX_FILE_NAME"
done
