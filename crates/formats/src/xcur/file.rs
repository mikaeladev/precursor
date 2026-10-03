use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use super::chunk::XcursorChunk;
use super::entry::XcursorTocEntry;

pub const XCUR_MAGIC: &[u8] = b"Xcur";

#[derive(Debug)]
pub struct XcursorFile {
  chunks: Vec<XcursorChunk>,
}

impl XcursorFile {
  const FILE_HEADER_SIZE: usize = 16;
  const FILE_VERSION: u32 = 0x10000;

  /// Constructs a new `XcursorFile`.
  ///
  /// # Panics
  ///
  /// Panics in the following situations:
  ///
  /// - `chunks` is empty;
  /// - `chunks` exceeds `u32::MAX` in length;
  /// - Any individal chunk exceeds `u32::MAX` in size.
  pub fn new(chunks: Vec<XcursorChunk>) -> Self {
    const MAX: usize = u32::MAX as usize;

    assert!(!chunks.is_empty(), "chunks should not be empty");

    assert!(
      chunks.len() <= MAX,
      "chunks length should not exceed u32::MAX"
    );

    for chunk in &chunks {
      assert!(
        chunk.exact_size() <= MAX,
        "chunk size should not exceed u32::MAX"
      )
    }

    Self { chunks }
  }

  /// Writes an Xcursor file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    let chunks_len = self.chunks.len();

    writer.write_all(XCUR_MAGIC)?;
    writer.write_u32::<LittleEndian>(Self::FILE_HEADER_SIZE as u32)?;
    writer.write_u32::<LittleEndian>(Self::FILE_VERSION)?;
    writer.write_u32::<LittleEndian>(chunks_len as u32)?;

    let mut data_pos = (Self::FILE_HEADER_SIZE
      + XcursorTocEntry::ENTRY_SIZE * chunks_len) as u32;

    for chunk in &self.chunks {
      let entry = match chunk {
        XcursorChunk::Comment { kind, .. } => XcursorTocEntry::Comment {
          kind: *kind,
          offset: data_pos,
        },
        XcursorChunk::Image { nominal, .. } => XcursorTocEntry::Image {
          nominal: *nominal,
          offset: data_pos,
        },
      };

      entry.write(writer)?;
      data_pos += chunk.exact_size() as u32;
    }

    for chunk in self.chunks {
      chunk.write(writer)?;
    }

    Ok(data_pos as usize)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    let f = |acc: usize, chunk: &XcursorChunk| {
      acc + XcursorTocEntry::ENTRY_SIZE + chunk.exact_size()
    };

    self.chunks.iter().fold(Self::FILE_HEADER_SIZE, f)
  }
}

// TODO: tests
