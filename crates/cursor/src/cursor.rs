use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use crate_formats::RasterImage;

use serde::Deserialize;

pub struct Cursor {
  pub frames: Vec<CursorFrame>,
  pub metadata: Option<CursorMetadata>,
}

impl Cursor {
  pub const fn is_animated(&self) -> bool {
    self.frames.len() != 1
  }
}

#[derive(Clone)]
pub struct CursorFrame {
  pub images: Vec<CursorImage>,
  pub duration: Option<CursorDuration>,
}

#[derive(Clone)]
pub struct CursorImage {
  pub nominal: u32,
  pub hotspot: CursorHotspot,
  pub raster: RasterImage,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct CursorDuration(u32);

impl CursorDuration {
  pub const ZERO: Self = Self(0);

  /// Returns the duration as a millisecond value.
  pub const fn milliseconds(self) -> u32 {
    self.0
  }

  /// Returns the duration as a jiffy value.
  pub const fn jiffies(self) -> u32 {
    self.0 / (1000 / 60)
  }

  /// Creates a new duration from a millisecond value.
  pub const fn from_milliseconds(value: u32) -> Self {
    Self(value)
  }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct CursorHotspot {
  pub x: u16,
  pub y: u16,
}

impl From<(u16, u16)> for CursorHotspot {
  fn from((x, y): (u16, u16)) -> Self {
    Self { x, y }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCursorHotspotError;

impl StdError for ParseCursorHotspotError {}

impl Display for ParseCursorHotspotError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    formatter.write_str("expected a `{u16},{u16}`")
  }
}

impl FromStr for CursorHotspot {
  type Err = ParseCursorHotspotError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (x, y) = s.split_once(',').ok_or(ParseCursorHotspotError)?;

    let x = x.parse::<u16>().map_err(|_| ParseCursorHotspotError)?;
    let y = y.parse::<u16>().map_err(|_| ParseCursorHotspotError)?;

    Ok(CursorHotspot { x, y })
  }
}

#[derive(Clone)]
pub struct CursorMetadata {
  // TODO
}
