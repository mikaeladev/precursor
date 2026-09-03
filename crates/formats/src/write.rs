use std::io::Write;

use crate::rasters::RasterResult;

pub type WriteResult = RasterResult;

pub trait WriteTo {
  /// Writes the formatted data to `writer`.
  fn write_to<W: Write>(self, writer: W) -> WriteResult;
}
