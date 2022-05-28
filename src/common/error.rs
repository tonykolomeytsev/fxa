use std::fmt;

/// Common struct for all errors in this app.
#[derive(Debug)]
pub struct CommonError {
    pub message: String,
    pub cause: Option<String>,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.cause {
            Some(cause) => write!(f, "{}\nCaused by: {}", self.message, cause),
            None => write!(f, "{}", self.message),
        }
    }
}

impl Into<CommonError> for std::io::Error {
    fn into(self) -> CommonError {
        CommonError {
            message: format!("{}", self),
            cause: None,
        }
    }
}

impl Into<CommonError> for serde_json::Error {
    fn into(self) -> CommonError {
        CommonError {
            message: format!("{}", self),
            cause: None,
        }
    }
}

impl Into<CommonError> for &str {
    fn into(self) -> CommonError {
        CommonError {
            message: self.to_string(),
            cause: None,
        }
    }
}
