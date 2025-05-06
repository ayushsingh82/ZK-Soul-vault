#!/bin/bash

# Build smart contracts
echo "Building identity vault contract..."
cd contracts/identity-vault
cargo contract build
cd ../..

echo "Building attestation contract..."
cd contracts/attestation
cargo contract build
cd ../..

# Build ZK circuits
echo "Building age verification circuit..."
cd circuits/age-verification
cargo build
cd ../..

echo "Building KYC verification circuit..."
cd circuits/kyc-verification
cargo build
cd ../..

# Build runtime
echo "Building ZK verifier runtime..."
cd runtime/zk-verifier
cargo build --release
cd ../..

echo "All components built successfully!" 