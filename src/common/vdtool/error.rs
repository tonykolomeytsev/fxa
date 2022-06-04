use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VectorDrawableError {
    #[error("Can't read file {0}. Cause: {1}")]
    CannotReadSvg(String, String),

    #[error("Can't parse file {0}. Cause: {1}")]
    CannotParseSvg(String, String),

    #[error("Invalid dimensions in <svg> tag")]
    InvalidDimensionSvgTag,

    #[error("Can't write to output xml file")]
    CannotWrite(#[from] io::Error),
}
