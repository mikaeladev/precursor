use std::io::{self, Seek, Write};

pub trait CursorFile {
  /// Returns the size of the file in bytes.
  fn size(&self) -> usize;

  /// Attempts to write the file to `writer`.
  fn write<W: Write + Seek>(self, writer: &mut W) -> io::Result<()>;
}
