#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::vec::Vec;
use ink_storage::traits::SpreadAllocate;

#[ink::contract]
mod identity_vault {
    use super::*;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Attestation {
        credential_type: Vec<u8>,
        proof: Vec<u8>,
        timestamp: u64,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct IdentityVault {
        owner: AccountId,
        attestations: ink_storage::Mapping<AccountId, Vec<Attestation>>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAuthorized,
        InvalidProof,
        AttestationNotFound,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl IdentityVault {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = Self::env().caller();
            })
        }

        #[ink(message)]
        pub fn add_attestation(
            &mut self,
            account: AccountId,
            credential_type: Vec<u8>,
            proof: Vec<u8>,
        ) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotAuthorized);
            }

            // TODO: Verify ZK proof here
            if !self.verify_proof(&proof) {
                return Err(Error::InvalidProof);
            }

            let attestation = Attestation {
                credential_type,
                proof,
                timestamp: self.env().block_timestamp(),
            };

            let mut user_attestations = self.attestations.get(account).unwrap_or_default();
            user_attestations.push(attestation);
            self.attestations.insert(account, &user_attestations);

            Ok(())
        }

        #[ink(message)]
        pub fn get_attestations(&self, account: AccountId) -> Vec<Attestation> {
            self.attestations.get(account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn verify_attestation(
            &self,
            account: AccountId,
            credential_type: Vec<u8>,
        ) -> Result<bool> {
            let attestations = self.attestations.get(account).ok_or(Error::AttestationNotFound)?;
            
            for attestation in attestations.iter() {
                if attestation.credential_type == credential_type {
                    return Ok(true);
                }
            }
            
            Ok(false)
        }

        fn verify_proof(&self, proof: &[u8]) -> bool {
            // TODO: Implement ZK proof verification
            // This is a placeholder that always returns true
            // In production, this should verify the ZK proof using the runtime module
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = IdentityVault::new();
            assert_eq!(contract.owner, AccountId::from([0x1; 32]));
        }

        #[ink::test]
        fn add_attestation_works() {
            let mut contract = IdentityVault::new();
            let account = AccountId::from([0x2; 32]);
            let credential_type = b"age".to_vec();
            let proof = b"proof".to_vec();

            assert_eq!(
                contract.add_attestation(account, credential_type.clone(), proof.clone()),
                Ok(())
            );

            let attestations = contract.get_attestations(account);
            assert_eq!(attestations.len(), 1);
            assert_eq!(attestations[0].credential_type, credential_type);
            assert_eq!(attestations[0].proof, proof);
        }
    }
} 