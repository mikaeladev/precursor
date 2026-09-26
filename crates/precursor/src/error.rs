use std::fmt;
use std::io;

use crate_formats::cur;
use crate_pixmap_png as png;

use thiserror::Error;
use toml::de;

#[derive(Error, Debug)]
pub enum PrecursorError {
  #[error("{0}")]
  IoError(#[from] io::Error),

  #[error("{0}")]
  FmtError(#[from] fmt::Error),

  #[error("{0}")]
  CurDecodeError(#[from] cur::ReadError),

  #[error("{0}")]
  PngDecodeError(#[from] png::DecodeError),

  #[error("{0}")]
  PngEncodeError(#[from] png::EncodeError),

  #[error("{0}")]
  TomlError(#[from] de::Error),

  #[error("invalid asset type, expected a PNG")]
  InvalidAssetType,

  #[error(
    "unrecognised cursor format, consider passing the '--kind' argument and trying again"
  )]
  UnrecognisedCursorFormat,
}

pub type PrecursorResult<T = ()> = Result<T, PrecursorError>;
