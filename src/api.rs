use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, RwLock},
};

use taple_core::{
    crypto::KeyPair, request::EventRequest as TapleEventRequestType, signature::Signature, Api,
    ApiError, Derivable, DigestDerivator, DigestIdentifier, KeyDerivator, KeyIdentifier,
};
use tokio::runtime::Runtime;

use crate::{
    error::TapleError,
    models::{event::TapleSignedEvent, user_subject::UserSubject},
    models::{
        others::{SubjectAndProviders, ValidationProofAndSignatures},
        user_subject::create_subject,
    },
    EventRequestType, TapleRequest, TapleSignature, TapleSignedEventRequest,
};

#[derive(Clone)]
pub struct TapleAPI {
    pub api: Api,
    pub runtime: Arc<Runtime>,
    keys: KeyPair,
    derivator: DigestDerivator,
}

pub fn create_taple_api(
    api: Api,
    runtime: Arc<Runtime>,
    keys: KeyPair,
    derivator: DigestDerivator,
) -> TapleAPI {
    TapleAPI {
        api,
        runtime,
        keys,
        derivator,
    }
}

fn error_conversion(error: ApiError) -> TapleError {
    match error {
        ApiError::NotFound(msg) => TapleError::NotFound(msg),
        _ => TapleError::ExecutionError(error.to_string()),
    }
}

impl TapleAPI {
    pub fn get_request(&self, request_id: String) -> Result<TapleRequest, TapleError> {
        self.runtime.block_on(async {
            let aux = self
                .api
                .get_request(
                    DigestIdentifier::from_str(&request_id)
                        .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                )
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(TapleRequest::from(aux))
        })
    }

    pub fn external_request(
        &self,
        event_request: TapleSignedEventRequest,
    ) -> Result<String, TapleError> {
        self.runtime.block_on(async {
            let request_id = self
                .api
                .external_request(event_request.try_into()?)
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(request_id.to_str())
        })
    }

    pub fn get_subjects(
        &self,
        namespace: String,
        from: Option<String>,
        quantity: Option<i64>,
    ) -> Result<Vec<Arc<UserSubject>>, TapleError> {
        self.runtime.block_on(async {
            let subjects = self
                .api
                .get_subjects(namespace, from, quantity)
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(subjects
                .into_iter()
                .map(|s| {
                    Arc::new(create_subject(
                        self.api.clone(),
                        self.keys.clone(),
                        self.runtime.clone(),
                        RwLock::new(Some(s)),
                        None,
                        self.derivator,
                    ))
                })
                .collect())
        })
    }

    pub fn get_governances(
        &self,
        namespace: String,
        from: Option<String>,
        quantity: Option<i64>,
    ) -> Result<Vec<Arc<UserSubject>>, TapleError> {
        self.runtime.block_on(async {
            // let quantity = quantity.map(|v| v as usize);
            let subjects = self
                .api
                .get_governances(namespace, from, quantity)
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(subjects
                .into_iter()
                .map(|s| {
                    Arc::new(create_subject(
                        self.api.clone(),
                        self.keys.clone(),
                        self.runtime.clone(),
                        RwLock::new(Some(s)),
                        None,
                        self.derivator,
                    ))
                })
                .collect())
        })
    }

    pub fn get_subjects_by_governance(
        &self,
        governance_id: String,
        from: Option<String>,
        quantity: Option<i64>,
    ) -> Result<Vec<Arc<UserSubject>>, TapleError> {
        self.runtime.block_on(async {
            let subjects = self
                .api
                .get_subjects_by_governance(
                    DigestIdentifier::from_str(&governance_id)
                        .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                    from,
                    quantity,
                )
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(subjects
                .into_iter()
                .map(|s| {
                    Arc::new(create_subject(
                        self.api.clone(),
                        self.keys.clone(),
                        self.runtime.clone(),
                        RwLock::new(Some(s)),
                        None,
                        self.derivator,
                    ))
                })
                .collect())
        })
    }

