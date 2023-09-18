use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

use taple_core::{
    crypto::KeyPair,
    request::StartRequest,
    signature::{Signature, Signed},
    Api, DigestIdentifier, EventRequest, KeyDerivator,
};
use tokio::runtime::Runtime;

use crate::{
    models::user_subject::{create_subject, UserSubject},
    TapleError,
};

pub struct SubjectBuilder {
    pub api: Api,
    pub runtime: Arc<Runtime>,
    pub keys: KeyPair,
    pub name: RwLock<Option<String>>,
    pub namespace: RwLock<Option<String>>,
}

impl SubjectBuilder {
    fn get_name(&self) -> Result<String, TapleError> {
        match self.name.read() {
            Ok(s) => match &*s {
                Some(name) => Ok(name.to_string()),
                None => Ok("".to_owned()),
            },
            Err(_) => Err(TapleError::LockIsPoisoned),
        }
    }

    pub fn with_name(&self, name: String) -> Result<(), TapleError> {
        let name_lock = self.name.write();
        match name_lock {
            Ok(mut nl) => {
                *nl = Some(name);
                Ok(())
            }
            Err(_) => Err(TapleError::LockIsPoisoned),
        }
    }

    fn get_namespace(&self) -> Result<String, TapleError> {
        match self.namespace.read() {
            Ok(s) => match &*s {
                Some(namespace) => Ok(namespace.to_string()),
                None => Ok("".to_owned()),
            },
            Err(_) => Err(TapleError::LockIsPoisoned),
        }
    }

    pub fn with_namespace(&self, namespace: String) -> Result<(), TapleError> {
        let namespace_lock = self.namespace.write();
        match namespace_lock {
            Ok(mut nsl) => {
                *nsl = Some(namespace);
                Ok(())
            }
            Err(_) => Err(TapleError::LockIsPoisoned),
        }
    }

    pub fn build(
        &self,
        governance_id: String,
        schema_id: String,
    ) -> Result<Arc<UserSubject>, TapleError> {
        let subject_namespace: String = match self.get_namespace() {
            Ok(e) => e,
            Err(e) => return Err(e),
        };
        let subject_name: String = match self.get_name() {
            Ok(e) => e,
            Err(e) => return Err(e),
        };
        let derivator = KeyDerivator::Ed25519;

        self.runtime.block_on(async {
            match self.api.add_keys(derivator).await {
                Ok(subject_key_identifier) => {
                    let create_event = EventRequest::Create(StartRequest {
                        governance_id: match DigestIdentifier::from_str(&governance_id) {
                            Ok(gid) => gid,
                            Err(e) => return Err(TapleError::ExecutionError(e.to_string())),
                        },
                        schema_id: schema_id,
                        namespace: subject_namespace,
                        name: subject_name,
                        public_key: subject_key_identifier.clone(),
                    });

                    let event_signed = match Signature::new(&create_event, &self.keys) {
                        Ok(signature) => Signed {
                            content: create_event,
                            signature,
                        },
                        Err(e) => return Err(TapleError::SignatureGenerationFailed(e.to_string())),
                    };

                    let request_id = self
                        .api
                        .external_request(event_signed)
                        .await
                        .map_err(|e| TapleError::ExecutionError(e.to_string()))?;

                    //Once the subject is created is where it gets complicated:
                    // 1. There is no direct way to know what has happened to the request.
                    // 2. We need to wait for the subject's approval.
                    // 3. We can't leave users hanging around waiting.

                    //Solution:
                    // 1. Return 2-upla with RequestID and SubjectBuilder.
                    // The SubjecBuilder will check the status of the subject.

                    let subject_api: Api = self.api.clone();
                    let subject_keys: KeyPair = self.keys.clone();
                    let subject_runtime: Arc<Runtime> = self.runtime.clone();
                    Ok(Arc::new(create_subject(
                        subject_api,
                        subject_keys,
                        subject_runtime,
                        RwLock::new(None),
                        Some(request_id),
                    )))
                }
                Err(e) => Err(TapleError::ExecutionError(e.to_string())),
            }
        })
    }
}
