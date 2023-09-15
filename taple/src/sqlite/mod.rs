use crate::{error::SQLiteError};
use std::fmt::Debug;
use taple_core::{DatabaseCollection, DatabaseManager};

#[derive(Debug)]
pub struct Tuple {
    pub key: String,
    pub value: Vec<u8>,
}

pub trait DbCollectionIteratorInterface: Send + Sync + Debug {
    fn next(&self) -> Result<Option<Tuple>, SQLiteError>;
}

pub trait DbCollectionInterface: Send + Sync + Debug {
    fn get(&self, key: String) -> Result<Option<Vec<u8>>, SQLiteError>;
    fn put(&self, key: String, value: Vec<u8>) -> Result<(), SQLiteError>;
    fn del(&self, key: String) -> Result<(), SQLiteError>;
    fn iter(&self, reverse: bool, prefix: String) -> Box<dyn DbCollectionIteratorInterface>;
}

pub struct WrapperIter {
    inner_iterator: Box<dyn DbCollectionIteratorInterface>,
}

impl Iterator for WrapperIter {
    type Item = (String, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        let tuple = self.inner_iterator.next();
        if let Ok(Some(data)) = tuple {
            return Some((data.key, data.value));
        }
        None
    }
}

pub trait DatabaseManagerInterface: Send + Sync + Debug {
    fn create_collection(&self, identifier: String) -> Box<dyn DbCollectionInterface>;
}

pub struct WrapperManager {
    pub inner_manager: Box<dyn DatabaseManagerInterface>,
}

impl DatabaseManager<WrapperCollection> for WrapperManager {
    //TODO: Revise default because no implementation can be performed
    fn default() -> Self {
        unimplemented!("Don't use the default method, you need to implement one for the platform you are developing for");
    }

    fn create_collection(&self, identifier: &str) -> WrapperCollection {
        WrapperCollection {
            inner_collection: self.inner_manager.create_collection(identifier.to_owned()),
        }
    }
}

pub struct WrapperCollection {
    inner_collection: Box<dyn DbCollectionInterface>,
}

impl DatabaseCollection for WrapperCollection {
    fn get(&self, key: &str) -> Result<Vec<u8>, taple_core::DbError> {
        match self.inner_collection.get(key.to_string()) {
            Ok(Some(data)) => return Ok(data),
            Ok(None) => return Err(taple_core::DbError::EntryNotFound),
            Err(err) => return Err(taple_core::DbError::CustomError(err.to_string())),
        }
    }

    fn put(&self, key: &str, data: Vec<u8>) -> Result<(), taple_core::DbError> {
        match self.inner_collection.put(key.to_string(), data) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(taple_core::DbError::CustomError(err.to_string())),
        }
    }

    fn del(&self, key: &str) -> Result<(), taple_core::DbError> {
        match self.inner_collection.del(key.to_string()) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(taple_core::DbError::CustomError(err.to_string())),
        }
    }

    fn iter<'a>(
        &'a self,
        reverse: bool,
        prefix: String,
    ) -> Box<dyn Iterator<Item = (String, Vec<u8>)> + 'a> {
        Box::new(WrapperIter {
            inner_iterator: self.inner_collection.iter(reverse, prefix),
        })
    }
}
