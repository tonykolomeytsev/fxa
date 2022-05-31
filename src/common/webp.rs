use image::io::Reader as ImageReader;
use image::EncodableLayout;
use webp::Encoder;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::error::AppError;

/// Converts PNG image to WEBP. Returns the path to the WEBP image, or an error with a description.
/// In case of successful conversion, the function will create a WEPB file next to the original PNG file.
///
/// # Arguments
///
/// * `file_path` - Path to PNG image
/// * `quality` - Encoding quality in percents (from 0 to 100). Pass 100% for loseless encoding
///
/// Function uses adopted code from: https://users.rust-lang.org/t/converting-png-jpeg-image-to-webp/71080
pub fn image_to_webp(file_path: &String, quality: f32) -> Result<String, AppError> {
    let reader = ImageReader::open(file_path)
        .map_err(|_| AppError::SourceNotFound(file_path.clone()))?
        .with_guessed_format()
        .map_err(|_| AppError::UnderlyingReader(file_path.clone()))?;

    let image = reader
        .decode()
        .map_err(|_| AppError::CannotDecode(file_path.clone()))?;

    let encoder =
        Encoder::from_image(&image).map_err(|_| AppError::CannotEncode(file_path.clone()))?;

    // Create webp encoded image in RAM
    let webp_memory = encoder.encode(quality);
    // Put webp-image in a separate webp-folder in the location of the original image.
    let original_image_file_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap();
    // Make full output path for webp-image.
    let webp_image_path = format!("{}.webp", original_image_file_name);

    File::create(webp_image_path.to_string())
        .map_err(|e| AppError::WriteWebpTemporarySave(format!("{}", e)))?
        .write_all(webp_memory.as_bytes())
        .map_err(|e| AppError::WriteWebpTemporarySave(format!("{}", e)))?;

    Ok(webp_image_path)
}
