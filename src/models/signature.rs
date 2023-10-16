use std::str::FromStr;

use taple_core::{
    signature::Signature, Derivable, DigestDerivator, DigestIdentifier, KeyIdentifier,
    SignatureIdentifier, TimeStamp,
};

use crate::error::TapleError;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TapleSignature {
    pub signer: String,
    pub timestamp: u64,
    pub value: String,
    pub content_hash: String,
}

impl From<Signature> for TapleSignature {
    fn from(value: Signature) -> Self {
        Self {
            signer: value.signer.to_str(),
            timestamp: value.timestamp.0,
            value: value.value.to_str(),
            content_hash: value.content_hash.to_str(),
        }
    }
}

impl TryInto<Signature> for TapleSignature {
    type Error = TapleError;

    fn try_into(self) -> Result<Signature, Self::Error> {
        Ok(Signature {
            signer: KeyIdentifier::from_str(&self.signer)
                .map_err(|_| TapleError::KeyIdentifierGenerationFailed)?,
            timestamp: TimeStamp(self.timestamp),
            value: SignatureIdentifier::from_str(&self.value)
                .map_err(|_| TapleError::SignatureIdentifierGenerationFailed)?,
            content_hash: DigestIdentifier::from_str(&self.content_hash)
                .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
        })
    }
}
