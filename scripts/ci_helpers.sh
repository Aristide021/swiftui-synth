#!/bin/bash

# CI/CD Helper Script for SwiftUI Layout Synthesizer
# Usage: ./scripts/ci_helpers.sh [command]

set -e

# Ensure we're in the project root
cd "$(dirname "$0")/.."

function test_all() {
    echo "Running all tests..."
    cargo test
    echo "✅ Tests passed"
}

function lint() {
    echo "Checking formatting..."
    cargo fmt --check
    echo "Running Clippy..."
    cargo clippy -- -D warnings
    echo "✅ Lint checks passed"
}

function build_release() {
    echo "Building release binary..."
    cargo build --release
    echo "✅ Release binary built at target/release/swiftui-synth"
}

function local_release_test() {
    echo "Testing release process locally..."
    
    # Build release binary
    build_release
    
    # Run a simple test
    ./target/release/swiftui-synth --examples "{(width:390,height:844):{title:\"Test\"}}" | grep "VStack"
    
    if [ $? -eq 0 ]; then
        echo "✅ Release test passed"
    else
        echo "❌ Release test failed"
        exit 1
    fi
}

function bump_version() {
    if [ -z "$1" ]; then
        echo "Usage: $0 bump_version [major|minor|patch]"
        exit 1
    fi
    
    # Get current version
    CURRENT_VERSION=$(grep -oP '^version = "\K[^"]+' Cargo.toml)
    IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
    MAJOR=${VERSION_PARTS[0]}
    MINOR=${VERSION_PARTS[1]}
    PATCH=${VERSION_PARTS[2]}
    
    # Calculate new version
    case "$1" in
        major)
            MAJOR=$((MAJOR + 1))
            MINOR=0
            PATCH=0
            ;;
        minor)
            MINOR=$((MINOR + 1))
            PATCH=0
            ;;
        patch)
            PATCH=$((PATCH + 1))
            ;;
        *)
            echo "Invalid version bump type. Use major, minor, or patch."
            exit 1
            ;;
    esac
    
    NEW_VERSION="$MAJOR.$MINOR.$PATCH"
    
    # Update version in Cargo.toml
    sed -i '' "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
    
    echo "Version bumped from $CURRENT_VERSION to $NEW_VERSION"
    echo "Don't forget to commit the change with:"
    echo "git commit -am \"chore(release): bump version to $NEW_VERSION\""
}

function ci_all() {
    test_all
    lint
    build_release
    local_release_test
    echo "✅ All CI checks passed"
}

function help() {
    echo "SwiftUI Layout Synthesizer CI Helper"
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  test        Run all tests"
    echo "  lint        Run formatting and linting checks"
    echo "  build       Build release binary"
    echo "  release     Test release process locally"
    echo "  bump        Bump version (usage: $0 bump [major|minor|patch])"
    echo "  ci          Run all CI checks"
    echo "  help        Show this help message"
}

# Main command router
case "$1" in
    test)
        test_all
        ;;
    lint)
        lint
        ;;
    build)
        build_release
        ;;
    release)
        local_release_test
        ;;
    bump)
        bump_version "$2"
        ;;
    ci)
        ci_all
        ;;
    help|*)
        help
        ;;
esac 