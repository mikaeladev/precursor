use std::fmt::Error as FmtError;
use std::io::Error as IoError;

use crate_formats::rasters::RasterError;

use thiserror::Error;
use toml::de::Error as TomlError;

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

  #[error("invalid asset type, expected a PNG")]
  InvalidAssetType,
}

pub type PrecursorResult<T = ()> = Result<T, PrecursorError>;
