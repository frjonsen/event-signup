use axum::http;

use crate::api::error::{error_codes, RestError};

#[derive(thiserror::Error, Debug)]
pub enum ImageUploadError {
    #[error("Image too large")]
    ImageTooLarge,
    #[error("Image too small")]
    ImageTooSmall,
    #[error("Can't guess image format")]
    ImageTypeGuessError,
    #[error("Image format not supported")]
    UnsupportedImageFormat,
    #[error("Image decoding error")]
    ImageDecodingError,
    #[error("Image encoding error")]
    ImageEncodingError,
    #[error("Storage error")]
    StorageError,
}

impl From<ImageUploadError> for RestError {
    fn from(val: ImageUploadError) -> Self {
        match val {
            ImageUploadError::UnsupportedImageFormat => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::UNSUPPORTED_IMAGE_FORMAT.to_string(),
                error_params: None,
            },
            ImageUploadError::ImageDecodingError => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_CONVERSION_ERROR.to_string(),
                error_params: None,
            },
            ImageUploadError::ImageEncodingError => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_CONVERSION_ERROR.to_string(),
                error_params: None,
            },
            ImageUploadError::ImageTypeGuessError => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_CONVERSION_ERROR.to_string(),
                error_params: None,
            },
            ImageUploadError::ImageTooLarge => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_TOO_LARGE.to_string(),
                error_params: None,
            },
            ImageUploadError::ImageTooSmall => RestError {
                status_code: http::StatusCode::BAD_REQUEST,
                error_code: error_codes::IMAGE_TOO_SMALL.to_string(),
                error_params: None,
            },
            ImageUploadError::StorageError => RestError {
                status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                error_code: error_codes::IMAGE_STORAGE_ERROR.to_string(),
                error_params: None,
            },
        }
    }
}
