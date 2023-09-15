use std::str::FromStr;

use taple_core::request::{RequestState, TapleRequest as CoreTapleRequest};
use taple_core::signature::Signed;
use taple_core::Derivable;

use taple_core::{
    request::{
        EOLRequest as TapleEOLRequest, EventRequest as TapleEventRequestType,
        FactRequest as TapleFactRequest, StartRequest as TapleCreateRequest,
        TransferRequest as TapleTransferRequest,
    },
    DigestIdentifier, KeyIdentifier, ValueWrapper,
};

use crate::{TapleError, TapleSignature};

#[derive(Clone, Debug)]
pub enum EventRequestType {
    Create {
        governance_id: String,
        schema_id: String,
        namespace: String,
        name: String,
        public_key: String,
    },
    Fact {
        subject_id: String,
        payload: String,
    },
    Transfer {
        subject_id: String,
        public_key: String,
    },
    EOL {
        subject_id: String,
    },
}

#[derive(Clone, Debug)]
pub struct TransferRequest {
    pub subject_id: String,
    pub public_key: String,
}

#[derive(Clone, Debug)]
pub struct EOLRequest {
    pub subject_id: String,
}

impl From<TapleEventRequestType> for EventRequestType {
    fn from(value: TapleEventRequestType) -> Self {
        match value {
            TapleEventRequestType::Create(value) => EventRequestType::Create {
                governance_id: value.governance_id.to_str(),
                schema_id: value.schema_id,
                namespace: value.namespace,
                name: value.name,
                public_key: value.public_key.to_str(),
            },
            TapleEventRequestType::Fact(value) => EventRequestType::Fact {
                subject_id: value.subject_id.to_str(),
                payload: value.payload.0.to_string(),
            },
            TapleEventRequestType::Transfer(value) => EventRequestType::Transfer {
                subject_id: value.subject_id.to_str(),
                public_key: value.public_key.to_str(),
            },
            TapleEventRequestType::EOL(value) => EventRequestType::EOL {
                subject_id: value.subject_id.to_str(),
            },
        }
    }
}

impl TryInto<TapleEventRequestType> for EventRequestType {
    type Error = TapleError;
    fn try_into(self) -> Result<TapleEventRequestType, Self::Error> {
        Ok(match self {
            Self::Create {
                governance_id,
                schema_id,
                namespace,
                name,
                public_key,
            } => TapleEventRequestType::Create(TapleCreateRequest {
                governance_id: DigestIdentifier::from_str(&governance_id)
                    .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                schema_id: schema_id,
                namespace: namespace,
                name: name,
                public_key: KeyIdentifier::from_str(&public_key)
                    .map_err(|_| TapleError::KeyIdentifierGenerationFailed)?,
            }),
            Self::Fact {
                subject_id,
                payload,
            } => TapleEventRequestType::Fact(TapleFactRequest {
                subject_id: DigestIdentifier::from_str(&subject_id)
                    .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                payload: ValueWrapper(
                    serde_json::from_str(&payload).map_err(|_| TapleError::NoJSONString)?,
                ),
            }),
            Self::Transfer {
                subject_id,
                public_key,
            } => TapleEventRequestType::Transfer(TapleTransferRequest {
                subject_id: DigestIdentifier::from_str(&subject_id)
                    .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                public_key: KeyIdentifier::from_str(&public_key)
                    .map_err(|_| TapleError::KeyIdentifierGenerationFailed)?,
            }),
            Self::EOL { subject_id } => TapleEventRequestType::EOL(TapleEOLRequest {
                subject_id: DigestIdentifier::from_str(&subject_id)
                    .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
            }),
        })
    }
}

#[derive(Clone, Debug)]
pub struct TapleSignedEventRequest {
    pub content: EventRequestType,
    pub signature: TapleSignature,
}

impl From<Signed<TapleEventRequestType>> for TapleSignedEventRequest {
    fn from(value: Signed<TapleEventRequestType>) -> Self {
        Self {
            content: EventRequestType::from(value.content),
            signature: TapleSignature::from(value.signature),
        }
    }
}

impl TryInto<Signed<TapleEventRequestType>> for TapleSignedEventRequest {
    type Error = TapleError;
    fn try_into(self) -> Result<Signed<TapleEventRequestType>, Self::Error> {
        Ok(Signed {
            content: self.content.try_into()?,
            signature: self.signature.try_into()?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum TapleRequestState {
    Finished,
    Error,
    Processing,
}

impl From<RequestState> for TapleRequestState {
    fn from(value: RequestState) -> Self {
        match value {
            RequestState::Finished => TapleRequestState::Finished,
            RequestState::Error => TapleRequestState::Error,
            RequestState::Processing => TapleRequestState::Processing,
        }
    }
}

impl Into<RequestState> for TapleRequestState {
    fn into(self) -> RequestState {
        match self {
            TapleRequestState::Finished => RequestState::Finished,
            TapleRequestState::Error => RequestState::Error,
            TapleRequestState::Processing => RequestState::Processing,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TapleRequest {
    pub id: String,
    pub subject_id: Option<String>,
    pub sn: u64,
    pub event_request: TapleSignedEventRequest,
    pub state: TapleRequestState,
    pub success: bool,
}

impl From<CoreTapleRequest> for TapleRequest {
    fn from(value: CoreTapleRequest) -> Self {
        log::debug!("{:?}", value);
        Self {
            id: value.id.to_str(),
            subject_id: if let Some(id) = value.subject_id {
                Some(id.to_str())
            } else {
                None
            },
            sn: value.sn.unwrap(),
            event_request: TapleSignedEventRequest::from(value.event_request),
            state: TapleRequestState::from(value.state),
            success: value.success.unwrap(),
        }
    }
}

impl TryInto<CoreTapleRequest> for TapleRequest {
    type Error = TapleError;
    fn try_into(self) -> Result<CoreTapleRequest, Self::Error> {
        Ok(CoreTapleRequest {
            id: DigestIdentifier::from_str(&self.id)
                .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
            subject_id: if let Some(id) = self.subject_id {
                Some(
                    DigestIdentifier::from_str(&id)
                        .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                )
            } else {
                None
            },
            sn: Some(self.sn),
            event_request: self.event_request.try_into()?,
            state: self.state.into(),
            success: Some(self.success),
        })
    }
}
