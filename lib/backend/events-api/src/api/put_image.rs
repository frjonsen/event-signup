use std::io::Cursor;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::{headers::ContentType, TypedHeader};
use bytes::Bytes;
use image::ImageReader;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    authentication::Claims,
    events::queries::DynamodbQueries,
    images::{conform_image, errors::ImageUploadError, is_image_too_small, upload_image},
};

use super::error::{NotEventOwnerError, RestError};

const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 10;

#[derive(Serialize)]
pub struct PutImageResponse {
    image_id: Uuid,
}

impl IntoResponse for PutImageResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

fn get_image_from_body(body: Bytes) -> Result<image::DynamicImage, ImageUploadError> {
    tracing::debug!("Reading image from body");
    let cursor = Cursor::new(body);
    let image = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| {
            sentry::capture_error(&e);
            tracing::debug!("Failed to guess image format");
            ImageUploadError::ImageTypeGuessError
        })?
        .decode()
        .map_err(|e| {
            sentry::capture_error(&e);
            tracing::error!("Failed to decode image: {:?}", e);
            ImageUploadError::ImageDecodingError
        })?;
    tracing::debug!("Image read from body");

    Ok(image)
}

pub async fn put_image(
    State(s3): State<aws_sdk_s3::Client>,
    State(dynamodb): State<DynamodbQueries>,
    Path(event_id): Path<Uuid>,
    claims: Claims,
    TypedHeader(content_type): TypedHeader<ContentType>,
    body: Bytes,
) -> Result<PutImageResponse, RestError> {
    let event = dynamodb.get_event(event_id).await?;

    if event.creator_username != claims.username {
        return Err(NotEventOwnerError.into());
    }

    if body.len() > MAX_IMAGE_SIZE {
        return Err(ImageUploadError::ImageTooLarge.into());
    }
    if content_type != ContentType::png() && content_type != ContentType::jpeg() {
        return Err(ImageUploadError::UnsupportedImageFormat.into());
    }

    let image = get_image_from_body(body)?;
    if is_image_too_small(&image) {
        return Err(ImageUploadError::ImageTooSmall.into());
    }

    let conformed_image = conform_image(image).await?;
    let image_id = upload_image(&s3, event_id, conformed_image).await?;
    dynamodb.set_event_image(event_id, image_id).await?;

    Ok(PutImageResponse { image_id })
}
