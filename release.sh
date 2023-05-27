#!/bin/bash

VERSION=$1

# Check that we are on main branch
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$BRANCH" != "main" ]]; then
  echo 'Not on branch main!';
  exit 1;
fi

# Verify that there are no active changes
if [[ `git status --porcelain` ]]; then
    echo 'There are changes in the repo! Please stash or commit them';
    exit 1;
fi

cargo v $VERSION
