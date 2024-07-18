#!/bin/bash

# Check if a version bump type is provided
if [ $# -eq 0 ]; then
    echo "Please provide a version bump type: patch, minor, or major"
    exit 1
fi

# Get the current version from Cargo.toml
current_version=$(grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/')

# Split the version into parts
IFS='.' read -ra version_parts <<< "$current_version"
major=${version_parts[0]}
minor=${version_parts[1]}
patch=${version_parts[2]}

# Bump the version based on the provided type
case $1 in
    patch)
        patch=$((patch + 1))
        ;;
    minor)
        minor=$((minor + 1))
        patch=0
        ;;
    major)
        major=$((major + 1))
        minor=0
        patch=0
        ;;
    *)
        echo "Invalid version bump type. Use patch, minor, or major."
        exit 1
        ;;
esac

# Construct the new version
new_version="$major.$minor.$patch"

# Update Cargo.toml with the new version
# This sed command works on both macOS and Linux
sed -i.bak "s/^version = .*/version = \"$new_version\"/" Cargo.toml && rm Cargo.toml.bak

echo "Version bumped from $current_version to $new_version"