use crate_formats::raster::RasterError;
use thiserror::Error;

pub use std::fmt::Error as FmtError;
pub use std::io::Error as IoError;
pub use toml::de::Error as TomlError;

#[derive(Error, Debug)]
pub enum PrecursorError {
  #[error("fmt error: {0}")]
  FmtError(#[from] FmtError),

  #[error("io error: {0}")]
  IoError(#[from] IoError),

  #[error("raster error: {0}")]
  RasterError(#[from] RasterError),

  #[error("toml error: {0}")]
  TomlError(#[from] TomlError),
}

pub type PrecursorResult<T = ()> = Result<T, PrecursorError>;
