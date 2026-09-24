use crate_pixmap::PixmapError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
  #[error("pixmap error: {0}")]
  PixmapError(#[from] PixmapError),
  #[error("png error: {0}")]
  PngError(#[from] png::DecodingError),
}

pub type DecodeResult<T> = Result<T, DecodeError>;

pub type EncodeError = png::EncodingError;

pub type EncodeResult<T> = Result<T, EncodeError>;
