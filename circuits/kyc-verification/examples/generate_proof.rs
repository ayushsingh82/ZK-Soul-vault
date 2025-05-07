use ark_bls12_381::Bls12_381;
use ark_groth16::{
    generate_random_parameters, create_random_proof,
};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::thread_rng;
use kyc_verification::KYCVerificationCircuit;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let rng = &mut thread_rng();

    // Create the circuit
    let circuit = KYCVerificationCircuit {
        document_hash: Some(vec![1, 2, 3, 4]),
        signature: Some(vec![5, 6, 7, 8]),
        public_key: Some(vec![9, 10, 11, 12]),
        verification_result: Some(true),
    };

    // Generate the proving and verification keys
    println!("Generating parameters...");
    let params = generate_random_parameters::<Bls12_381, _, _>(circuit.clone(), rng).unwrap();

    // Create the proof
    println!("Creating proof...");
    let proof = create_random_proof(circuit, &params, rng).unwrap();

    // Serialize the proof and parameters
    let mut proof_bytes = Vec::new();
    proof.serialize(&mut proof_bytes).unwrap();

    let mut vk_bytes = Vec::new();
    params.vk.serialize(&mut vk_bytes).unwrap();

    // Save to file
    let proof_path = Path::new("kyc_proof.json");
    let mut proof_file = File::create(proof_path).unwrap();
    proof_file.write_all(&proof_bytes).unwrap();

    let vk_path = Path::new("kyc_vk.json");
    let mut vk_file = File::create(vk_path).unwrap();
    vk_file.write_all(&vk_bytes).unwrap();

    println!("Proof and verification key saved to kyc_proof.json and kyc_vk.json");
} 