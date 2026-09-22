use std::io::{self, BufRead, Seek, Write};

use crate_point::Point;

use super::common::{self, DirEntry};
use super::error::ReadResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CursorDir(pub(crate) Vec<(CursorDirEntry, Box<[u8]>)>);

impl CursorDir {
  /// Reads a CUR file from `reader`, returning the constructed `CursorDir`.
  ///
  /// # Errors
  ///
  /// TODO
  pub(crate) fn read<R: BufRead + Seek>(reader: &mut R) -> ReadResult<Self> {
    Ok(Self(common::read_impl(reader)?))
  }

  /// Writes a CUR file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  ///
  /// [`Write::write_all`]: Write::write_all
  pub(crate) fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    common::write_impl(writer, self.0)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub(crate) fn exact_size(&self) -> usize {
    common::exact_size_impl(&self.0)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CursorDirEntry {
  pub width: u8,
  pub height: u8,
  pub color_count: u8,
  pub hotspot: Point<u16>,
}

impl DirEntry for CursorDirEntry {
  const RESOURCE_TYPE: u16 = 2;

  fn new(
    width: u8,
    height: u8,
    color_count: u8,
    planes_or_hotx: u16,
    depth_or_hoty: u16,
  ) -> Self {
    Self {
      width,
      height,
      color_count,
      hotspot: Point {
        x: planes_or_hotx,
        y: depth_or_hoty,
      },
    }
  }

  fn width(&self) -> u8 {
    self.width
  }
  fn height(&self) -> u8 {
    self.height
  }
  fn color_count(&self) -> u8 {
    self.color_count
  }
  fn planes_or_hotx(&self) -> u16 {
    self.hotspot.x
  }
  fn depth_or_hoty(&self) -> u16 {
    self.hotspot.y
  }
}
