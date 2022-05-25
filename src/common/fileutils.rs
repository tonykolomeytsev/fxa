use std::{fs, io};

pub const TEMP_DIR_PATH: &str = ".fxn";

pub fn create_temp_dir() -> io::Result<()> {
    fs::create_dir_all(TEMP_DIR_PATH)
}
