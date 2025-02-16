use std::io::Cursor;

use axum::{
    extract::{multipart::Field, Multipart},
    http,
};
use bytes::BytesMut;
use image::{DynamicImage, ImageReader};
use tracing::info;

use crate::{
    images::{
        errors::ImageUploadError, is_image_too_small, is_image_within_bounds, reencode_image,
        ImageType,
    },
    model::rest::{PostImagesResponse, RestError},
};

// 10mb
const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 10;

async fn read_file_with_limit(
    name: &str,
    mut field: Field<'_>,
) -> Result<DynamicImage, ImageUploadError> {
    let mut read_so_far = 0usize;
    let mut field_bytes = BytesMut::new();
    info!("Reading file {}", name);
    loop {
        match field.chunk().await {
            Err(e) => {
                sentry::capture_error(&e);
                return Err(ImageUploadError::ReadError {
                    image_name: name.to_owned(),
                });
            }
            Ok(Some(chunk)) => {
                read_so_far += chunk.len();
                if read_so_far > MAX_IMAGE_SIZE {
                    return Err(ImageUploadError::ImageTooLarge {
                        image_name: name.to_owned(),
                    });
                }

                field_bytes.extend_from_slice(&chunk);
            }
            Ok(None) => break,
        }
    }

    info!("Decoding file {}", name);
    let image = ImageReader::new(Cursor::new(field_bytes))
        .with_guessed_format()
        .map_err(|_| ImageUploadError::ImageTypeGuessError {
            image_name: name.to_owned(),
        })?
        .decode()
        .map_err(|_| ImageUploadError::ImageDecodingError {
            image_name: name.to_owned(),
        })?;

    info!("Decoded file {}", name);
    Ok(image)
}

async fn handle_image(field: Field<'_>) -> Result<Vec<u8>, ImageUploadError> {
    let name = match field.file_name() {
        Some(name) => name.to_owned(),
        None => return Err(ImageUploadError::InvalidImage),
    };
    let content_type: ImageType = match field.content_type() {
        Some(content_type) => content_type.try_into()?,
        None => return Err(ImageUploadError::UnsupportedImageFormat { image_name: name }),
    };
    info!("Handling file {} of type {}", name, content_type);

    let incoming_image = read_file_with_limit(&name, field).await?;
    if is_image_too_small(&incoming_image) {
        return Err(ImageUploadError::ImageTooSmall { image_name: name });
    }
    if content_type == ImageType::Avif && is_image_within_bounds(&incoming_image) {
        info!("Image is already of the right type and acceptable size. Returning as is.");
        return Ok(Vec::from(incoming_image.as_bytes()));
    } else {
        reencode_image(&name, incoming_image).await
    }
}

pub async fn post_image(mut multipart: Multipart) -> Result<PostImagesResponse, RestError> {
    loop {
        match multipart.next_field().await {
            Ok(Some(file)) => {
                let _ = handle_image(file).await.map_err(|e| e.into())?;
            }
            Ok(None) => break,
            Err(e) => {
                sentry::capture_error(&e);
                return Err(RestError {
                    status_code: http::StatusCode::BAD_REQUEST,
                    error_code: "MULTIPART_READ_ERROR".to_string(),
                    error_params: None,
                });
            }
        }
    }
    Ok(PostImagesResponse {
        event: uuid::Uuid::new_v4(),
        images: vec!["image.jpg".to_string()],
    })
}
