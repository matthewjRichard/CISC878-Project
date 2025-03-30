// arkworks libraries
use ark_bls12_381::Fr as BlsFr;
use ark_relations::{lc, r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},};

// circuit definition
#[derive(Clone)]
pub struct VotingCircuit {
    pub vote: Option<BlsFr>, //witness
    pub nullifier_hash: Option<BlsFr>,
    pub blinding: Option<BlsFr>,
}

// create constraints for proof generation and verification
impl ConstraintSynthesizer<BlsFr> for VotingCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<BlsFr>) -> Result<(), SynthesisError> {

        // create variables for constraints
        let vote_var = cs.new_witness_variable(|| self.vote.ok_or(SynthesisError::AssignmentMissing))?;
        let _nullifier_var = cs.new_input_variable(|| self.nullifier_hash.ok_or(SynthesisError::AssignmentMissing))?;
        let _blinding_var = cs.new_input_variable(|| self.blinding.ok_or(SynthesisError::AssignmentMissing))?;
        let one = BlsFr::from(1u64);
        let two = BlsFr::from(2u64);
        let three = BlsFr::from(3u64);

        // constraint one
        let product1 = cs.new_witness_variable(|| Ok(self.vote.unwrap() * (self.vote.unwrap() - one)))?;
        cs.enforce_constraint(
            lc!() + vote_var,
            lc!() + vote_var - (one, Variable::One),
            lc!() + product1,
        )?;

        // constraint two
        let product2 = cs.new_witness_variable(|| Ok((self.vote.unwrap() - two) * (self.vote.unwrap() - three)))?;
        cs.enforce_constraint(
            lc!() + vote_var - (two, Variable::One),
            lc!() + vote_var - (three, Variable::One),
            lc!() + product2,
        )?;

        // constraint three; ensure votes are a valid option
        cs.enforce_constraint(
            lc!() + product1,
            lc!() + product2,
            lc!(),
        )?;

        Ok(())
    }
}