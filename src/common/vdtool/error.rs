use thiserror::Error;

#[derive(Debug, Error)]
pub enum Svg2VectorError {
    #[error("Can't read file {0}. Cause: {1}")]
    CannotReadSvg(String, String),

    #[error("Can't parse file {0}. Cause: {1}")]
    CannotParseSvg(String, String),

    #[error("Invalid content in svg file {0}. Root tag isn't <svg> tag.")]
    InvalidContentNotRoot(String),

    #[error("Invalid dimensions in <svg> tag")]
    InvalidDimensionSvgTag,
}
