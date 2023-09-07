use std::{sync::{Arc, RwLock}};

use api::TapleAPI;
use models::{event::TapleSignedEvent, user_governance::UserGovernance, user_subject::UserSubject};
use notification::{TapleNotification};
use settings::TapleSettings;
use taple_core::{
    crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Secp256k1KeyPair},
    Node,
};
use tokio::runtime::Runtime;
mod api;
mod error;
mod models;
mod notification;
mod settings;
mod shutdown;
mod sqlite;
mod subject_builder;
mod node;

pub use shutdown::ShutdownSignal;
use std::fmt::Debug;

pub use error::{
    InitializationError, NotificationError, SQLiteError, SettingsError, ShutdownError, TapleError,
};
pub use models::approval::{
    TapleApprovalRequest, TapleApprovalResponse, TapleSignedApprovalRequest,
    TapleSignedApprovalResponse,
};
pub use models::approval_entity::{TapleApprovalEntity, TapleApprovalState};
pub use models::event::TapleEvent;
pub use models::others::{SubjectAndProviders, ValidationProofAndSignatures};
pub use models::policy::{Policy, Quorum, Validation};
pub use models::request::{
    EOLRequest, EventRequestType, TapleRequest, TapleRequestState, TapleSignedEventRequest,
    TransferRequest,
};
pub use models::role::{Role, RoleEnum, SchemaEnum, Who};
pub use models::schema::Schema;
pub use models::signature::TapleSignature;
pub use models::validation_proof::ValidationProof;
pub use sqlite::{
    DatabaseManagerInterface, DbCollectionInterface, DbCollectionIteratorInterface, Tuple,
};
pub use node::{TapleNode, NotificationHandlerInterface};
use sqlite::{WrapperManager};
use subject_builder::SubjectBuilder;

use crate::{models::others::TapleKeyDerivator};

pub fn generate_key(key_derivator: TapleKeyDerivator) -> Vec<u8> {
    match key_derivator {
        TapleKeyDerivator::Ed25519 => {
            let keypair = Ed25519KeyPair::from_seed(&[]);
            keypair.secret_key_bytes()
        }
        TapleKeyDerivator::Secp256k1 => {
            let keypair = Secp256k1KeyPair::from_seed(&[]);
            keypair.secret_key_bytes()
        }
    }
}

/// Funcion de start

pub fn start(
    manager: Box<dyn DatabaseManagerInterface>,
    settings: TapleSettings,
) -> Result<Arc<TapleNode>, InitializationError> {
    #[cfg(feature = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("taple"),
    );
    #[cfg(feature = "android")]
    log::debug!("Android Rust logger running");

    #[cfg(feature = "ios")]
    oslog::OsLogger::new("com.opencanarias.taple")
        .level_filter(log::LevelFilter::Debug)
        .category_level_filter("Settings", log::LevelFilter::Debug)
        .init()
        .unwrap();
    #[cfg(feature = "ios")]
    log::debug!("IOS Rust logger running");

    let keypair = match &settings.key_derivator {
        TapleKeyDerivator::Ed25519 => {
            KeyPair::Ed25519(Ed25519KeyPair::from_secret_key(&settings.private_key))
        }
        TapleKeyDerivator::Secp256k1 => {
            KeyPair::Secp256k1(Secp256k1KeyPair::from_secret_key(&settings.private_key))
        }
    };

    let settings = settings.try_into().unwrap();
    // .map_err(|e| InitializationError::InvalidSettings(e.to_string()))?;

    let (sx, mut rx) = tokio::sync::mpsc::channel::<()>(10);

    let rt: Arc<Runtime> = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let rt_start = rt.clone();

    rt_start.block_on(async {
        match Node::build(
            settings,
            WrapperManager {
                inner_manager: manager,
            },
        ) {
            Ok((taple, api)) => {
                taple.bind_with_shutdown(Box::pin(async move {
                    rx.recv().await;
                }));
                let node = Arc::new(TapleNode::new(
                    sx,
                    api,
                    keypair.clone(),
                    RwLock::new(Some(taple)),
                    rt.clone()
                ));
                Ok(node)
            }
            Err(error) => {
                Err(InitializationError::StartFailed(error.to_string()))
            }
        }
    })
}

uniffi::include_scaffolding!("taple_sdk");