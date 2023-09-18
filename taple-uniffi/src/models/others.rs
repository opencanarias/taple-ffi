use taple_core::KeyDerivator;

use super::signature::TapleSignature;
use super::validation_proof::ValidationProof;

pub struct SubjectAndProviders {
  pub subject_id: String,
  pub providers: Vec<String>
}

pub struct ValidationProofAndSignatures {
  pub validation_proof: ValidationProof,
  pub signatures: Vec<TapleSignature>
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