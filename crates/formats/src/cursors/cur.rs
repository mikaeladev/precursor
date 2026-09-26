use std::io::{self, Read, Seek, Write};

use crate_point::Point;

use crate::containers::ico::{CursorDir, CursorDirEntry};

pub type ReadError = crate::containers::ico::ReadError;
pub type ReadResult<T> = crate::containers::ico::ReadResult<T>;

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

  /// Reads a CUR file from reader, returning the constructed `CurFile`.
  ///
  /// # Errors
  ///
  /// TODO
  pub fn read<R: Read + Seek>(reader: &mut R) -> ReadResult<Self> {
    Ok(Self(CursorDir::read(reader)?))
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

  /// Converts `self` into a `Vec<`[`CurIcon`]`>`.
  ///
  /// Note that the `width` and `height` fields of a given icon may be zero.
  /// This is because the ICO format cannot hold width/height values over `255`,
  /// and as such the true values should be gained by inspecting the PNG/BMP
  /// buffer.
  pub fn into_icons(self) -> Vec<CurIcon> {
    let CurFile(CursorDir(entries)) = self;

    entries
      .into_iter()
      .map(|(entry, buffer)| CurIcon {
        width: entry.width as u16,
        height: entry.height as u16,
        hotspot: entry.hotspot,
        buffer,
      })
      .collect()
  }
}

pub struct CurIcon {
  pub width: u16,
  pub height: u16,
  pub hotspot: Point<u16>,
  pub buffer: Box<[u8]>,
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
