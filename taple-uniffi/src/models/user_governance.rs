use std::sync::{Arc, RwLock};

use serde::Deserialize;
use taple_core::{Derivable, Api, SubjectData};
use tokio::runtime::Runtime;

use crate::TapleError;

use super::{policy::Policy, role::Role, schema::Schema, user_subject::UserSubject};

//Governance abstration to simplify usage for third parties
pub struct UserGovernance {
    pub api: Api,
    pub governance_data: RwLock<SubjectData>,
    pub runtime: Arc<Runtime>,
}

//Implementar un constructor que verifique el schema ID para saber si puede ser governanza
impl UserGovernance {
    pub fn new(subject: Arc<UserSubject>) -> Result<Self, TapleError> {
        match subject.subject_data.read().unwrap().clone() {
            Some(subject_data) => {
                if subject_data.schema_id == "governance" {
                    Ok(Self {
                        api: subject.api.clone(),
                        governance_data: RwLock::new(subject_data),
                        runtime: subject.runtime.clone(),
                    })
                } else {
                    Err(TapleError::NotFound("Schema ID not valid".to_owned()))
                }
            }
            None => Err(TapleError::NotFound("Subject data not found".to_owned())),
        }
    }

    pub fn refresh(&self) -> Result<(), TapleError> {
        //Checks local SN and compares it to Taples version. If is underversioned it updates itself, if not it does nothing.
        self.runtime.block_on(async {
            let (subject_id, sn) = {
                let Ok(lock) = self.governance_data.read() else {
                    return Err(TapleError::LockIsPoisoned);
                };
                (lock.subject_id.clone(), lock.sn)
            };
            match self.api.get_subject(subject_id).await {
                Ok(api_sid) => {
                    if api_sid.sn == sn {
                        //Estado actualizado
                        Ok(())
                    } else {
                        //Actualizar estado
                        if sn < api_sid.sn {
                            let Ok(mut lock) = self.governance_data.write() else {
                                return Err(TapleError::LockIsPoisoned);
                            };
                            *lock = api_sid;
                        }
                        Ok(())
                    }
                }
                Err(error) => Err(TapleError::ExecutionError(error.to_string())),
            }
        })
    }

    pub fn get_members(&self) -> Result<Vec<String>, TapleError> {
        let governance_properties_value = &self.governance_data.read().unwrap().properties.0;

        let members_value = governance_properties_value
            .get("members")
            .unwrap()
            .as_array()
            .unwrap()
            .to_owned();

        let mut members_id: Vec<String> = vec![];
        for member in members_value.into_iter() {
            let id = match member.get("id") {
                Some(val) => match val.as_str() {
                    Some(id) => id,
                    None => return Err(TapleError::NoJSONString),
                },
                None => return Err(TapleError::NoJSONString),
            };

            members_id.push(String::from(id))
        }
        Ok(members_id)
    }

    fn get_from_properties<V: for<'a> Deserialize<'a>, S: Into<String>>(
        &self,
        property_name: S,
    ) -> Result<V, TapleError> {
        let governance_properties_value = &self.governance_data.read().unwrap().properties.0;

        let inner_value = governance_properties_value
            .get(property_name.into())
            .unwrap()
            .to_owned();

        Ok(serde_json::from_value(inner_value).map_err(|_| TapleError::DeserializationError)?)
    }

    pub fn get_policies(&self) -> Result<Vec<Policy>, TapleError> {
        self.get_from_properties("policies")
    }

    pub fn get_roles(&self) -> Result<Vec<Role>, TapleError> {
        self.get_from_properties("roles")
    }

    pub fn get_schemas(&self) -> Result<Vec<Schema>, TapleError> {
        let governance_properties_value = &self.governance_data.read().unwrap().properties.0;

        let schemas_value = governance_properties_value
            .get("schemas")
            .unwrap()
            .as_array()
            .unwrap()
            .to_owned();

        let mut schemas: Vec<Schema> = vec![];

        for schema in schemas_value {
            let id = match schema.get("id") {
                Some(id) => id
                    .as_str()
                    .ok_or(TapleError::IncorrectGovernanceProperties)?
                    .to_owned(),
                None => return Err(TapleError::NoJSONString),
            };

            let schema_data = match schema.get("schema") {
                Some(schema) => schema.to_string(),
                None => return Err(TapleError::NoJSONString),
            };

            let initial_value = match schema.get("initial_value") {
                Some(schema) => schema.to_string(),
                None => return Err(TapleError::NoJSONString),
            };

            schemas.push(Schema {
                id,
                schema: schema_data,
                initial_value,
            });
        }

        Ok(schemas)
    }

    pub fn get_subject_id(&self) -> String {
        self.governance_data.read().unwrap().subject_id.to_str()
    }

    pub fn get_governance_id(&self) -> String {
        self.governance_data.read().unwrap().governance_id.to_str()
    }

    pub fn get_sn(&self) -> u64 {
        self.governance_data.read().unwrap().sn
    }

    pub fn get_public_key(&self) -> String {
        self.governance_data.read().unwrap().public_key.to_str()
    }

    pub fn get_namespace(&self) -> String {
        self.governance_data.read().unwrap().namespace.clone()
    }

    pub fn get_schema_id(&self) -> String {
        self.governance_data.read().unwrap().schema_id.clone()
    }

    pub fn get_owner(&self) -> String {
        self.governance_data.read().unwrap().owner.to_str()
    }

    pub fn get_creator(&self) -> String {
        self.governance_data.read().unwrap().creator.to_str()
    }

    pub fn get_is_active(&self) -> bool {
        self.governance_data.read().unwrap().active
    }

    pub fn get_properties(&self) -> String {
        self.governance_data
            .read()
            .unwrap()
            .properties
            .0
            .to_string()
    }
}
