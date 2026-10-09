use precursor_formats::cur::{CurFile, CurImage};
use precursor_formats::png::{PngFile, WriteResult};
use precursor_point::Point;

use crate::cursor::CursorIcon;

#[derive(Debug, Clone)]
pub struct CursorFrame {
  pub icons: Vec<CursorIcon>,
  pub duration: Option<CursorDuration>,
}

impl CursorFrame {
  /// Constructs a new `CursorFrame`.
  ///
  /// # Panics
  ///
  /// Panics if `icons` is empty.
  pub const fn new(
    icons: Vec<CursorIcon>,
    duration: Option<CursorDuration>,
  ) -> Self {
    assert!(!icons.is_empty(), "icons should not be empty");

    Self { icons, duration }
  }

  /// Constructs a new [`CurFile`].
  ///
  /// # Errors
  ///
  /// Returns the same errors as [`precursor_pixmap_png::encode`].
  ///
  /// # Panics
  ///
  /// Panics if any icon [hotspot] is out of bounds.
  ///
  /// [hotspot]: Point
  pub fn to_cur(&self) -> WriteResult<CurFile> {
    let mut icons = Vec::with_capacity(self.icons.len());

    for icon in &self.icons {
      let (width, height) = icon.pixmap.dimensions();

      let hotspot = Point::from((icon.hotspot.x as u16, icon.hotspot.y as u16));

      let mut buffer = Vec::with_capacity(width as usize * height as usize);
      PngFile::new(icon.pixmap.clone()).write(&mut buffer)?;

      icons.push(CurImage::new(
        width as u16,
        height as u16,
        hotspot,
        buffer.into_boxed_slice(),
      ));
    }

    Ok(CurFile::new(icons))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorDuration(u32);

impl CursorDuration {
  const JIFFY: f32 = 16.666666;

  /// Constructs a new `CursorDuration`.
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn duration_to_jiffy() {
    assert_eq!(CursorDuration::new(200).jiffies(), 12);
  }
}
