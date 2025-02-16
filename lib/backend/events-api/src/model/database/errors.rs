use crate::model::rest::RestError;

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

impl Into<RestError> for ModelError {
    fn into(self) -> RestError {
        RestError {
            status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            error_code: "INVALID_MODEL".to_string(),
            error_params: None,
        }
    }
}
