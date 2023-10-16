use taple_core::{DigestDerivator, KeyDerivator};

use super::signature::TapleSignature;
use super::validation_proof::ValidationProof;

pub struct SubjectAndProviders {
    pub subject_id: String,
    pub providers: Vec<String>,
}

pub struct ValidationProofAndSignatures {
    pub validation_proof: ValidationProof,
    pub signatures: Vec<TapleSignature>,
}

pub enum TapleKeyDerivator {
    Ed25519,
    Secp256k1,
}

impl From<KeyDerivator> for TapleKeyDerivator {
    fn from(value: KeyDerivator) -> Self {
        match value {
            KeyDerivator::Ed25519 => Self::Ed25519,
            KeyDerivator::Secp256k1 => Self::Secp256k1,
        }
    }
}

impl Into<KeyDerivator> for TapleKeyDerivator {
    fn into(self) -> KeyDerivator {
        match self {
            TapleKeyDerivator::Ed25519 => KeyDerivator::Ed25519,
            TapleKeyDerivator::Secp256k1 => KeyDerivator::Secp256k1,
        }
    }
}
#[derive(Clone)]
pub enum TapleDigestDerivator {
    Blake3_256,
    Blake3_512,
    SHA2_256,
    SHA2_512,
    SHA3_256,
    SHA3_512,
}

impl From<DigestDerivator> for TapleDigestDerivator {
    fn from(value: DigestDerivator) -> Self {
        match value {
            DigestDerivator::Blake3_256 => TapleDigestDerivator::Blake3_256,
            DigestDerivator::Blake3_512 => TapleDigestDerivator::Blake3_512,
            DigestDerivator::SHA2_256 => TapleDigestDerivator::SHA2_256,
            DigestDerivator::SHA2_512 => TapleDigestDerivator::SHA2_512,
            DigestDerivator::SHA3_256 => TapleDigestDerivator::SHA3_256,
            DigestDerivator::SHA3_512 => TapleDigestDerivator::SHA3_512,
        }
    }
}

impl Into<DigestDerivator> for TapleDigestDerivator {
    fn into(self) -> DigestDerivator {
        match self {
            TapleDigestDerivator::Blake3_256 => DigestDerivator::Blake3_256,
            TapleDigestDerivator::Blake3_512 => DigestDerivator::Blake3_256,
            TapleDigestDerivator::SHA2_256 => DigestDerivator::SHA2_256,
            TapleDigestDerivator::SHA2_512 => DigestDerivator::SHA2_512,
            TapleDigestDerivator::SHA3_256 => DigestDerivator::SHA3_256,
            TapleDigestDerivator::SHA3_512 => DigestDerivator::SHA3_512,
        }
    }
}
