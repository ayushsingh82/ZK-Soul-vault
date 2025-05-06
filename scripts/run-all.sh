#!/bin/bash

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for required tools
if ! command_exists cargo; then
    echo "Error: cargo is not installed. Please install Rust first."
    exit 1
fi

if ! command_exists cargo-contract; then
    echo "Installing cargo-contract..."
    cargo install cargo-contract --force
fi

# Build all components
echo "Building all components..."
./scripts/build-all.sh

# Run age verification
echo "Running age verification..."
cd circuits/age-verification
cargo test
cd ../..

# Run KYC verification
echo "Running KYC verification..."
cd circuits/kyc-verification
cargo test
cargo run --example generate_proof
cd ../..

# Run smart contract tests
echo "Running smart contract tests..."
cd contracts/identity-vault
cargo contract test
cd ../..

# Run runtime tests
echo "Running runtime tests..."
cd runtime/zk-verifier
cargo test
cd ../..

echo "All components have been built and tested successfully!" 