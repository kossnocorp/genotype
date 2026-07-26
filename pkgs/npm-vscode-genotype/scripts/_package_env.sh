#!/usr/bin/env bash

# This script provides environment variables for extension building & publishing.
#
# Usage:
#     source "$(dirname "$0")/_package_env.sh"

set -eo pipefail

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
	echo "🔴 This script is meant to be sourced, not executed!"
	exit 1
fi

# Make sure mise is activated
eval "$(mise activate bash --shims)"
eval "$(mise env -s bash)"

# Provide base variables
set_vars() {
	local script_path="$0"
	root_dir="$(dirname "$0")/.."
	package_version=$(cat package.json | jaq -r '.version')
	package_name=$(cat package.json | jaq -r '.name')
	package_publisher=$(cat package.json | jaq -r '.publisher')
	package_dist_dir="$(pwd)/dist/production"
	package_filename="$package_name-$package_version.vsix"
	package_path="$package_dist_dir/$package_filename"
}

set_vars
