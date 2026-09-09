use std::io;

use crate_pixmap::PixmapError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RasterError {
  #[error("io error: {0}")]
  IoError(#[from] io::Error),

  #[error("pixmap error: {0}")]
  PixmapError(#[from] PixmapError),

  #[error("png decoding error: {0}")]
  PngDecodeError(#[from] png::DecodingError),

  #[error("png encoding error: {0}")]
  PngEncodeError(#[from] png::EncodingError),
}

pub type RasterResult<T = ()> = Result<T, RasterError>;
