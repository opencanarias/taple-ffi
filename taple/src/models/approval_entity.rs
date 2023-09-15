use taple_core::{ApprovalEntity, ApprovalState, Derivable};

use super::approval::{TapleSignedApprovalRequest, TapleSignedApprovalResponse};

pub enum TapleApprovalState {
    Pending,
    RespondedAccepted,
    RespondedRejected,
    Obsolete,
}

impl From<ApprovalState> for TapleApprovalState {
    fn from(value: ApprovalState) -> Self {
        match value {
            ApprovalState::Pending => TapleApprovalState::Pending,
            ApprovalState::RespondedAccepted => TapleApprovalState::RespondedAccepted,
            ApprovalState::RespondedRejected => TapleApprovalState::RespondedRejected,
            ApprovalState::Obsolete => TapleApprovalState::Obsolete,
        }
    }
}

pub struct TapleApprovalEntity {
    pub id: String,
    pub request: TapleSignedApprovalRequest,
    pub response: Option<TapleSignedApprovalResponse>,
    pub state: TapleApprovalState,
}

impl From<ApprovalEntity> for TapleApprovalEntity {
    fn from(value: ApprovalEntity) -> Self {
        Self {
            id: value.id.to_str(),
            request: value.request.into(),
            response: if let Some(response) = value.response {
                Some(response.into())
            } else {
                None
            },
            state: value.state.into(),
        }
    }
}
