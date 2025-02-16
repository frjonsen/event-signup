use axum::response::IntoResponse;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PostImagesResponse {
    pub images: Vec<String>,
    pub event: Uuid,
}

impl IntoResponse for PostImagesResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
