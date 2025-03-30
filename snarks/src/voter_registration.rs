// import types from libaries and modules
use ark_bls12_381::Fr as BlsFr;
use ark_ff::{BigInteger, PrimeField, UniformRand};
use rand::rngs::OsRng;
use sha3::{Sha3_256, Digest};

// voter structure
#[derive(Debug, Clone)]
pub struct Voter {
    pub id: String, // voter ID
    id_blinding: BlsFr, // blinding factor
}

// voter implementation
impl Voter {
    
    // function to create a new voter
    pub fn new(id: String) -> Self {

        let mut rng = OsRng;

        // constructed voter
        Self {
            id,
            id_blinding: BlsFr::rand(&mut rng),
        }

    }

    // function to compute a voters unique identifier
    pub fn nullifier(&self) -> BlsFr {
        let mut hash = Sha3_256::new();
        hash.update(self.id.as_bytes());
        hash.update(self.id_blinding.into_bigint().to_bytes_le());
        BlsFr::from_le_bytes_mod_order(&hash.finalize())
    }

}

// party enum
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Party {
    Liberal = 0,
    Conservative = 1,
    NDP = 2,
    Green = 3,
}

// party implementation
impl Party {
    
    // function to convert the enum party to u64
    pub fn as_u64(&self) -> u64 {
        *self as u64
    }
    
    // function to convert the u64 part to an enum
    pub fn from_u64(value: u64) -> Self {
        match value {
            0 => Party::Liberal,
            1 => Party::Conservative,
            2 => Party::NDP,
            3 => Party::Green,
            _ => panic!("Invalid party value"),
        }
    }
    
}