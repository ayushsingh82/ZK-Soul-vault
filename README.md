# ZK-Soul Vault

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

## ZK Circuits

### KYC Verification Circuit

The KYC verification circuit (`circuits/kyc-verification`) is responsible for verifying KYC documents and signatures. Here's how it works:

1. **Inputs**:
   - `document_hash`: Hash of the KYC document
   - `signature`: Digital signature of the document
   - `public_key`: Public key of the signer
   - `verification_result`: Boolean indicating if verification passed

2. **Circuit Implementation**:
   - Uses arkworks libraries (ark-bls12-381, ark-groth16)
   - Converts inputs to field elements
   - Verifies digital signatures
   - Generates zero-knowledge proofs

3. **Usage**:
```rust
// Create the circuit
let circuit = KYCVerificationCircuit {
    document_hash: Some(vec![1, 2, 3, 4]),
    signature: Some(vec![5, 6, 7, 8]),
    public_key: Some(vec![9, 10, 11, 12]),
    verification_result: Some(true),
};

// Generate proof
let params = generate_random_parameters::<Bls12_381, _, _>(circuit.clone(), rng).unwrap();
let proof = create_random_proof(circuit, &params, rng).unwrap();
```

### Age Verification Circuit

The age verification circuit (`circuits/age-verification`) verifies age-related claims. Here's how it works:

1. **Inputs**:
   - `birth_date`: Date of birth
   - `current_date`: Current date
   - `minimum_age`: Minimum age requirement
   - `verification_result`: Boolean indicating if age requirement is met

2. **Circuit Implementation**:
   - Uses arkworks libraries
   - Converts dates to field elements
   - Performs age comparison
   - Generates zero-knowledge proofs

3. **Usage**:
```rust
// Create the circuit
let circuit = AgeVerificationCircuit {
    birth_date: Some(vec![1990, 1, 1]),
    current_date: Some(vec![2024, 3, 15]),
    minimum_age: Some(18),
    verification_result: Some(true),
};

// Generate proof
let params = generate_random_parameters::<Bls12_381, _, _>(circuit.clone(), rng).unwrap();
let proof = create_random_proof(circuit, &params, rng).unwrap();
```

## Runtime Module

The ZK verifier runtime module (`runtime/zk-verifier`) is responsible for verifying ZK proofs on-chain. Here's how it works:

1. **Storage**:
   - `VerifyingKeys`: Maps circuit IDs to their verifying keys
   - `VerifiedProofs`: Tracks which proofs have been verified

2. **Functions**:
   - `verify_proof`: Verifies a ZK proof using the stored verifying key
   - `set_verifying_key`: Sets the verifying key for a circuit

3. **Usage**:
```rust
// Set verifying key
zk_verifier::Module::<T>::set_verifying_key(
    origin,
    circuit_id,
    vk_bytes,
)?;

// Verify proof
zk_verifier::Module::<T>::verify_proof(
    origin,
    proof,
    circuit_id,
)?;
```

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
