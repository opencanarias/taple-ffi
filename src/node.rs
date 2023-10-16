use std::sync::{Arc, RwLock};

use taple_core::{crypto::KeyPair, Api, DigestDerivator, Node};
use tokio::runtime::Runtime;

use crate::{
    api::{create_taple_api, TapleAPI},
    db::{WrapperCollection, WrapperManager},
    notification::TapleNotification,
    shutdown::ShutdownSignal,
    subject_builder::SubjectBuilder,
    NotificationError, TapleError,
};

pub trait NotificationHandlerInterface {
    fn process_notification(&self, notification: TapleNotification);
}

pub struct TapleNode {
    shutdown_sender: tokio::sync::mpsc::Sender<()>,
    api: Api,
    keypair: KeyPair,
    taple: RwLock<Option<Node<WrapperManager, WrapperCollection>>>,
    runtime: Arc<Runtime>,
    derivator: DigestDerivator,
}

impl TapleNode {
    pub fn new(
        shutdown_sender: tokio::sync::mpsc::Sender<()>,
        api: Api,
        keypair: KeyPair,
        taple: RwLock<Option<Node<WrapperManager, WrapperCollection>>>,
        runtime: Arc<Runtime>,
        derivator: DigestDerivator,
    ) -> Self {
        Self {
            shutdown_sender,
            api,
            keypair,
            taple,
            runtime,
            derivator,
        }
    }

    pub fn get_api(&self) -> Arc<TapleAPI> {
        Arc::new(create_taple_api(
            self.api.clone(),
            self.runtime.clone(),
            self.keypair.clone(),
            self.derivator,
        ))
    }

    pub fn receive_blocking(&self) -> Result<TapleNotification, NotificationError> {
        self.runtime.block_on(async {
            let mut write_lock = self
                .taple
                .write()
                .map_err(|_| NotificationError::LockIsPoisoned)?;
            if write_lock.is_none() {
                return Err(NotificationError::NoConnection);
            }
            let notification = write_lock
                .as_mut()
                .unwrap()
                .recv_notification()
                .await
                .ok_or(NotificationError::NoConnection)?;
            Ok(notification.into())
        })
    }

    pub fn drop_notifications(&self) -> Result<(), TapleError> {
        let node = {
            let mut write_lock = self.taple.write().map_err(|_| TapleError::LockIsPoisoned)?;
            if write_lock.is_none() {
                return Err(TapleError::NodeUnavailable);
            }
            write_lock.take().unwrap()
        };
        self.runtime.block_on(async {
            node.handle_notifications(|_| {}).await;
        });
        Ok(())
    }

    pub fn handle_notifications(
        &self,
        handler: Box<dyn NotificationHandlerInterface>,
    ) -> Result<(), TapleError> {
        let node = {
            let mut write_lock = self.taple.write().map_err(|_| TapleError::LockIsPoisoned)?;
            if write_lock.is_none() {
                return Err(TapleError::NodeUnavailable);
            }
            write_lock.take().unwrap()
        };
        self.runtime.block_on(async {
            node.handle_notifications(|notification| {
                handler.process_notification(TapleNotification::from(notification));
            })
            .await;
        });
        Ok(())
    }

    pub fn get_shutdown_handler(&self) -> Arc<ShutdownSignal> {
        Arc::new(ShutdownSignal {
            runtime: self.runtime.clone(),
            shutdown: RwLock::new(self.shutdown_sender.clone()),
        })
    }

    pub fn shutdown_gracefully(&self) -> Result<(), TapleError> {
        let node = {
            let mut write_lock = self.taple.write().map_err(|_| TapleError::LockIsPoisoned)?;
            if write_lock.is_none() {
                return Err(TapleError::NodeUnavailable);
            }
            write_lock.take().unwrap()
        };
        self.runtime.block_on(async {
            node.shutdown_gracefully().await;
        });
        Ok(())
    }

    pub fn get_subject_builder(&self) -> Arc<SubjectBuilder> {
        let sb_api = self.get_api().api.clone();
        let sb_runtime = self.runtime.clone();
        let sb_keys = self.keypair.clone();
        Arc::new(SubjectBuilder {
            api: sb_api,
            runtime: sb_runtime,
            keys: sb_keys,
            name: RwLock::new(None),
            namespace: RwLock::new(None),
            derivator: self.derivator,
        })
    }
}
