// import types from libaries and modules
use std::collections::HashSet;
use bulletproofs::{BulletproofGens, PedersenGens};
use crate::vote_proof::Vote;
use crate::voter_registration::Party;

// vote submission structure
pub struct VoteSubmission {
    nullifiers: HashSet<[u8; 32]>, // voter identifier
    pub bp_gens: BulletproofGens, // proof generator
    pub pc_gens: PedersenGens, // commitment generator
    votes: Vec<Party>, // votes
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

        Self {
            nullifiers: HashSet::new(),
            bp_gens: BulletproofGens::new(8, 1),
            pc_gens: PedersenGens::default(),
            votes: Vec::new(),
            total_gen_time: 0,
            total_verify_time: 0,
            total_proof_size: 0,
            total_votes: 0,
            total_memory_used: 0,
        }

    }

    // function to submit the vote
    pub fn submit_vote(&mut self, mut vote: Vote) -> bool {

        // start measuring memory usage
        let before = memory_stats::memory_stats()
            .map(|m| m.physical_mem)
            .unwrap_or(0);

        // ensure voter is not a duplicate voter
        if self.nullifiers.contains(&vote.nullifier) {
            return false;
        }

        // verify the proof
        if !vote.verify(&self.bp_gens, &self.pc_gens) {
            return false;
        }

        self.nullifiers.insert(vote.nullifier);
        self.votes.push(vote.party);

        // stop measuring memory
        let after = memory_stats::memory_stats().map(|m| m.physical_mem).unwrap_or(0);
        self.total_memory_used += after.saturating_sub(before);

        // update the measured performance metrics
        self.total_votes += 1;
        self.total_gen_time += vote.gen_time;
        self.total_verify_time += vote.verify_time;
        self.total_proof_size += vote.proof_size;

        true

    }

    // function to count the votes for each party
    pub fn tally(&self) -> Vec<(Party, usize)> {
        let mut counts = vec![0; 4];
        for party in &self.votes {
            counts[*party as usize] += 1;
        }
        Party::values().iter().map(|&p| (p, counts[p as usize])).collect()
    }

    // function to display the performance metrics
    pub fn display_metrics(&self) {

        println!("\n\n--- Performance Metrics ---");

        // zero vote error guard
        if self.total_votes == 0 {
            println!("No valid votes were submitted.");
            return;
        }

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