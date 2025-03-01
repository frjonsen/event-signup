use std::collections::HashMap;

use axum::http;

use crate::model::rest::{error_codes, RestError};

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
    #[error("Storage error")]
    StorageError,
}

impl From<ImageUploadError> for RestError {
    fn from(val: ImageUploadError) -> Self {
        match val {
            ImageUploadError::InvalidImage => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::INVALID_IMAGE.to_string(),
                error_params: None,
            },
            ImageUploadError::UnsupportedImageFormat { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::UNSUPPORTED_IMAGE_FORMAT.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageDecodingError { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_CONVERSION_ERROR.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageEncodingError { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_CONVERSION_ERROR.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageTypeGuessError { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_CONVERSION_ERROR.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageTooLarge { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_TOO_LARGE.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ReadError { image_name } => RestError {
                status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                error_code: error_codes::IMAGE_UPLOAD_FAILED.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::ImageTooSmall { image_name } => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_TOO_SMALL.to_string(),
                error_params: Some(HashMap::from([("image_name".to_string(), image_name)])),
            },
            ImageUploadError::StorageError => RestError {
                status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                error_code: error_codes::IMAGE_STORAGE_ERROR.to_string(),
                error_params: None,
            },
        }
    }
}
