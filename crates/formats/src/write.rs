use std::io::Write;

use crate::raster::RasterError;

pub trait WriteTo {
  /// Writes the formatted data to `writer`.
  fn write_to<W: Write>(self, writer: W) -> WriteResult;
}

pub type WriteResult<T = ()> = Result<T, RasterError>;