    pub fn get_events(
        &self,
        subject_id: String,
        from: Option<i64>,
        quantity: Option<i64>,
    ) -> Result<Vec<TapleSignedEvent>, TapleError> {
        self.runtime.block_on(async {
            let events = self
                .api
                .get_events(
                    DigestIdentifier::from_str(&subject_id)
                        .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                    from,
                    quantity,
                )
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(events.into_iter().map(|e| e.into()).collect())
        })
    }

    pub fn get_event(&self, subject_id: String, sn: u64) -> Result<TapleSignedEvent, TapleError> {
        self.runtime.block_on(async {
            let event = self
                .api
                .get_event(
                    DigestIdentifier::from_str(&subject_id)
                        .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                    sn,
                )
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(event.into())
        })
    }

    pub fn get_subject(&self, subject_id: String) -> Result<Arc<UserSubject>, TapleError> {
        self.runtime.block_on(async {
            let subject = self
                .api
                .get_subject(
                    DigestIdentifier::from_str(&subject_id)
                        .map_err(|_e| TapleError::DigestIdentifierGenerationFailed)?,
                )
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(Arc::new(create_subject(
                self.api.clone(),
                self.keys.clone(),
                self.runtime.clone(),
                RwLock::new(Some(subject)),
                None,
                self.derivator,
            )))
        })
    }

    pub fn add_preauthorize_subject(
        &self,
        subject_id: String,
        providers: Vec<String>,
    ) -> Result<(), TapleError> {
        self.runtime.block_on(async {
            let mut converted_providers = HashSet::new();
            for provider in providers {
                converted_providers.insert(
                    KeyIdentifier::from_str(&provider)
                        .map_err(|_| TapleError::KeyIdentifierGenerationFailed)?,
                );
            }
            let subject_id = DigestIdentifier::from_str(&subject_id)
                .map_err(|_e| TapleError::DigestIdentifierGenerationFailed)?;
            self.api
                .add_preauthorize_subject(&subject_id, &converted_providers)
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(())
        })
    }

    pub fn get_all_allowed_subjects_and_providers(
        &self,
        from: Option<String>,
        quantity: Option<i64>,
    ) -> Result<Vec<SubjectAndProviders>, TapleError> {
        self.runtime.block_on(async {
            let result = self
                .api
                .get_all_allowed_subjects_and_providers(from, quantity)
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(result
                .into_iter()
                .map(|(subject_id, providers)| SubjectAndProviders {
                    subject_id: subject_id.to_str(),
                    providers: providers.into_iter().map(|i| i.to_str()).collect(),
                })
                .collect())
        })
    }

    pub fn add_keys(&self, derivator: String) -> Result<String, TapleError> {
        let kd = KeyDerivator::from_str(&derivator).map_err(|_| TapleError::InvalidKeyDerivator)?;

        self.runtime.block_on(async {
            let ki = self
                .api
                .add_keys(kd)
                .await
                .map_err(|e| error_conversion(e))?;
            Ok(ki.to_str())
        })
    }

    pub fn get_validation_proof(
        &self,
        subject_id: String,
    ) -> Result<ValidationProofAndSignatures, TapleError> {
        self.runtime.block_on(async {
            let (signatures, proof) = self
                .api
                .get_validation_proof(
                    DigestIdentifier::from_str(&subject_id)
                        .map_err(|_| TapleError::DigestIdentifierGenerationFailed)?,
                )
                .await
                .map_err(|e| error_conversion(e))?;
            let signatures = signatures.into_iter().map(|s| s.into()).collect();
            Ok(ValidationProofAndSignatures {
                signatures,
                validation_proof: proof.into(),
            })
        })
    }

    pub fn sign_event_request(
        &self,
        event_request: EventRequestType,
    ) -> Result<TapleSignature, TapleError> {
        let event_signature = Signature::new::<TapleEventRequestType>(
            &event_request.try_into()?,
            &self.keys,
            self.derivator,
        )
        .map_err(|e| TapleError::SignatureGenerationFailed(e.to_string()))?;
        Ok(event_signature.into())
    }
}
