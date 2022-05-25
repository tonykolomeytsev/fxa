use image::io::Reader as ImageReader;
use image::{DynamicImage, EncodableLayout}; // Using image crate: https://github.com/image-rs/image
use webp::{Encoder, WebPMemory}; // Using webp crate: https://github.com/jaredforth/webp

use std::fs::File;
use std::io::Write;
use std::path::Path;

/*
    Function which converts an image in PNG or JPEG format to WEBP.
    :param file_path: &String with the path to the image to convert.
    :return Option<String>: Return the path of the WEBP-image as String when succesfull, returns None if function fails.
*/
pub fn image_to_webp(file_path: &String, quality: f32) -> Option<String> {
    // Open path as DynamicImage
    let image: DynamicImage = match ImageReader::open(file_path) {
        Ok(img) => img.with_guessed_format().unwrap().decode().unwrap(), //ImageReader::with_guessed_format() function guesses if image needs to be opened in JPEG or PNG format.
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };

    // Make webp::Encoder from DynamicImage.
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory.
    let encoded_webp: WebPMemory = encoder.encode(quality);

    // Put webp-image in a separate webp-folder in the location of the original image.
    let path: &Path = Path::new(file_path);

    // Get filename of original image.
    let filename_original_image = path.file_stem().unwrap().to_str().unwrap();

    // Make full output path for webp-image.
    let webp_image_path = format!("{}.webp", filename_original_image);

    // Make File-stream for WebP-result and write bytes into it, and save to path "output.webp".
    let mut webp_image = File::create(webp_image_path.to_string()).unwrap();
    match webp_image.write_all(encoded_webp.as_bytes()) {
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
        Ok(_) => return Some(webp_image_path),
    }
}
