use std::io::Error as IoError;

use png::{DecodingError, EncodingError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RasterError {
  #[error("decoding error: {0}")]
  DecodingError(#[from] DecodingError),

  #[error("encoding error: {0}")]
  EncodingError(#[from] EncodingError),

  #[error("io error: {0}")]
  IoError(#[from] IoError),

  #[error("wrong dimensions, expected {0} pixels got {1}")]
  WrongDimensions(usize, usize),
}
