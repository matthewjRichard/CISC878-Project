// import types from libaries and modules
use std::collections::HashSet;
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_snark::CircuitSpecificSetupSNARK;
use ark_std::{rand::thread_rng, UniformRand};
use memory_stats::memory_stats;
use crate::{circuit::VotingCircuit, vote_proof::SnarkVote};

// vote submission structure
pub struct VoteSubmission {
    nullifiers: HashSet<BlsFr>, // voter identifier
    pub pk: ProvingKey<Bls12_381>, // proving key
    pub vk: VerifyingKey<Bls12_381>, // verifiying key
    votes: Vec<(u64, BlsFr)>, // vote
    total_gen_time: u128, // proof generation time
    total_verify_time: u128, // proof verification time
    total_proof_size: usize, // size of all proofs
    total_votes: usize, // number of votes
    total_memory_used: usize, // memory usage
}

// vote submission implementation
impl VoteSubmission {

    // function to create a new vote submission
    pub fn new() -> Self {

        let mut rng = thread_rng();
        
        // create a circuit for the trusted setup
        let setup_circuit = VotingCircuit {
            vote: Some(BlsFr::from(1)),
            nullifier_hash: Some(BlsFr::rand(&mut rng)),
            blinding: Some(BlsFr::rand(&mut rng)),
        };

        // generate proving and verification keys
        let (pk, vk) = Groth16::<Bls12_381>::setup(setup_circuit, &mut rng)
            .expect("Trusted setup failed.");

        // structure vote submission
        Self {
            nullifiers: HashSet::new(),
            pk,
            vk,
            votes: Vec::new(),
            total_gen_time: 0,
            total_verify_time: 0,
            total_proof_size: 0,
            total_votes: 0,
            total_memory_used: 0,
        }
    }

    // function to submit the vote
    pub fn submit_vote(&mut self, mut vote: SnarkVote) -> bool {

        // start measuring memory usage
        let mem_before = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

        // ensure voter is not a duplicate voter
        if self.nullifiers.contains(&vote.nullifier) {
            return false;
        }

        // verify the proof
        if !vote.verify(&self.vk) {
            return false;
        }

        // stop measuring memory
        let mem_after = memory_stats().map(|m| m.physical_mem).unwrap_or(0);
        self.total_memory_used += mem_after.saturating_sub(mem_before);

        // update the measured performance metrics
        self.total_votes += 1;
        self.total_gen_time += vote.gen_time;
        self.total_verify_time += vote.verify_time;
        self.total_proof_size += vote.proof_size;

        // record the vote
        self.nullifiers.insert(vote.nullifier);
        self.votes.push((vote.party, vote.commitment));
        true

    }

    // function to count the votes for each party
    pub fn tally(&self) -> Vec<(u64, usize)> {
        let mut counts = vec![0; 4];
        for (party, _) in &self.votes {
            counts[*party as usize] += 1;
        }
        counts.into_iter().enumerate().map(|(i, c)| (i as u64, c)).collect()
    }

    // function to display the performance metrics
    pub fn display_metrics(&self) {

        println!("\n=== Performance Metrics ===");
        println!("Number of voted processed: {}", self.total_votes);
        println!("Average proof generation time: {:.2} ms", 
            self.total_gen_time as f64 / self.total_votes as f64 / 1_000_000.0);
        println!("Average verification time: {:.2} ms", 
            self.total_verify_time as f64 / self.total_votes as f64 / 1_000_000.0);
        println!("Average proof size: {} bytes", 
            self.total_proof_size / self.total_votes);
        println!("Total memory usage: {} KB", 
            self.total_memory_used / 1024);

    }

}
