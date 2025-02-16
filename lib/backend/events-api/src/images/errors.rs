use std::collections::HashMap;

use axum::http;

use crate::model::rest::RestError;

#[derive(thiserror::Error, Debug)]
pub enum ImageUploadError {
    #[error("Invalid image")]
    InvalidImage,
    #[error("Read error")]
    ReadError { image_name: String },
    #[error("Image too large")]
    ImageTooLarge { image_name: String },
    #[error("Image too small")]
    ImageTooSmall { image_name: String },
    #[error("Can't guess image format")]
    ImageTypeGuessError { image_name: String },
    #[error("Image format not supported")]
    UnsupportedImageFormat { image_name: String },
    #[error("Image decoding error")]
    ImageDecodingError { image_name: String },
    #[error("Image encoding error")]
    ImageEncodingError { image_name: String },
}

impl Into<RestError> for ImageUploadError {
    fn into(self) -> RestError {
        match self {
            ImageUploadError::InvalidImage => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "INVALID_IMAGE".to_string(),
                error_params: None,
            },
            ImageUploadError::UnsupportedImageFormat { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "UNSUPPORTED_IMAGE_FORMAT".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageDecodingError { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "IMAGE_DECODING_ERROR".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageEncodingError { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "IMAGE_ENCODING_ERROR".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageTypeGuessError { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "IMAGE_TYPE_GUESS_ERROR".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageTooLarge { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "IMAGE_TOO_LARGE".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ReadError { image_name } => RestError {
                status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                error_code: "IMAGE_READ_ERROR".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageTooSmall { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: "IMAGE_TOO_SMALL".to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
        }
    }
}
