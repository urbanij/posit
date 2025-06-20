#!/bin/bash

set -e

cd py-posit

# Get the version from Cargo.toml
cargo_version=$(grep '^version' Cargo.toml | head -n 1 | sed 's/version = "\(.*\)"/\1/')

# Get the version from pyproject.toml
pyproject_version=$(grep '^version' pyproject.toml | head -n 1 | sed 's/version = "\(.*\)"/\1/')

# Get the latest Git tag
git_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

# Compare Cargo.toml version with pyproject.toml version
if [ "$cargo_version" != "$pyproject_version" ]; then
  echo -e "\033[0;31m✗ Version mismatch between Cargo.toml and pyproject.toml:\033[0m"
  echo "  Cargo.toml version:    $cargo_version"
  echo "  pyproject.toml version: $pyproject_version"
  echo ""
  echo "→ Please update the version in pyproject.toml to match Cargo.toml."
  exit 1
else
  echo -e "\033[0;32m✓ Version in Cargo.toml and pyproject.toml match ($cargo_version)\033[0m"
fi

# Check if the tag is a "release" tag (strict SemVer, no -dev, -rc, etc)
# If Cargo.toml version is a pre-release, skip check
if [[ "$cargo_version" =~ - ]]; then
  echo -e "\033[0;33m⚠ Skipping version check: Cargo.toml version '$cargo_version' is a pre-release.\033[0m"
  exit 0
fi

# Strip leading 'v' if present in Git tag
stripped_git_tag="${git_tag#v}"

# # Compare Cargo.toml version with Git tag
# if [ "$cargo_version" != "$stripped_git_tag" ]; then
#   echo -e "\033[0;31m✗ Version mismatch between Cargo.toml and Git tag:\033[0m"
#   echo "  Cargo.toml version: $cargo_version"
#   echo "  Latest Git tag:     $git_tag"
#   echo ""
#   echo "→ Please update the version in Cargo.toml to match the latest release tag."
#   exit 1
# else
#   echo -e "\033[0;32m✓ Cargo.toml version matches Git tag ($git_tag)\033[0m"
# fi
