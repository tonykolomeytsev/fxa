use std::{fs, path::Path};

use super::error::CommonError;

pub const TEMP_DIR_PATH: &str = ".fxn";

pub fn create_temp_dir() -> Result<(), CommonError> {
    fs::create_dir_all(TEMP_DIR_PATH).map_err(|e| CommonError {
        message: "while creating temporary dir".to_string(),
        cause: Some(format!("{}", e)),
    })
}

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

pub fn create_dir(path: &String) -> Result<(), CommonError> {
    fs::create_dir_all(path).map_err(|e| CommonError {
        message: format!("while creating dir {}", &path),
        cause: Some(format!("{}", e)),
    })
}

pub fn move_file(from: &String, to: &String) -> Result<(), CommonError> {
    fs::rename(from, to).map_err(|e| CommonError {
        message: "while moving image from temporary dir to `res` dir".to_string(),
        cause: Some(format!("{}", e)),
    })
}
