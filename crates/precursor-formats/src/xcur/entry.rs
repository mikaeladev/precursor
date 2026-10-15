use std::io::{self, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::chunk::{XcursorChunk, XcursorCommentKind};
use super::error::{ReadError, ReadResult};

#[derive(Debug, PartialEq, Eq)]
pub enum XcursorTocEntry {
  Comment {
    kind: XcursorCommentKind,
    offset: u32,
  },
  Image {
    nominal: u32,
    offset: u32,
  },
}

impl XcursorTocEntry {
  pub(super) const ENTRY_SIZE: usize = 12;

  /// Reads a table of contents entry from `reader`.
  ///
  /// # Errors
  ///
  /// See the [`ReadError`] enum for details.
  pub fn read<R: Read>(reader: &mut R) -> ReadResult<Self> {
    let chunk_type = reader.read_u32::<LittleEndian>()?;
    let chunk_subtype = reader.read_u32::<LittleEndian>()?;
    let chunk_offset = reader.read_u32::<LittleEndian>()?;

    match chunk_type {
      XcursorChunk::COMMENT_TYPE => Ok(Self::Comment {
        kind: XcursorCommentKind::try_from(chunk_subtype)?,
        offset: chunk_offset,
      }),
      XcursorChunk::IMAGE_TYPE => Ok(Self::Image {
        nominal: chunk_subtype,
        offset: chunk_offset,
      }),
      _ => Err(ReadError::UnrecognisedChunkType(chunk_type)),
    }
  }

  /// Writes this entry to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    let chunk_type;
    let chunk_subtype;
    let chunk_offset;

    match self {
      Self::Comment { kind, offset } => {
        chunk_type = XcursorChunk::COMMENT_TYPE;
        chunk_subtype = kind as u32;
        chunk_offset = offset;
      }
      Self::Image { nominal, offset } => {
        chunk_type = XcursorChunk::IMAGE_TYPE;
        chunk_subtype = nominal;
        chunk_offset = offset;
      }
    }

    writer.write_u32::<LittleEndian>(chunk_type)?;
    writer.write_u32::<LittleEndian>(chunk_subtype)?;
    writer.write_u32::<LittleEndian>(chunk_offset)?;

    Ok(Self::ENTRY_SIZE)
  }

  /// Returns the chunk offset.
  pub fn offset(&self) -> u32 {
    *match self {
      Self::Comment { offset, .. } => offset,
      Self::Image { offset, .. } => offset,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const COMMENT_ENTRY: XcursorTocEntry = XcursorTocEntry::Comment {
    kind: XcursorCommentKind::Other,
    offset: 128,
  };

  const COMMENT_BYTES: &[u8] = &[
    0x01, 0x00, 0xFE, 0xFF, // chunk type
    0x03, 0x00, 0x00, 0x00, // comment kind
    0x80, 0x00, 0x00, 0x00, // data offset
  ];

  #[test]
  fn read_comment_entry() {
    let mut reader = COMMENT_BYTES;
    assert_eq!(XcursorTocEntry::read(&mut reader).unwrap(), COMMENT_ENTRY)
  }

  #[test]
  fn write_comment_entry() {
    let expected_size = COMMENT_BYTES.len();
    assert_eq!(XcursorTocEntry::ENTRY_SIZE, expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(COMMENT_ENTRY.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer, COMMENT_BYTES);
  }

  const IMAGE_ENTRY: XcursorTocEntry = XcursorTocEntry::Image {
    nominal: 12,
    offset: 256,
  };

  const IMAGE_BYTES: &[u8] = &[
    0x02, 0x00, 0xFD, 0xFF, // image type
    0x0C, 0x00, 0x00, 0x00, // nominal size
    0x00, 0x01, 0x00, 0x00, // data offset
  ];

  #[test]
  fn read_image_entry() {
    let mut reader = IMAGE_BYTES;
    assert_eq!(XcursorTocEntry::read(&mut reader).unwrap(), IMAGE_ENTRY)
  }

  #[test]
  fn write_image_entry() {
    let expected_size = IMAGE_BYTES.len();
    assert_eq!(XcursorTocEntry::ENTRY_SIZE, expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(IMAGE_ENTRY.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer, IMAGE_BYTES);
  }
}
