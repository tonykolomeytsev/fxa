use image::io::Reader as ImageReader;
use image::{DynamicImage, EncodableLayout, ImageError};
use webp::Encoder;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::error::CommonError;

/// Converts PNG image to WEBP. Returns the path to the WEBP image, or an error with a description.
/// In case of successful conversion, the function will create a WEPB file next to the original PNG file.
///
/// # Arguments
///
/// * `file_path` - Path to PNG image
/// * `quality` - Encoding quality in percents (from 0 to 100). Pass 100% for loseless encoding
///
/// Function uses adopted code from: https://users.rust-lang.org/t/converting-png-jpeg-image-to-webp/71080
pub fn image_to_webp(file_path: &String, quality: f32) -> Result<String, CommonError> {
    ImageReader::open(file_path)
        .map_err(|e| e.into())
        .and_then(|reader| reader.with_guessed_format().map_err(|e| e.into()))
        .and_then(|reader| reader.decode().map_err(|e| e.into()))
        .and_then(|image: DynamicImage| {
            Encoder::from_image(&image)
                .map(|encoder| encoder.encode(quality))
                .map_err(|e| CommonError {
                    message: format!(
                        "encountered because image file {} has unsupported color model \
                    (8-bit RGB and RGBA are supported now)",
                        &file_path
                    ),
                    cause: Some(e.to_string()),
                })
        })
        .and_then(|webp_memory| {
            // Put webp-image in a separate webp-folder in the location of the original image.
            let original_image_file_name =
                Path::new(file_path).file_stem().unwrap().to_str().unwrap();
            // Make full output path for webp-image.
            let webp_image_path = format!("{}.webp", original_image_file_name);
            match File::create(webp_image_path.to_string()) {
                Ok(mut webp_image_file) => webp_image_file
                    .write_all(webp_memory.as_bytes())
                    .map_err(|e| e.into())
                    .map(|_| webp_image_path),
                Err(e) => Err(e.into()),
            }
        })
}

impl Into<CommonError> for ImageError {
    fn into(self) -> CommonError {
        CommonError {
            message: format!("{}", self),
            cause: None,
        }
    }
}
