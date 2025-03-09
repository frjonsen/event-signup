mod error;
mod event;
mod post_images_response;
pub use error::*;
pub use event::*;
pub use post_images_response::*;

use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    response::IntoResponse,
};
use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(RestError))]
pub struct Json<T>(T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl From<JsonRejection> for RestError {
    fn from(value: JsonRejection) -> Self {
        Self {
            status_code: value.status(),
            error_code: "invalid_json".to_string(),
            error_params: None,
        }
    }
}
