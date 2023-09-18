use std::sync::{Arc, RwLock};

use tokio::runtime::Runtime;

use crate::error::ShutdownError;

pub struct ShutdownSignal {
    pub runtime: Arc<Runtime>,
    pub shutdown: RwLock<tokio::sync::mpsc::Sender<()>>,
}

impl ShutdownSignal {
    pub fn shutdown(&self) -> Result<(), ShutdownError> {
        let write_lock = self
            .shutdown
            .write()
            .map_err(|_| ShutdownError::InnerLockIsPoisoned)?;
        self.runtime.block_on(async {
            let _ = write_lock.send(()).await;
        });
        Ok(())
    }
}
