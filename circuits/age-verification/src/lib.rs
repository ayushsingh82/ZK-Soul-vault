use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_r1cs_std::prelude::*;
use ark_std::rand::RngCore;

#[derive(Clone)]
pub struct AgeVerificationCircuit {
    pub birth_year: Option<u32>,
    pub current_year: Option<u32>,
    pub minimum_age: Option<u32>,
}

impl ConstraintSynthesizer<ark_bls12_381::Fr> for AgeVerificationCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ark_bls12_381::Fr>,
    ) -> Result<(), SynthesisError> {
        // Allocate the birth year as a private input
        let birth_year = UInt32::new_witness(cs.clone(), || {
            self.birth_year.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate the current year as a public input
        let current_year = UInt32::new_input(cs.clone(), || {
            self.current_year.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate the minimum age as a public input
        let minimum_age = UInt32::new_input(cs.clone(), || {
            self.minimum_age.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Calculate age
        let age = current_year.sub(&birth_year)?;

        // Enforce that age >= minimum_age
        age.enforce_cmp(&minimum_age, std::cmp::Ordering::Greater, false)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_groth16::{create_random_proof, generate_random_parameters, verify_proof};
    use ark_std::rand::thread_rng;

    #[test]
    fn test_age_verification() {
        let rng = &mut thread_rng();

        // Create the circuit
        let circuit = AgeVerificationCircuit {
            birth_year: Some(1990),
            current_year: Some(2024),
            minimum_age: Some(18),
        };

        // Generate the proving and verification keys
        let params = generate_random_parameters::<Bls12_381, _, _>(circuit.clone(), rng).unwrap();

        // Create the proof
        let proof = create_random_proof(circuit, &params, rng).unwrap();

        // Verify the proof
        let pvk = params.vk.clone();
        let is_valid = verify_proof(&pvk, &proof, &[Fr::from(2024), Fr::from(18)]).unwrap();
        assert!(is_valid);
    }
} 