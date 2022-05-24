use std::{fs, io};

use crate::models::config::ImageFormat;

pub const TEMP_DIR_PATH: &str = ".fxn";

pub fn create_temp_dir() -> io::Result<()> {
    fs::create_dir_all(TEMP_DIR_PATH)
}

pub fn save_image(
    res_path: &String,
    image_name: &String,
    image_format: ImageFormat,
    scale: f32,
) -> io::Result<()> {
    Ok(())
}
