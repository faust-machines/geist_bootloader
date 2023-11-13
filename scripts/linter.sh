#!/bin/bash -e

# Define the path to Cargo.toml relative to this script's location
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "Script directory: $SCRIPT_DIR"
MANIFEST_PATH="$SCRIPT_DIR/../Cargo.toml"
echo "Manifest path: $MANIFEST_PATH"

# Validate that the manifest file actually exists
if [ ! -f "$MANIFEST_PATH" ]; then
    echo "Error: Manifest path does not exist or is not a file."
    exit 1
fi

# Function to print usage
function usage() {
    echo "Usage: $0 [options]"
    echo "  --ci  Run script in CI mode. Will not format code, only checks."
    echo "  -h    Show this help message."
}

# Parse command line arguments
CI_MODE=false
while [ "$1" != "" ]; do
    case $1 in
        --ci ) CI_MODE=true
               ;;
        -h | --help ) usage
                      exit
                      ;;
        * ) usage
            exit 1
    esac
    shift
done

# Run Linter and Formatter
echo "Running Linter..."

if [ "$CI_MODE" = true ]; then
    echo "Running in CI mode. Will only check code style, not format."
    cargo fmt --manifest-path="$MANIFEST_PATH" --all -- --check
else
    echo "Not running in CI mode. Will format code."
    cargo fmt --manifest-path="$MANIFEST_PATH" --all
fi

# Run Clippy for linting
cargo clippy --manifest-path="$MANIFEST_PATH" --all

echo "Linter complete!"