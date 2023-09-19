use std::str::FromStr;

use taple_core::{
    signature::Signed, ApprovalRequest, ApprovalResponse, Derivable, DigestIdentifier, ValueWrapper,
};

use crate::{TapleError, TapleSignature};

use super::request::TapleSignedEventRequest;

#[derive(Clone, Debug)]
pub struct TapleApprovalRequest {
    pub event_request: TapleSignedEventRequest,
    pub sn: u64,
    pub gov_version: u64,
    pub patch: String,
    pub state_hash: String,
    pub hash_prev_event: String,
    pub gov_id: String,
}

impl From<ApprovalRequest> for TapleApprovalRequest {
    fn from(value: ApprovalRequest) -> Self {
        Self {
            event_request: value.event_request.into(),
            sn: value.sn,
            gov_version: value.gov_version,
            patch: value.patch.0.to_string(),
            state_hash: value.state_hash.to_str(),
            hash_prev_event: value.hash_prev_event.to_str(),
            gov_id: value.gov_id.to_str(),
        }
    }
}

impl TryInto<ApprovalRequest> for TapleApprovalRequest {
    type Error = TapleError;
    fn try_into(self) -> Result<ApprovalRequest, Self::Error> {
        Ok(ApprovalRequest {
            event_request: self.event_request.try_into()?,
            sn: self.sn,
            gov_version: self.gov_version,
            gov_id: DigestIdentifier::from_str(&self.gov_id)
                .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
            patch: ValueWrapper(
                serde_json::from_str(&self.patch).map_err(|_| TapleError::NoJSONString)?,
            ),
            state_hash: DigestIdentifier::from_str(&self.state_hash)
                .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
            hash_prev_event: DigestIdentifier::from_str(&self.hash_prev_event)
                .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
        })
    }
}

pub struct TapleSignedApprovalRequest {
    pub content: TapleApprovalRequest,
    pub signature: TapleSignature,
}

impl From<Signed<ApprovalRequest>> for TapleSignedApprovalRequest {
    fn from(value: Signed<ApprovalRequest>) -> Self {
        Self {
            content: value.content.into(),
            signature: value.signature.into(),
        }
    }
}

impl TryInto<Signed<ApprovalRequest>> for TapleSignedApprovalRequest {
    type Error = TapleError;
    fn try_into(self) -> Result<Signed<ApprovalRequest>, Self::Error> {
        Ok(Signed {
            content: self.content.try_into()?,
            signature: self.signature.try_into()?,
        })
    }
}

pub struct TapleSignedApprovalResponse {
    pub content: TapleApprovalResponse,
    pub signature: TapleSignature,
}

impl From<Signed<ApprovalResponse>> for TapleSignedApprovalResponse {
    fn from(value: Signed<ApprovalResponse>) -> Self {
        Self {
            content: value.content.into(),
            signature: value.signature.into(),
        }
    }
}

impl TryInto<Signed<ApprovalResponse>> for TapleSignedApprovalResponse {
    type Error = TapleError;
    fn try_into(self) -> Result<Signed<ApprovalResponse>, Self::Error> {
        Ok(Signed {
            content: self.content.try_into()?,
            signature: self.signature.try_into()?,
        })
    }
}

//TODO: revisar porque el el TappleApprovalResponse me da error
#[derive(Clone, Debug)]
pub struct TapleApprovalResponse {
    pub appr_req_hash: String,
    pub approved: bool,
}

impl From<ApprovalResponse> for TapleApprovalResponse {
    fn from(value: ApprovalResponse) -> Self {
        Self {
            appr_req_hash: value.appr_req_hash.to_str(),
            approved: value.approved,
        }
    }
}

impl TryInto<ApprovalResponse> for TapleApprovalResponse {
    type Error = TapleError;
    fn try_into(self) -> Result<ApprovalResponse, Self::Error> {
        Ok(ApprovalResponse {
            appr_req_hash: DigestIdentifier::from_str(&self.appr_req_hash)
                .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
            approved: self.approved,
        })
    }
}
