use crate_formats::DynamicPixmap;

#[derive(Debug, Clone)]
pub struct Cursor {
  pub frames: Vec<CursorFrame>,
  pub metadata: Option<CursorMetadata>,
}

impl Cursor {
  // TODO: description
  pub const fn is_animated(&self) -> bool {
    self.frames.len() != 1
  }
}

#[derive(Debug, Clone)]
pub struct CursorFrame {
  pub icons: Vec<CursorIcon>,
  pub duration: Option<CursorDuration>,
}

#[derive(Debug, Clone)]
pub struct CursorIcon {
  pub nominal: u32,
  pub hotspot: CursorHotspot,
  pub pixmap: DynamicPixmap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorHotspot {
  pub x: u32,
  pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorDuration(u32);

impl CursorDuration {
  const JIFFY: f32 = 16.666666;

  /// Creates a new `CursorDuration`.
  pub const fn new(ms: u32) -> Self {
    Self(ms)
  }

  /// Returns the duration in milliseconds.
  pub const fn milliseconds(self) -> u32 {
    self.0
  }

  /// Returns the duration in jiffies.
  pub const fn jiffies(self) -> u32 {
    (self.0 as f32 / Self::JIFFY) as u32
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorMetadata {
  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn duration_to_jiffy() {
    assert_eq!(CursorDuration::new(200).jiffies(), 12);
  }
}
