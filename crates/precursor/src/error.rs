use std::fmt;
use std::io;

use crate_pixmap_png::{DecodeError, EncodeError};

use thiserror::Error;
use toml::de;

#[derive(Error, Debug)]
pub enum PrecursorError {
  #[error("{0}")]
  IoError(#[from] io::Error),

  #[error("{0}")]
  FmtError(#[from] fmt::Error),

  #[error("{0}")]
  DecodeError(#[from] DecodeError),

  #[error("{0}")]
  EncodeError(#[from] EncodeError),

  #[error("{0}")]
  TomlError(#[from] de::Error),

  #[error("invalid asset type, expected a PNG")]
  InvalidAssetType,
}

pub type PrecursorResult<T = ()> = Result<T, PrecursorError>;
