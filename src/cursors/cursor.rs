use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::Deserialize;

use crate::images::RgbaImage;

pub struct Cursor<'i> {
  pub frames: Vec<CursorFrame<'i>>,
  pub metadata: Option<CursorMetadata>,
}

#[derive(Clone)]
pub struct CursorFrame<'i> {
  pub images: &'i [CursorImage<'i>],
  pub duration: Option<CursorDuration>,
}

pub struct CursorImage<'i> {
  pub nominal: u32,
  pub hotspot: CursorHotspot,
  pub rgba: &'i RgbaImage,
}

#[derive(Clone, Copy, Deserialize)]
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
}

#[derive(Clone, Copy, Deserialize)]
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
