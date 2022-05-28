use std::fmt;

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
