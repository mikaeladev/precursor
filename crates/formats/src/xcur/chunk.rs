use std::io::{self, Read, Write};

use crate_point::Point;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::error::{ReadError, ReadResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XcursorChunk {
  Comment {
    kind: XcursorCommentKind,
    value: Box<str>,
  },
  Image {
    nominal: u32,
    width: u32,
    height: u32,
    hotspot: Point<u32>,
    duration: u32,
    pixels: Box<[u8]>,
  },
}

impl XcursorChunk {
  pub(super) const VERSION: u32 = 1;

  pub(super) const COMMENT_HEADER_SIZE: usize = 20;
  pub(super) const COMMENT_TYPE: u32 = 0xfffe0001;

  pub(super) const IMAGE_HEADER_SIZE: usize = 36;
  pub(super) const IMAGE_TYPE: u32 = 0xfffd0002;

  /// Reads a chunk from `reader`.
  ///
  /// # Errors
  ///
  /// See the [`ReadError`] enum for details.
  pub fn read<R: Read>(reader: &mut R) -> ReadResult<Self> {
    let header_size = reader.read_u32::<LittleEndian>()? as usize;
    let chunk_type = reader.read_u32::<LittleEndian>()?;
    let kind_or_nominal = reader.read_u32::<LittleEndian>()?;
    let chunk_version = reader.read_u32::<LittleEndian>()?;

    if chunk_version != Self::VERSION {
      return Err(ReadError::UnsupportedChunkVersion(chunk_version));
    }

    match chunk_type {
      Self::COMMENT_TYPE if header_size != Self::COMMENT_HEADER_SIZE => {
        Err(ReadError::InvalidCommentHeaderSize(header_size))
      }
      Self::COMMENT_TYPE => {
        let kind = XcursorCommentKind::try_from(kind_or_nominal)?;

        let data_len = reader.read_u32::<LittleEndian>()? as usize;

        let mut buffer = vec![0; data_len];
        reader.read_exact(&mut buffer)?;

        let value = String::from_utf8(buffer)?.into_boxed_str();

        Ok(Self::Comment { kind, value })
      }
      Self::IMAGE_TYPE if header_size != Self::IMAGE_HEADER_SIZE => {
        Err(ReadError::InvalidImageHeaderSize(header_size))
      }
      Self::IMAGE_TYPE => {
        let nominal = kind_or_nominal;

        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;

        let hotspot = Point {
          x: reader.read_u32::<LittleEndian>()?,
          y: reader.read_u32::<LittleEndian>()?,
        };

        let duration = reader.read_u32::<LittleEndian>()?;

        let mut buffer = vec![0; width as usize * height as usize * 4];
        reader.read_exact(&mut buffer)?;

        let pixels = buffer.into_boxed_slice();

        Ok(Self::Image {
          nominal,
          width,
          height,
          hotspot,
          duration,
          pixels,
        })
      }
      _ => Err(ReadError::UnrecognisedChunkType(chunk_type)),
    }
  }

  /// Writes this chunk to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    match self {
      Self::Comment { kind, value } => {
        writer.write_u32::<LittleEndian>(Self::COMMENT_HEADER_SIZE as u32)?;
        writer.write_u32::<LittleEndian>(Self::COMMENT_TYPE)?;
        writer.write_u32::<LittleEndian>(kind as u32)?;
        writer.write_u32::<LittleEndian>(Self::VERSION)?;
        writer.write_u32::<LittleEndian>(value.len() as u32)?;

        writer.write_all(value.as_bytes())?;

        Ok(Self::COMMENT_HEADER_SIZE as usize + value.len())
      }

      Self::Image {
        nominal,
        width,
        height,
        hotspot,
        duration,
        pixels,
      } => {
        writer.write_u32::<LittleEndian>(Self::IMAGE_HEADER_SIZE as u32)?;
        writer.write_u32::<LittleEndian>(Self::IMAGE_TYPE)?;
        writer.write_u32::<LittleEndian>(nominal)?;
        writer.write_u32::<LittleEndian>(Self::VERSION)?;
        writer.write_u32::<LittleEndian>(width)?;
        writer.write_u32::<LittleEndian>(height)?;
        writer.write_u32::<LittleEndian>(hotspot.x)?;
        writer.write_u32::<LittleEndian>(hotspot.y)?;
        writer.write_u32::<LittleEndian>(duration)?;

        writer.write_all(&pixels)?;

        Ok(Self::IMAGE_HEADER_SIZE + pixels.len())
      }
    }
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub const fn exact_size(&self) -> usize {
    match self {
      Self::Comment { value, .. } => Self::COMMENT_HEADER_SIZE + value.len(),
      Self::Image { pixels, .. } => Self::IMAGE_HEADER_SIZE + pixels.len(),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum XcursorCommentKind {
  Copyright = 1,
  License = 2,
  Other = 3,
}

impl TryFrom<u32> for XcursorCommentKind {
  type Error = ReadError;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(XcursorCommentKind::Copyright),
      2 => Ok(XcursorCommentKind::License),
      3 => Ok(XcursorCommentKind::Other),
      _ => Err(ReadError::UnrecognisedCommentType(value)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const COMMENT_BYTES: &[u8] = &[
    //-- chunk header --//
    0x14, 0x00, 0x00, 0x00, // header size
    0x01, 0x00, 0xFE, 0xFF, // chunk type
    0x03, 0x00, 0x00, 0x00, // comment kind
    0x01, 0x00, 0x00, 0x00, // header version
    0x0B, 0x00, 0x00, 0x00, // data length
    //--  chunk data  --//
    0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "hello "
    0x77, 0x6F, 0x72, 0x6C, 0x64, // "world"
  ];

  fn comment_chunk() -> XcursorChunk {
    XcursorChunk::Comment {
      kind: XcursorCommentKind::Other,
      value: Box::from("hello world"),
    }
  }

  #[test]
  fn read_comment_chunk() {
    let mut reader = COMMENT_BYTES;

    let value = XcursorChunk::read(&mut reader).unwrap();
    let expected = comment_chunk();

    assert_eq!(value, expected)
  }

  #[test]
  fn write_comment_chunk() {
    let chunk = comment_chunk();

    let expected_size = COMMENT_BYTES.len();
    assert_eq!(chunk.exact_size(), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(chunk.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer, COMMENT_BYTES);
  }

  #[rustfmt::skip]
  const IMAGE_BYTES: &[u8] = &[
    //-- chunk header --//
    0x24, 0x00, 0x00, 0x00, // header size
    0x02, 0x00, 0xFD, 0xFF, // chunk type
    0x03, 0x00, 0x00, 0x00, // nominal size
    0x01, 0x00, 0x00, 0x00, // header version
    0x03, 0x00, 0x00, 0x00, // image width
    0x03, 0x00, 0x00, 0x00, // image height
    0x00, 0x00, 0x00, 0x00, // hotspot x
    0x00, 0x00, 0x00, 0x00, // hotspot y
    0x00, 0x00, 0x00, 0x00, // duration (N/A)
    //--  chunk data  --//
    0x00, 0x00, 0x00, 0xFF, // 0,0 - black   this is a 3x3 rgba pixmap depicting
    0x00, 0x00, 0x00, 0xFF, // 1,0 - black   a very basic cursor. it has a white
    0x00, 0x00, 0x00, 0xFF, // 2,0 - black   dot in the middle and a transparent
    0x00, 0x00, 0x00, 0xFF, // 0,1 - black   bottom right corner.
    0xFF, 0xFF, 0xFF, 0xFF, // 1,1 - white
    0x00, 0x00, 0x00, 0xFF, // 2,1 - black
    0x00, 0x00, 0x00, 0xFF, // 0,2 - black
    0x00, 0x00, 0x00, 0xFF, // 1,2 - black
    0x00, 0x00, 0x00, 0x00, // 2,2 - trans
  ];

  fn image_chunk() -> XcursorChunk {
    const PIXELS: &[u8] = &[
      0x00, 0x00, 0x00, 0xFF, // 0,0 - black
      0x00, 0x00, 0x00, 0xFF, // 1,0 - black
      0x00, 0x00, 0x00, 0xFF, // 2,0 - black
      0x00, 0x00, 0x00, 0xFF, // 0,1 - black
      0xFF, 0xFF, 0xFF, 0xFF, // 1,1 - white
      0x00, 0x00, 0x00, 0xFF, // 2,1 - black
      0x00, 0x00, 0x00, 0xFF, // 0,2 - black
      0x00, 0x00, 0x00, 0xFF, // 1,2 - black
      0x00, 0x00, 0x00, 0x00, // 2,2 - trans
    ];

    XcursorChunk::Image {
      nominal: 3,
      width: 3,
      height: 3,
      hotspot: Point { x: 0, y: 0 },
      duration: 0,
      pixels: Box::from(PIXELS),
    }
  }

  #[test]
  fn read_image_chunk() {
    let mut reader = IMAGE_BYTES;

    let value = XcursorChunk::read(&mut reader).unwrap();
    let expected = image_chunk();

    assert_eq!(value, expected)
  }

  #[test]
  fn write_image_chunk() {
    let chunk = image_chunk();

    let expected_size = IMAGE_BYTES.len();
    assert_eq!(chunk.exact_size(), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(chunk.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer, IMAGE_BYTES);
  }
}
