// import types from libaries and modules
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use sha3::{Sha3_256, Digest};

// voter structure
#[derive(Debug, Clone)]
pub struct Voter {
    pub id: String, // voter ID
    id_blinding: Scalar, // blinding factor
}

// voter implementation
impl Voter {
    
    // function to create a new voter
    pub fn new(id: String) -> Self {

        let mut rng = OsRng;

        // constructed voter
        Self {
            id,
            id_blinding: Scalar::random(&mut rng),
        }

    }

    // function to compute a voters unique identifier
    pub fn nullifier(&self) -> [u8; 32] {
        let mut hash = Sha3_256::new();
        hash.update(self.id.as_bytes());
        hash.update(self.id_blinding.as_bytes());
        hash.finalize().into()
    }

}

// party enum
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Party {
    Liberal = 0,
    Conservative = 1,
    NDP = 2,
    Green = 3,
}

// party implementation
impl Party {

    // function to return all the parties
    pub fn values() -> Vec<Self> {
        vec![Self::Liberal, Self::Conservative, Self::NDP, Self::Green]
    }

    // function to convert the enum party to u64
    pub fn as_u64(&self) -> u64 {
        *self as u64
    }

}