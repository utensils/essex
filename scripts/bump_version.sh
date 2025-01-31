#!/bin/bash

set -e

# Ensure a version argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <new-version>"
    echo "Example: $0 1.0.0"
    exit 1
fi

NEW_VERSION="$1"

# Remove 'v' prefix if provided
NEW_VERSION="${NEW_VERSION#v}"

# Validate version format
if ! echo "$NEW_VERSION" | grep -E '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$' > /dev/null; then
    echo "Error: Version must be in format X.Y.Z or X.Y.Z-suffix"
    exit 1
fi

# Update version in Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Create git commit and tag
git add Cargo.toml
git commit -m "chore: bump version to v$NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

echo "Version bumped to v$NEW_VERSION"
echo "Next steps:"
echo "1. Review the changes: git show HEAD"
echo "2. Push the changes: git push origin main v$NEW_VERSION"
