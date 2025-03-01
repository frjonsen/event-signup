use crate::model::database::errors::{DatabaseQueryFailed, UnknownSdkError};

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
