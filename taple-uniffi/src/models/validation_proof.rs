use taple_core::{ValidationProof as TapleValidationProof, Derivable};

pub struct ValidationProof {
    pub subject_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub name: String,
    pub subject_public_key: String,
    pub governance_id: String,
    pub genesis_governance_version: u64,
    pub sn: u64,
    pub prev_event_hash: String,
    pub event_hash: String,
    pub governance_version: u64,
}

impl From<TapleValidationProof> for ValidationProof {
    fn from(value: TapleValidationProof) -> Self {
        Self {
            subject_id: value.subject_id.to_str(),
            schema_id: value.schema_id,
            namespace: value.namespace,
            name: value.name,
            subject_public_key: value.subject_public_key.to_str(),
            governance_id: value.governance_id.to_str(),
            genesis_governance_version: value.genesis_governance_version,
            sn: value.sn,
            prev_event_hash: value.prev_event_hash.to_str(),
            event_hash: value.event_hash.to_str(),
            governance_version: value.governance_version,
        }
    }
}
