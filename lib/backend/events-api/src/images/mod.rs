use std::io::Cursor;

use errors::ImageUploadError;
use image::{DynamicImage, GenericImageView};
use tracing::info;

pub mod errors;

const MAX_IMAGE_DIMENSION: u32 = 1920;
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
            _ => Err(ImageUploadError::UnsupportedImageFormat {
                image_name: value.to_owned(),
            }),
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

pub async fn conform_image(name: &str, image: DynamicImage) -> Result<Vec<u8>, ImageUploadError> {
    let incoming_image = assert_image_size(image);

    info!("Encoding file {} as avif", name);
    let mut encoded_image: Vec<u8> = Vec::new();
    incoming_image
        .write_to(
            &mut Cursor::new(&mut encoded_image),
            image::ImageFormat::Avif,
        )
        .map_err(|e| {
            sentry::capture_error(&e);
            ImageUploadError::ImageEncodingError {
                image_name: name.to_owned(),
            }
        })?;
    info!("Encoded file {} as avif", name);

    Ok(encoded_image)
}
