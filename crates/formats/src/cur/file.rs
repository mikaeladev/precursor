use std::io::{self, Read, Seek, Write};

use crate_point::Point;

use crate::ico::macros::wrap_u8;
use crate::ico::traits::IcoDir;

use super::dir::{CursorDir, CursorDirEntry};

pub type ReadError = crate::ico::ReadError;
pub type ReadResult<T> = crate::ico::ReadResult<T>;

pub struct CurFile(CursorDir);

impl CurFile {
  /// Constructs a new `CurFile`.
  pub fn new(images: impl IntoIterator<Item = CurImage>) -> Self {
    let images = images.into_iter().map(From::from).collect();
    Self(CursorDir(images))
  }

  /// Reads a CUR file from `reader`, returning the constructed `CurFile`.
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
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    self.0.write(writer)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    self.0.exact_size()
  }

  /// Converts `self` into a `Vec<`[`CurImage`]`>`.
  ///
  /// Note that the `width` and `height` fields of a given icon may be zero.
  /// This is because the ICO format cannot hold width/height values over `255`,
  /// and as such the true values should be gained by inspecting the PNG/BMP
  /// buffer.
  pub fn into_icons(self) -> Vec<CurImage> {
    let CurFile(CursorDir(entries)) = self;

    entries
      .into_iter()
      .map(|(entry, buffer)| CurImage {
        width: entry.width as u16,
        height: entry.height as u16,
        hotspot: entry.hotspot,
        buffer,
      })
      .collect()
  }
}

pub struct CurImage {
  pub width: u16,
  pub height: u16,
  pub hotspot: Point<u16>,
  pub buffer: Box<[u8]>,
}

impl CurImage {
  /// Constructs a new `CurImage`.
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

impl From<CurImage> for (CursorDirEntry, Box<[u8]>) {
  fn from(image: CurImage) -> Self {
    let entry = CursorDirEntry {
      width: wrap_u8!(image.width),
      height: wrap_u8!(image.height),
      hotspot: image.hotspot,
    };
    (entry, image.buffer)
  }
}
