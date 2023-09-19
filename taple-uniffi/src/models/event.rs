use std::collections::HashMap;

use taple_core::{signature::Signed, Derivable, Event};

use crate::{TapleSignature, TapleSignedEventRequest};

#[derive(Clone, Debug)]
pub struct TapleEvent {
    pub subject_id: String,
    pub event_request: TapleSignedEventRequest,
    pub sn: u64,
    pub gov_version: u64,
    pub patch: String,
    pub state_hash: String,
    pub eval_success: bool,
    pub appr_required: bool,
    pub approved: bool,
    pub hash_prev_event: String,
    pub evaluators: HashMap<String, TapleSignature>,
    pub approvers: HashMap<String, TapleSignature>,
}

impl From<Event> for TapleEvent {
    fn from(value: Event) -> Self {
        let mut evaluators_aux: HashMap<String, TapleSignature> = HashMap::new();
        let mut approvers_aux: HashMap<String, TapleSignature> = HashMap::new();

        for x in value.evaluators {
            evaluators_aux.insert(x.value.to_str(), x.into());
        }

        for x in value.approvers {
            approvers_aux.insert(x.value.to_str(), x.into());
        }

        Self {
            subject_id: value.subject_id.to_str(),
            event_request: value.event_request.into(),
            sn: value.sn,
            gov_version: value.gov_version,
            patch: value.patch.0.to_string(),
            state_hash: value.state_hash.to_str(),
            eval_success: value.eval_success,
            appr_required: value.appr_required,
            approved: value.approved,
            hash_prev_event: value.hash_prev_event.to_str(),
            evaluators: evaluators_aux,
            approvers: approvers_aux,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TapleSignedEvent {
    pub content: TapleEvent,
    pub signature: TapleSignature,
}

impl From<Signed<Event>> for TapleSignedEvent {
    fn from(value: Signed<Event>) -> Self {
        Self {
            content: value.content.into(),
            signature: value.signature.into(),
        }
    }
}
