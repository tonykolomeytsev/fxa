use std::{fs, path::Path};

pub const TEMP_DIR_PATH: &str = ".fxa";

/// Create temporary directory `.fxn` at the root of the working directory
/// (where the app was launched from).
pub fn create_temp_dir() -> Result<(), std::io::Error> {
    fs::create_dir_all(TEMP_DIR_PATH)
}

/// Remove temporary directory `.fxn` from the root of the working directory
/// (where the app was launched from).
pub fn remove_temp_dir() -> Result<(), std::io::Error> {
    if Path::new(TEMP_DIR_PATH).exists() {
        fs::remove_dir_all(TEMP_DIR_PATH)
    } else {
        Ok(())
    }
}

/// Recursively create all directories given in the path.
///
/// # Arguments
///
/// * `path` - Path to the directory to be created
pub fn create_dir(path: &String) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

/// Move file from one place to another, replacing the original file if `to` already exists.
/// This will not work if the new name is on a different mount point.
pub fn move_file(from: &String, to: &String) -> Result<(), std::io::Error> {
    fs::rename(from, to)
}
