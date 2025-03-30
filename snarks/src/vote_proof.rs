// import types from libaries and modules
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_groth16::{Groth16, ProvingKey, Proof, VerifyingKey};
use ark_snark::SNARK;
use ark_std::{rand::rngs::OsRng, UniformRand};
use std::time::Instant;
use ark_serialize::CanonicalSerialize;
use crate::{circuit::VotingCircuit, voter_registration::Party};

// vote structure
pub struct SnarkVote {
    pub party: u64, // voted party
    pub nullifier: BlsFr, // identifier
    pub proof: Proof<Bls12_381>,  // zk-SNARK proof
    pub commitment: BlsFr,  // blinding factor commitment
    pub proof_size: usize,  // proof size
    pub gen_time: u128,    // proof generation time
    pub verify_time: u128, // proof verification time
}

// implement vote
impl SnarkVote {
    
    // function to create a new vote
    pub fn new(party: Party, nullifier: BlsFr, pk: &ProvingKey<Bls12_381>) -> Self {

        // variable initialization
        let mut rng = OsRng;
        let blinding = BlsFr::rand(&mut rng);

        // voting circuit initialization
        let circuit = VotingCircuit {
            vote: Some(BlsFr::from(party.as_u64())),
            nullifier_hash: Some(nullifier),
            blinding: Some(blinding),
        };

        // proof generation
        let gen_start = Instant::now();
        let proof = Groth16::<Bls12_381>::prove(pk, circuit, &mut rng)
            .expect("Proof generation failed.");
        let gen_time = gen_start.elapsed().as_nanos();

        // proof serialization
        let mut serialized = Vec::new();
        proof.serialize_compressed(&mut serialized)
            .expect("Proof serialization failed.");
        let proof_size = serialized.len();

        // constructed vote with the completed proof
        Self {
            party: party.as_u64(),
            nullifier,
            proof,
            commitment: blinding,
            proof_size,
            gen_time,
            verify_time: 0,
        }
    }

    // function for proof verification
    pub fn verify(&mut self, vk: &VerifyingKey<Bls12_381>) -> bool {

        // verification time initialization
        let verify_start = Instant::now();
        
        // verify the proof
        let result = Groth16::<Bls12_381>::verify(
            vk,
            &[self.nullifier, self.commitment],  // Public inputs
            &self.proof
        ).unwrap_or(false);
        
        // measure verification time
        self.verify_time = verify_start.elapsed().as_nanos();
        result
    }
}