use std::io::{self, Write};

use super::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IconDir(pub(crate) Vec<(IconDirEntry, Box<[u8]>)>);

impl IconDir {
  /// Writes an ICO file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  ///
  /// [`Write::write_all`]: Write::write_all
  pub(crate) fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    write_impl(writer, 1, self.0)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub(crate) fn exact_size(&self) -> usize {
    exact_size_impl(&self.0)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IconDirEntry {
  pub width: u8,
  pub height: u8,
  pub color_count: u8,
  pub color_planes: u16,
  pub bit_depth: u16,
}

impl IcoDirEntry for IconDirEntry {
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
    self.color_planes
  }
  fn depth_or_hoty(&self) -> u16 {
    self.bit_depth
  }
}
