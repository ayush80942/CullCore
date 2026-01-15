use thiserror::Error;

#[derive(Error, Debug)]
pub enum CullError {
    #[error("Failed to load image: {0}")]
    ImageLoad(String),

    #[error("Invalid input path")]
    InvalidPath,

    #[error("Unsupported image format")]
    UnsupportedFormat,
}
