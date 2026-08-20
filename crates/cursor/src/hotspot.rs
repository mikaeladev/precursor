use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct CursorHotspot {
  pub x: u32,
  pub y: u32,
}

impl CursorHotspot {
  /// Creates a new `CursorHotspot`.
  pub const fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCursorHotspotError;

impl StdError for ParseCursorHotspotError {}

impl Display for ParseCursorHotspotError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    formatter.write_str("expected a hotspot string")
  }
}

impl FromStr for CursorHotspot {
  type Err = ParseCursorHotspotError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (x, y) = s.split_once(',').ok_or(ParseCursorHotspotError)?;

    let x = x.parse::<u32>().map_err(|_| ParseCursorHotspotError)?;
    let y = y.parse::<u32>().map_err(|_| ParseCursorHotspotError)?;

    Ok(CursorHotspot { x, y })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_str() {
    assert_eq!(
      CursorHotspot::from_str("4,4").unwrap(),
      CursorHotspot::new(4, 4)
    );
  }

  #[test]
  fn test_from_str_err() {
    assert_eq!(
      CursorHotspot::from_str("test"),
      Err(ParseCursorHotspotError)
    );
  }
}
