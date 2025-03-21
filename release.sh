#!/bin/bash

# JanusLens Release Script
# Usage: ./release.sh [patch|minor|major]

set -e

RELEASE_TYPE=${1:-minor}

if [[ "$RELEASE_TYPE" != "patch" && "$RELEASE_TYPE" != "minor" && "$RELEASE_TYPE" != "major" ]]; then
  echo "Invalid release type: $RELEASE_TYPE"
  echo "Usage: ./release.sh [patch|minor|major]"
  exit 1
fi

# Check for uncommitted changes
if [[ $(git status --porcelain) ]]; then
  echo "Error: There are uncommitted changes in the repository."
  echo "Please commit or stash them before releasing."
  exit 1
fi

# Make sure we're on the main branch
CURRENT_BRANCH=$(git symbolic-ref --short HEAD)
if [[ "$CURRENT_BRANCH" != "main" ]]; then
  echo "Error: You must be on the main branch to release."
  echo "Current branch: $CURRENT_BRANCH"
  exit 1
fi

# Get current version from package.json
CURRENT_VERSION=$(node -p "require('./package.json').version")
echo "Current version: $CURRENT_VERSION"

# Calculate new version
npm version $RELEASE_TYPE --no-git-tag-version
NEW_VERSION=$(node -p "require('./package.json').version")
echo "New version: $NEW_VERSION"

# Update version in Cargo.toml
cd src-tauri
sed -i '' "s/version = \".*\"/version = \"$NEW_VERSION\"/g" Cargo.toml
cd ..

# Generate changelog entry
PREVIOUS_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

if [ -z "$PREVIOUS_TAG" ]; then
  echo "# Changes in $NEW_VERSION ($(date +%Y-%m-%d))" > CHANGELOG.new.md
  echo "" >> CHANGELOG.new.md
  echo "Initial release" >> CHANGELOG.new.md
else
  echo "# Changes in $NEW_VERSION ($(date +%Y-%m-%d))" > CHANGELOG.new.md
  echo "" >> CHANGELOG.new.md
  git log ${PREVIOUS_TAG}..HEAD --pretty=format:"- %s" --reverse | grep -v "Merge" >> CHANGELOG.new.md
fi

# Insert new changelog at the top
echo "Generating new CHANGELOG.md"
touch CHANGELOG.md
cat CHANGELOG.new.md <(echo "") <(cat CHANGELOG.md) > CHANGELOG.temp.md
mv CHANGELOG.temp.md CHANGELOG.md
rm CHANGELOG.new.md

# Commit changes
git add package.json src-tauri/Cargo.toml CHANGELOG.md
git commit -m "chore: release v$NEW_VERSION"

# Tag the release
git tag -a "v$NEW_VERSION" -m "Version $NEW_VERSION"

echo ""
echo "✅ Release v$NEW_VERSION prepared!"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git show"
echo "  2. Push the commits: git push"
echo "  3. Push the tag: git push --tags"
echo "  4. Create a release on GitHub with the changelog"
echo ""
echo "To build the release, run: npm run tauri build" 