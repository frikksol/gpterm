#!/bin/bash

echo "✨ Creating new release"

# Check that we are on main branch
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$BRANCH" != "main" ]]; then
  echo '🚨 Not on branch main! Aborting.';
  exit 1;
fi

# Verify that there are no active changes
if [[ `git status --porcelain` ]]; then
    echo '🚨 There are changes in the repo! Please stash or commit them';
    exit 1;
fi

# Getting new version number
echo "🔖 Current version is: $(cargo get version)"
echo "Enter new version:"
read NEW_VERSION
echo "👌 Setting version to $NEW_VERSION"

# Set new version and push
git pull
cargo v $NEW_VERSION -y
git push && git push --tags
