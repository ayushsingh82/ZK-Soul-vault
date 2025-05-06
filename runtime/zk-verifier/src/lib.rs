#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch::DispatchResult,
    traits::{Get, Time},
};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct ZkProof {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<Vec<u8>>,
}

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Time: Time;
}

decl_storage! {
    trait Store for Module<T: Config> as ZkVerifier {
        VerifyingKeys get(fn verifying_key): map hasher(blake2_128_concat) Vec<u8> => Vec<u8>;
        VerifiedProofs get(fn verified_proof): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) Vec<u8> => bool;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        ProofVerified(AccountId, Vec<u8>),
        VerifyingKeySet(Vec<u8>),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        InvalidProof,
        VerifyingKeyNotFound,
        ProofAlreadyVerified,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        type Event = Event<T>;

        #[weight = 10_000]
        pub fn verify_proof(
            origin,
            proof: ZkProof,
            circuit_id: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Get the verifying key for the circuit
            let vk_bytes = Self::verifying_key(circuit_id.clone())
                .ok_or(Error::<T>::VerifyingKeyNotFound)?;

            // Deserialize the verifying key
            let vk = VerifyingKey::<Bls12_381>::deserialize(&vk_bytes[..])
                .map_err(|_| Error::<T>::InvalidProof)?;

            // Deserialize the proof
            let proof = Proof::<Bls12_381>::deserialize(&proof.proof[..])
                .map_err(|_| Error::<T>::InvalidProof)?;

            // Convert public inputs to field elements
            let public_inputs: Vec<Fr> = proof.public_inputs
                .iter()
                .map(|input| Fr::deserialize(&input[..]))
                .collect::<Result<_, _>>()
                .map_err(|_| Error::<T>::InvalidProof)?;

            // Verify the proof
            if !ark_groth16::verify_proof(&vk, &proof, &public_inputs)
                .map_err(|_| Error::<T>::InvalidProof)? {
                return Err(Error::<T>::InvalidProof.into());
            }

            // Store the verification result
            VerifiedProofs::insert(who.clone(), circuit_id.clone(), true);

            // Emit the event
            Self::deposit_event(Event::ProofVerified(who, circuit_id));

            Ok(())
        }

        #[weight = 10_000]
        pub fn set_verifying_key(
            origin,
            circuit_id: Vec<u8>,
            vk: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            // Store the verifying key
            VerifyingKeys::insert(circuit_id.clone(), vk);

            // Emit the event
            Self::deposit_event(Event::VerifyingKeySet(circuit_id));

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_ok, assert_noop,
        parameter_types,
        traits::{OnFinalize, OnInitialize},
    };
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Module, Call, Config, Storage, Event<T>},
            ZkVerifier: Module<Test>,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: u32 = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = ();
        type BlockWeights = ();
        type BlockLength = ();
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type DbWeight = ();
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
    }

    impl Config for Test {
        type Event = Event;
        type Time = ();
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn test_verify_proof() {
        new_test_ext().execute_with(|| {
            // TODO: Add test implementation
        });
    }
} 