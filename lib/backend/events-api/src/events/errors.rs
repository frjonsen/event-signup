use crate::database::errors::{DatabaseQueryFailed, UnknownSdkError};

#[derive(thiserror::Error, Debug)]
pub enum GetEventError {
    #[error("Event not found")]
    NotFound,
    #[error("Failed to read event")]
    InvalidStoredEvent(uuid::Uuid),
    #[error(transparent)]
    DatabaseQueryFailed(#[from] DatabaseQueryFailed),
    #[error(transparent)]
    UnexpectedSdkError(#[from] UnknownSdkError),
}

#[derive(thiserror::Error, Debug)]
pub enum AddImageError {
    #[error(transparent)]
    DatabaseQueryFailed(#[from] DatabaseQueryFailed),
    #[error(transparent)]
    UnexpectedSdkError(#[from] UnknownSdkError),
    #[error(transparent)]
    GetEventError(#[from] GetEventError),
}
