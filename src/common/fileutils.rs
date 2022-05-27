use std::{fmt, fs, path::Path};

pub const TEMP_DIR_PATH: &str = ".fxn";

#[derive(Debug)]
pub struct FileUtilsError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for FileUtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nCaused by: {}", &self.message, &self.cause)
    }
}

pub fn create_temp_dir() -> Result<(), FileUtilsError> {
    fs::create_dir_all(TEMP_DIR_PATH).map_err(|e| FileUtilsError {
        message: "while creating temporary dir".to_string(),
        cause: format!("{}", e),
    })
}

pub fn remove_temp_dir() -> Result<(), FileUtilsError> {
    if Path::new(TEMP_DIR_PATH).exists() {
        fs::remove_dir_all(TEMP_DIR_PATH).map_err(|e| FileUtilsError {
            message: "while removing temporary dir".to_string(),
            cause: format!("{}", e),
        })
    } else {
        Ok(())
    }
}

pub fn create_dir(path: &String) -> Result<(), FileUtilsError> {
    fs::create_dir_all(path).map_err(|e| FileUtilsError {
        message: format!("while creating dir {}", &path),
        cause: format!("{}", e),
    })
}

pub fn move_file(from: &String, to: &String) -> Result<(), FileUtilsError> {
    fs::rename(from, to).map_err(|e| FileUtilsError {
        message: "while moving image from temporary dir to `res` dir".to_string(),
        cause: format!("{}", e),
    })
}
