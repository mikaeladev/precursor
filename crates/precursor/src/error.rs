use std::fmt;
use std::io;

use precursor_error::Error;
use precursor_formats::ani;
use precursor_formats::cur;
use precursor_formats::png;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrecursorError {
  #[error("{0}")]
  Error(#[from] Error),

  #[error("{0}")]
  IoError(#[from] io::Error),

  #[error("{0}")]
  FmtError(#[from] fmt::Error),

  #[error("{0}")]
  AniReadError(#[from] ani::ReadError),

  #[error("{0}")]
  CurReadError(#[from] cur::ReadError),

  #[error("{0}")]
  PngReadError(#[from] png::ReadError),

  #[error("{0}")]
  PngWriteError(#[from] png::WriteError),

  #[error("invalid asset type, expected a PNG")]
  InvalidAssetType,

  #[error(
    "unrecognised cursor format, consider passing the '--kind' argument and trying again"
  )]
  UnrecognisedCursorFormat,
}

pub type PrecursorResult<T = ()> = Result<T, PrecursorError>;
