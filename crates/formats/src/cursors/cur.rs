use std::io::{self, Write};

use crate_point::Point;

use crate::containers::ico::{CursorDir, CursorDirEntry};

pub struct CurFile(CursorDir);

impl CurFile {
  /// Constructs a new `CurFile`.
  pub fn new(icons: impl IntoIterator<Item = CurIcon>) -> Self {
    let images = icons.into_iter().map(|i| {
      let entry = CursorDirEntry {
        width: if i.width > 255 { 0 } else { i.width as u8 },
        height: if i.height > 255 { 0 } else { i.height as u8 },
        color_count: 0,
        hotspot: i.hotspot,
      };

      (entry, i.buffer)
    });

    Self(CursorDir(images.collect()))
  }

  /// Writes a CUR file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  ///
  /// [`Write::write_all`]: Write::write_all
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    self.0.write(writer)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    self.0.exact_size()
  }
}

pub struct CurIcon {
  width: u16,
  height: u16,
  hotspot: Point<u16>,
  buffer: Box<[u8]>,
}

impl CurIcon {
  /// Constructs a new `CurIcon`.
  ///
  /// # Panics
  ///
  /// Panics if `hotspot` is out of bounds.
  pub const fn new(
    width: u16,
    height: u16,
    hotspot: Point<u16>,
    buffer: Box<[u8]>,
  ) -> Self {
    assert!(hotspot.x <= width, "hotspot.x should be ≤ width");
    assert!(hotspot.y <= height, "hotspot.y should be ≤ height");

    Self {
      width,
      height,
      hotspot,
      buffer,
    }
  }
}
