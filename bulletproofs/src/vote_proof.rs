// import types from libaries and modules
use std::time::Instant;
use bincode;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use rand::rngs::OsRng;
use crate::voter_registration::Party;

// vote structure
pub struct Vote {
    pub party: Party, // voted party
    pub nullifier: [u8; 32], // identifier
    pub proof: RangeProof, // range proof
    pub commitment: CompressedRistretto, // blinding factor commitment
    pub proof_size: usize, // proof size
    pub gen_time: u128, // proof generation time
    pub verify_time: u128, // proof verification time
}

// implement vote
impl Vote {
    
    // function to create a new vote
    pub fn new(
        party: Party,
        nullifier: [u8; 32],
        bp_gens: &BulletproofGens,
        pc_gens: &PedersenGens,
    ) -> Option<Self> {
        let mut rng = OsRng;
        
        // only allow the four valid parties
        if party.as_u64() > 3 {
            return None;
        }

        // variable initialization
        let vote_value = party.as_u64();
        let blinding = Scalar::random(&mut rng);
        let mut transcript = Transcript::new(b"VoteProof");

        // proof generation
        let start_time = Instant::now();
        let (proof, commitment) = RangeProof::prove_single(
            bp_gens,
            pc_gens,
            &mut transcript,
            vote_value,
            &blinding,
            8,
        ).ok()?;
        let gen_time = start_time.elapsed().as_nanos();

        // proof serialization
        let proof_size = bincode::serialized_size(&proof).ok()? as usize;

        // constructed vote with the completed proof
        Some(Self {
            party,
            nullifier,
            proof,
            commitment,
            proof_size,
            gen_time,
            verify_time: 0,
        })

    }

    // function for proof verification
    pub fn verify(&mut self, bp_gens: &BulletproofGens, pc_gens: &PedersenGens) -> bool {

        let mut transcript = Transcript::new(b"VoteProof");

        // verification time initialization
        let start_time = Instant::now();

        // verify the proof
        let result = self.proof.verify_single(
            bp_gens, 
            pc_gens, 
            &mut transcript, 
            &self.commitment, 
            8
        ).is_ok();

        // measure verification time
        self.verify_time = start_time.elapsed().as_nanos();
        result

    }

}