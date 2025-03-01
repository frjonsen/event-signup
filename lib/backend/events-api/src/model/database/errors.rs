#[derive(Debug, thiserror::Error)]
#[error("Failed to query database")]
pub struct DatabaseQueryFailed;

#[derive(Debug, thiserror::Error)]
#[error("SDK returned an unexpected response: {0}")]
pub struct UnknownSdkError(pub String);

#[derive(thiserror::Error, Debug)]
pub enum ModelError {
    #[error("Model has invalid values: {0}")]
    InvalidData(String),
    #[error("Field is missing: {0}")]
    MissingField(String),
    #[error("Field {0} is not a {1}: {2}")]
    InvalidType(String, String, String),
    #[error("Field {0} is not of the expected type: {1}")]
    InvalidGenericType(String, String),
    #[error("Field {0} is missing delimiter")]
    MissingDelimiter(String),
}
