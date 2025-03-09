use std::io::Cursor;

use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use errors::ImageUploadError;
use image::{DynamicImage, GenericImageView};
use tracing::info;
use uuid::Uuid;

use crate::configuration::{EVENT_IMAGES_BUCKET_NAME, EVENT_IMAGES_BUCKET_PREFIX};

pub mod errors;

const MAX_IMAGE_DIMENSION: u32 = 1280;
const MIN_IMAGE_DIMENSION: u32 = 800;

#[derive(Debug, PartialEq)]
pub enum ImageType {
    Jpeg,
    Png,
    Avif,
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Jpeg => write!(f, "image/jpeg"),
            ImageType::Png => write!(f, "image/png"),
            ImageType::Avif => write!(f, "image/avif"),
        }
    }
}

impl TryFrom<&str> for ImageType {
    type Error = ImageUploadError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "image/jpeg" => Ok(ImageType::Jpeg),
            "image/png" => Ok(ImageType::Png),
            "image/avif" => Ok(ImageType::Avif),
            _ => Err(ImageUploadError::UnsupportedImageFormat {}),
        }
    }
}
pub fn is_image_within_bounds(image: &DynamicImage) -> bool {
    let size = image.dimensions();
    size.0 < MAX_IMAGE_DIMENSION && size.1 < MAX_IMAGE_DIMENSION
}

pub fn is_image_too_small(image: &DynamicImage) -> bool {
    let size = image.dimensions();
    size.0 < MIN_IMAGE_DIMENSION || size.1 < MIN_IMAGE_DIMENSION
}

pub fn assert_image_size(image: DynamicImage) -> DynamicImage {
    if is_image_within_bounds(&image) {
        return image;
    }

    info!(
        "Resizing image to fit within {}x{}",
        MAX_IMAGE_DIMENSION, MAX_IMAGE_DIMENSION
    );
    image.resize(
        MAX_IMAGE_DIMENSION,
        MAX_IMAGE_DIMENSION,
        image::imageops::FilterType::CatmullRom,
    )
}

pub async fn conform_image(image: DynamicImage) -> Result<Vec<u8>, ImageUploadError> {
    let incoming_image = assert_image_size(image);

    info!("Encoding file as avif");
    let mut encoded_image: Vec<u8> = Vec::new();
    incoming_image
        .write_to(
            &mut Cursor::new(&mut encoded_image),
            image::ImageFormat::Avif,
        )
        .map_err(|e| {
            sentry::capture_error(&e);
            ImageUploadError::ImageEncodingError {}
        })?;
    info!("Encoded file as avif");

    Ok(encoded_image)
}

pub async fn upload_image(
    s3: &aws_sdk_s3::Client,
    event: Uuid,
    image: Vec<u8>,
) -> Result<Uuid, ImageUploadError> {
    let new_image_id = Uuid::new_v4();

    let body = SdkBody::from(image);
    let image = ByteStream::from(body);
    let path = format!(
        "{prefix}/{event}/{id}.avif",
        prefix = &*EVENT_IMAGES_BUCKET_PREFIX,
        event = event,
        id = new_image_id
    );
    s3.put_object()
        .bucket(&*EVENT_IMAGES_BUCKET_NAME)
        .key(path)
        .body(image)
        .send()
        .await
        .map_err(|e| {
            sentry::capture_error(&e);
            ImageUploadError::StorageError
        })?;

    Ok(new_image_id)
}
