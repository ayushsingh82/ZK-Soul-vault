# ZK-ID Vault

A ZK-verified on-chain identity vault built on Polkadot using PolkaVM. This project combines smart contracts and ZK runtime development to create a secure and private identity management system.

## Features

- ZK-verified identity attestations
- Off-chain credential verification
- On-chain attestation storage
- Custom runtime module for ZK proof verification
- React frontend for user interaction

## Prerequisites

- Rust and Cargo
- Node.js and npm
- Polkadot.js
- ink! CLI
- Substrate development environment

## Setup

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install ink! CLI:
```bash
cargo install cargo-contract --force
```

3. Install Node.js dependencies:
```bash
cd frontend
npm install
```

## Running the Project

### Smart Contracts

1. Build the contracts:
```bash
cd contracts/identity-vault
cargo contract build
cd ../attestation
cargo contract build
```

2. Deploy to Westend:
```bash
cargo contract instantiate --suri //Alice --constructor new --args "initial_value"
```

### Runtime Module

1. Build the runtime:
```bash
cd runtime/zk-verifier
cargo build --release
```

2. Run the node:
```bash
./target/release/zk-id-vault-node --dev
```

### Frontend

1. Start the development server:
```bash
cd frontend
npm start
```

## Project Structure

- `contracts/`: ink! smart contracts
- `circuits/`: ZK circuits for verification
- `runtime/`: Custom runtime module
- `frontend/`: React + Polkadot.js frontend
- `scripts/`: Deployment and testing scripts

## Development

1. Smart Contract Development:
   - Use `cargo contract build` to build contracts
   - Use `cargo contract test` to run tests

2. ZK Circuit Development:
   - Circuits are built using arkworks
   - Use `cargo test` in the circuits directory

3. Runtime Development:
   - Use `cargo build` to build the runtime
   - Use `cargo test` to run tests

## Testing

Run all tests:
```bash
cargo test --all
```

## License

MIT 