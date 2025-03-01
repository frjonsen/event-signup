use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    events::errors::{AddImagesError, GetEventError},
    model::database::errors::{DatabaseQueryFailed, UnknownSdkError},
};

pub struct RestError {
    pub status_code: StatusCode,
    pub error_code: String,
    pub error_params: Option<HashMap<String, String>>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestErrorBody {
    pub error_code: String,
    pub error_params: Option<HashMap<String, String>>,
}

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(RestErrorBody {
                error_code: self.error_code,
                error_params: self.error_params,
            }),
        )
            .into_response()
    }
}

impl Into<(StatusCode, RestErrorBody)> for RestError {
    fn into(self) -> (StatusCode, RestErrorBody) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            RestErrorBody {
                error_code: self.error_code,
                error_params: self.error_params,
            },
        )
    }
}

impl From<DatabaseQueryFailed> for RestError {
    fn from(_val: DatabaseQueryFailed) -> Self {
        RestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: error_codes::UNEXPECTED_SERVER_ERROR.to_string(),
            error_params: None,
        }
    }
}

impl From<UnknownSdkError> for RestError {
    fn from(_val: UnknownSdkError) -> Self {
        RestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: error_codes::UNEXPECTED_SERVER_ERROR.to_string(),
            error_params: None,
        }
    }
}

impl From<GetEventError> for RestError {
    fn from(val: GetEventError) -> Self {
        match val {
            GetEventError::NotFound => RestError {
                status_code: axum::http::StatusCode::NOT_FOUND,
                error_code: error_codes::EVENT_NOT_FOUND.to_string(),
                error_params: None,
            },
            GetEventError::InvalidStoredEvent(id) => RestError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                error_code: error_codes::INVALID_STORED_EVENT.to_string(),
                error_params: Some(HashMap::from_iter(vec![("id".to_string(), id.to_string())])),
            },
            GetEventError::DatabaseQueryFailed(e) => e.into(),
            GetEventError::UnexpectedSdkError(e) => e.into(),
        }
    }
}

impl From<AddImagesError> for RestError {
    fn from(val: AddImagesError) -> Self {
        match val {
            AddImagesError::DatabaseQueryFailed(e) => e.into(),
            AddImagesError::UnexpectedSdkError(e) => e.into(),
            AddImagesError::GetEventError(e) => e.into(),
        }
    }
}

pub mod error_codes {
    pub const UNEXPECTED_SERVER_ERROR: &str = "UNEXPECTED_SERVER_ERROR";
    pub const EVENT_NOT_FOUND: &str = "EVENT_NOT_FOUND";
    pub const IMAGE_UPLOAD_FAILED: &str = "IMAGE_UPLOAD_FAILED";
    pub const INVALID_IMAGE: &str = "INVALID_IMAGE";
    pub const UNSUPPORTED_IMAGE_FORMAT: &str = "UNSUPPORTED_IMAGE_FORMAT";
    pub const IMAGE_CONVERSION_ERROR: &str = "IMAGE_CONVERSION_ERROR";
    pub const IMAGE_TOO_LARGE: &str = "IMAGE_TOO_LARGE";
    pub const IMAGE_TOO_SMALL: &str = "IMAGE_TOO_SMALL";
    pub const IMAGE_STORAGE_ERROR: &str = "IMAGE_STORAGE_ERROR";
    pub const INVALID_STORED_EVENT: &str = "INVALID_STORED_EVENT";
}
