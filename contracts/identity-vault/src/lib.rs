#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod identity_vault {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};
    use scale_info::TypeInfo;

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    pub struct Attestation {
        credential_type: Vec<u8>,
        proof: Vec<u8>,
        timestamp: u64,
    }

    #[ink(storage)]
    pub struct IdentityVault {
        owner: AccountId,
        attestations: Mapping<AccountId, Vec<Attestation>>,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    #[allow(clippy::cast_possible_truncation)]
    pub enum Error {
        NotAuthorized,
        InvalidProof,
        AttestationNotFound,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Default for IdentityVault {
        fn default() -> Self {
            Self::new()
        }
    }

    impl IdentityVault {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                attestations: Mapping::default(),
            }
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

        fn verify_proof(&self, _proof: &[u8]) -> bool {
            // TODO: Implement ZK proof verification
            // This is a placeholder that always returns true
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = IdentityVault::new();
            assert_eq!(contract.owner, ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice);
        }

        #[ink::test]
        fn add_attestation_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = IdentityVault::new();
            
            let credential_type = b"age".to_vec();
            let proof = b"proof".to_vec();

            assert_eq!(
                contract.add_attestation(accounts.bob, credential_type.clone(), proof.clone()),
                Ok(())
            );

            let attestations = contract.get_attestations(accounts.bob);
            assert_eq!(attestations.len(), 1);
            assert_eq!(attestations[0].credential_type, credential_type);
            assert_eq!(attestations[0].proof, proof);
        }
    }
} 