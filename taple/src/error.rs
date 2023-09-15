use thiserror::Error;

#[derive(Error, Debug)]
pub enum TapleError {
    #[error("{0}")]
    ExecutionError(String),
    #[error("Node unavailable")]
    NodeUnavailable,
    #[error("{0}")]
    NotFound(String),
    #[error("Internal error")]
    InternalError,
    #[error("Digest Identifier generation failed")]
    DigestIdentifierGenerationFailed,
    #[error("Key Identifier generation failed")]
    KeyIdentifierGenerationFailed,
    #[error("Signature Identifier generation failed")]
    SignatureIdentifierGenerationFailed,
    #[error("Invalid KeyDerivator specified")]
    InvalidKeyDerivator,
    #[error("No JSON String")]
    NoJSONString,
    #[error("Signature generation failed: {0}")]
    SignatureGenerationFailed(String),
    #[error("Internal lock is poisoned")]
    LockIsPoisoned,
    #[error("Deserialization error")]
    DeserializationError,
    #[error("Incorrect format of governance properties")]
    IncorrectGovernanceProperties
}

impl From<uniffi::UnexpectedUniFFICallbackError> for TapleError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalError
    }
}

#[derive(Error, Debug)]
pub enum SQLiteError {
    #[error("General error")]
    KeyElementsError,
    #[error("Internal error")]
    InternalSQLiteError,
}

impl From<uniffi::UnexpectedUniFFICallbackError> for SQLiteError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalSQLiteError
    }
}

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Connection with TAPLE node has been closed")]
    NoConnection,
    #[error("Internal Error")]
    InternalError,
    #[error("Internal lock is poisoned")]
    LockIsPoisoned,
}

impl From<uniffi::UnexpectedUniFFICallbackError> for NotificationError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalError
    }
}

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Invalid ListenAddr")]
    InvalidListenAddr,
    #[error("Internal Error")]
    InternalError,
}

impl From<uniffi::UnexpectedUniFFICallbackError> for SettingsError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalError
    }
}

#[derive(Error, Debug)]
pub enum InitializationError {
    #[error("Invalid Settings: {0}")]
    InvalidSettings(String),
    #[error("Start process failed {0}")]
    StartFailed(String),
    #[error("Internal Error")]
    InternalError,
}

impl From<uniffi::UnexpectedUniFFICallbackError> for InitializationError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalError
    }
}

#[derive(Error, Debug)]
pub enum ShutdownError {
    #[error("Inner lock is poisoned")]
    InnerLockIsPoisoned,
    #[error("Internal Error")]
    InternalError,
}

impl From<uniffi::UnexpectedUniFFICallbackError> for ShutdownError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalError
    }
}
