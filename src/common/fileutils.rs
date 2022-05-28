use std::{fs, path::Path};

use super::error::CommonError;

pub const TEMP_DIR_PATH: &str = ".fxn";

/// Create temporary directory `.fxn` at the root of the working directory
/// (where the app was launched from).
pub fn create_temp_dir() -> Result<(), CommonError> {
    fs::create_dir_all(TEMP_DIR_PATH).map_err(|e| CommonError {
        message: "while creating temporary dir".to_string(),
        cause: Some(format!("{}", e)),
    })
}

/// Remove temporary directory `.fxn` from the root of the working directory
/// (where the app was launched from).
pub fn remove_temp_dir() -> Result<(), CommonError> {
    if Path::new(TEMP_DIR_PATH).exists() {
        fs::remove_dir_all(TEMP_DIR_PATH).map_err(|e| CommonError {
            message: "while removing temporary dir".to_string(),
            cause: Some(format!("{}", e)),
        })
    } else {
        Ok(())
    }
}

/// Recursively create all directories given in the path.
///
/// # Arguments
///
/// * `path` - Path to the directory to be created
pub fn create_dir(path: &String) -> Result<(), CommonError> {
    fs::create_dir_all(path).map_err(|e| CommonError {
        message: format!("while creating dir {}", &path),
        cause: Some(format!("{}", e)),
    })
}

/// Move file from one place to another, replacing the original file if `to` already exists.
/// This will not work if the new name is on a different mount point.
pub fn move_file(from: &String, to: &String) -> Result<(), CommonError> {
    fs::rename(from, to).map_err(|e| CommonError {
        message: "while moving image from temporary dir to `res` dir".to_string(),
        cause: Some(format!("{}", e)),
    })
}
