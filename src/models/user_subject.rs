use std::sync::{Arc, RwLock};

//Abstraccion del Sujeto de Taple para facilitar su uso por terceros
use crate::{TapleError, TapleSignedEventRequest};
use taple_core::{
    crypto::KeyPair,
    request::{EOLRequest, FactRequest, RequestState},
    signature::{Signature, Signed},
    Api, Derivable, DigestDerivator, DigestIdentifier, EventRequest, SubjectData, ValueWrapper,
};
use tokio::runtime::Runtime;

use super::user_governance::UserGovernance;

pub struct UserSubject {
    pub api: Api,
    keys: KeyPair,
    pub runtime: Arc<Runtime>,
    pub subject_data: RwLock<Option<SubjectData>>,
    subject_request: Option<DigestIdentifier>,
    derivator: DigestDerivator,
}

pub fn create_subject(
    api: Api,
    keys: KeyPair,
    runtime: Arc<Runtime>,
    subject_data: RwLock<Option<SubjectData>>,
    subject_request: Option<DigestIdentifier>,
    derivator: DigestDerivator,
) -> UserSubject {
    UserSubject {
        api,
        keys,
        runtime,
        subject_data,
        subject_request: subject_request,
        derivator: derivator,
    }
}

impl UserSubject {
    fn _get_subject_id(&self) -> Result<DigestIdentifier, TapleError> {
        let Ok(lock) = self.subject_data.read() else {
            return Err(TapleError::LockIsPoisoned);
        };
        match &*lock {
            Some(di) => Ok(di.subject_id.clone()),
            None => Err(TapleError::InternalError),
        }
    }

    pub fn end_life_cicle(&self) -> Result<String, TapleError> {
        let subject_id = self._get_subject_id()?;

        let eol_event = EventRequest::EOL(EOLRequest {
            subject_id: subject_id,
        });

        let signed_event = self.event_signing(eol_event);

        match signed_event {
            Ok(se) => self.runtime.block_on(async {
                let req_id = self
                    .api
                    .external_request(se)
                    .await
                    .map_err(|e| TapleError::ExecutionError(e.to_string()))?;

                let id = self
                    .api
                    .get_request(req_id)
                    .await
                    .map_err(|e| TapleError::ExecutionError(e.to_string()))?;
                Ok(id.id.to_str())
            }),
            Err(e) => Err(e),
        }
    }

    pub fn new_fact_event(&self, payload: String) -> Result<String, TapleError> {
        let subject_id = self._get_subject_id()?;

        let fact_event = EventRequest::Fact(FactRequest {
            subject_id: subject_id,
            payload: ValueWrapper(serde_json::from_str(&payload).unwrap()),
        });

        let signed_event = self.event_signing(fact_event);

        match signed_event {
            Ok(se) => self.runtime.block_on(async {
                let req = self
                    .api
                    .external_request(se)
                    .await
                    .map_err(|e| TapleError::ExecutionError(e.to_string()))?;
                Ok(req.to_str())
            }),
            Err(e) => Err(e),
        }
    }

    pub fn refresh(&self) -> Result<(), TapleError> {
        //Checks local SN and compares it to Taples version. If is underversioned it updates itself, if not it does nothing.
        //Comprovar event request y si esta completada recuperar sujeto
        let subject_d = {
            match self.subject_data.read() {
                Ok(sd_pointer) => sd_pointer.clone(),
                Err(_) => return Err(TapleError::LockIsPoisoned),
            }
        };
        match subject_d {
            Some(subject_data) => {
                self.runtime.block_on(async {
                    let sn = subject_data.sn;
                    match self.api.get_subject(subject_data.subject_id.clone()).await {
                        Ok(api_sid) => {
                            if api_sid.sn == sn {
                                //Estado actualizado
                                Ok(())
                            } else {
                                //Actualizar estado
                                if sn < api_sid.sn {
                                    let Ok(mut lock) = self.subject_data.write() else {
                                                return Err(TapleError::LockIsPoisoned);
                                            };
                                    *lock = Some(api_sid);
                                }
                                Ok(())
                            }
                        }
                        Err(error) => Err(TapleError::ExecutionError(error.to_string())),
                    }
                })
            }
            None => match &self.subject_request {
                Some(request) => self.runtime.block_on(async {
                    match self.api.get_request(request.clone()).await {
                        Ok(event) => {
                            if event.state != RequestState::Finished {
                                return Ok(());
                            }
                            match event.subject_id {
                                Some(sid) => {
                                    let subject =
                                        self.api.get_subject(sid).await.map_err(|e| {
                                            TapleError::ExecutionError(e.to_string())
                                        })?;

                                    match self.subject_data.write() {
                                        Ok(mut lock) => {
                                            *lock = Some(subject);
                                            return Ok(());
                                        }
                                        Err(_) => return Err(TapleError::LockIsPoisoned),
                                    }
                                }
                                None => {
                                    return Err(TapleError::NotFound(
                                        "Subject ID not found".to_owned(),
                                    ))
                                }
                            }
                        }
                        Err(e) => return Err(TapleError::ExecutionError(e.to_string())),
                    }
                }),
                None => return Err(TapleError::NotFound("Event request not found".to_owned())),
            },
        }
    }

    pub fn external_invokation(
        &self,
        event: TapleSignedEventRequest,
    ) -> Result<String, TapleError> {
        let data: Signed<EventRequest> = event
            .try_into()
            .map_err(|e: TapleError| TapleError::ExecutionError(e.to_string()))?;

        self.runtime.block_on(async {
            let res = self
                .api
                .external_request(data)
                .await
                .map_err(|e| TapleError::ExecutionError(e.to_string()))?;
            Ok(res.to_str())
        })
    }

    pub fn to_governance(&self) -> Option<Arc<UserGovernance>> {
        match self.subject_data.read().unwrap().clone() {
            Some(subject_data) => Some(Arc::new(UserGovernance {
                api: self.api.clone(),
                governance_data: RwLock::new(subject_data),
                runtime: self.runtime.clone(),
            })),
            None => None,
        }
    }

    //Getter of subjectData fields
    pub fn get_subject_id(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.subject_id.to_str())
    }

    pub fn get_governance_id(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.governance_id.to_str())
    }

    pub fn get_sn(&self) -> Option<u64> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.sn)
    }

    pub fn get_public_key(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.public_key.to_str())
    }

    pub fn get_namespace(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.namespace.clone())
    }

    pub fn get_schema_id(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.schema_id.clone())
    }

    pub fn get_owner(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.owner.to_str())
    }

    pub fn get_creator(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.creator.to_str())
    }

    pub fn get_properties(&self) -> Option<String> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.properties.0.to_string())
    }

    pub fn get_subject_request(&self) -> Option<String> {
        match &self.subject_request {
            Some(sr) => Some(sr.to_str()),
            None => None,
        }
    }

    pub fn get_is_active(&self) -> Option<bool> {
        let Some(lock) = &*self.subject_data.read().unwrap() else { return None };
        Some(lock.active)
    }

    fn event_signing(&self, event: EventRequest) -> Result<Signed<EventRequest>, TapleError> {
        let event_signature = Signature::new::<EventRequest>(&event, &self.keys, self.derivator);

        match event_signature {
            Ok(signature) => Ok(Signed {
                content: event,
                signature,
            }),
            Err(e) => Err(TapleError::SignatureGenerationFailed(e.to_string())),
        }
    }
}
