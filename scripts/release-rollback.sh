#!/usr/bin/env bash

# This script roll backs GitHub release by tag. It only deletes the release and tags, but won't
# yank the published packages.

set -euo pipefail

echo "🚧 Rolling back the release $VERSION..."

echo -e "\n🌀 Deleting Git tag and GitHub release..."

if output=$(git tag -d "$VERSION" 2>&1); then
	echo "🟢 Git tag deleted"
else
	echo "🟡 Git tag already removed"
fi

if output=$(gh release delete "$VERSION" --cleanup-tag 2>&1); then
	echo "🟢 GitHub release deleted"
else
	echo "🟡 GitHub release already removed"
fi

recreate=false
[[ " $* " == *" --recreate "* ]] && recreate=true
if $recreate; then
	echo -e "\n🌀 Recreating the release..."
	git tag -a "$VERSION" -m "genotype@$VERSION"
	git push --tags --force
	echo "🟢 Git tag recreated and pushed"

	echo "🎉 Release recreated successfully."
else
	echo -e "\n🎉 Release rolled back successfully. Use --recreate to create it again."
fi
