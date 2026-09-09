use std::fmt;
use std::io;

use crate_formats::rasters::RasterError;

use thiserror::Error;
use toml::de;

#[derive(Error, Debug)]
pub enum PrecursorError {
  #[error("fmt error: {0}")]
  FmtError(#[from] fmt::Error),

  #[error("io error: {0}")]
  IoError(#[from] io::Error),

  #[error("raster error: {0}")]
  RasterError(#[from] RasterError),

  #[error("toml error: {0}")]
  TomlError(#[from] de::Error),

  #[error("invalid asset type, expected a PNG")]
  InvalidAssetType,
}

pub type PrecursorResult<T = ()> = Result<T, PrecursorError>;
