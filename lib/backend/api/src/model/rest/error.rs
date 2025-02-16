use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};

pub struct RestError {
    pub status_code: StatusCode,
    pub error_code: String,
    pub error_params: Option<HashMap<String, String>>,
}

#[derive(serde::Serialize)]
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

pub struct EventNotFoundError(pub String);

impl Into<RestError> for EventNotFoundError {
    fn into(self) -> RestError {
        let mut params = HashMap::new();
        params.insert("event_id".to_string(), self.0);
        RestError {
            status_code: StatusCode::NOT_FOUND,
            error_code: "NOT_FOUND".to_string(),
            error_params: Some(params),
        }
    }
}

pub struct DatabaseQueryFailed;

impl Into<RestError> for DatabaseQueryFailed {
    fn into(self) -> RestError {
        RestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: "DATABASE_QUERY_FAILED".to_string(),
            error_params: None,
        }
    }
}

pub struct UnknownSdkError;

impl Into<RestError> for UnknownSdkError {
    fn into(self) -> RestError {
        RestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: "UNKNOWN_SDK_ERROR".to_string(),
            error_params: None,
        }
    }
}
