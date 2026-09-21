use std::io::{self, Write};

use crate_point::Point;

use byteorder::{LittleEndian, WriteBytesExt};

const FILE_HEADER_SIZE: u32 = 16;
const FILE_VERSION: u32 = 0x10000;

const TOC_ENTRY_SIZE: u32 = 12;

const CHUNK_VERSION: u32 = 1;

const COMMENT_HEADER_SIZE: u32 = 20;
const COMMENT_CHUNK_TYPE: u32 = 0xfffe0001;

const IMAGE_HEADER_SIZE: u32 = 36;
const IMAGE_CHUNK_TYPE: u32 = 0xfffd0002;

#[derive(Debug)]
pub struct XcursorFile {
  chunks: Vec<XcursorChunk>,
}

impl XcursorFile {
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
    assert!(!chunks.is_empty(), "chunks should not be empty");

    assert!(
      chunks.len() <= u32::MAX as usize,
      "chunks length should not exceed u32::MAX"
    );

    for chunk in &chunks {
      let chunk_size = match chunk {
        XcursorChunk::Comment { value, .. } => {
          COMMENT_HEADER_SIZE as usize + value.len()
        }
        XcursorChunk::Image { pixels, .. } => {
          IMAGE_HEADER_SIZE as usize + pixels.len()
        }
      };

      assert!(
        chunk_size <= u32::MAX as usize,
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
  ///
  /// [`Write::write_all`]: Write::write_all
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    let chunks_len = self.chunks.len();

    writer.write_all(b"Xcur")?;
    writer.write_u32::<LittleEndian>(FILE_HEADER_SIZE)?;
    writer.write_u32::<LittleEndian>(FILE_VERSION)?;
    writer.write_u32::<LittleEndian>(chunks_len as u32)?;

    let mut data_pos = FILE_HEADER_SIZE + TOC_ENTRY_SIZE * chunks_len as u32;

    for chunk in &self.chunks {
      let chunk_type;
      let chunk_kind;
      let chunk_size;

      match chunk {
        XcursorChunk::Comment { kind, value } => {
          chunk_type = COMMENT_CHUNK_TYPE;
          chunk_kind = *kind as u32;
          chunk_size = COMMENT_HEADER_SIZE as usize + value.len();
        }

        XcursorChunk::Image {
          nominal, pixels, ..
        } => {
          chunk_type = IMAGE_CHUNK_TYPE;
          chunk_kind = *nominal;
          chunk_size = IMAGE_HEADER_SIZE as usize + pixels.len();
        }
      }

      writer.write_u32::<LittleEndian>(chunk_type)?;
      writer.write_u32::<LittleEndian>(chunk_kind)?;
      writer.write_u32::<LittleEndian>(data_pos)?;

      data_pos += chunk_size as u32;
    }

    for chunk in self.chunks {
      match chunk {
        XcursorChunk::Comment { kind, value } => {
          writer.write_u32::<LittleEndian>(COMMENT_HEADER_SIZE)?;
          writer.write_u32::<LittleEndian>(COMMENT_CHUNK_TYPE)?;
          writer.write_u32::<LittleEndian>(kind as u32)?;
          writer.write_u32::<LittleEndian>(CHUNK_VERSION)?;
          writer.write_u32::<LittleEndian>(value.len() as u32)?;

          writer.write_all(value.as_bytes())?;
        }

        XcursorChunk::Image {
          nominal,
          width,
          height,
          hotspot,
          duration,
          pixels,
        } => {
          writer.write_u32::<LittleEndian>(IMAGE_HEADER_SIZE)?;
          writer.write_u32::<LittleEndian>(IMAGE_CHUNK_TYPE)?;
          writer.write_u32::<LittleEndian>(nominal)?;
          writer.write_u32::<LittleEndian>(CHUNK_VERSION)?;
          writer.write_u32::<LittleEndian>(width)?;
          writer.write_u32::<LittleEndian>(height)?;
          writer.write_u32::<LittleEndian>(hotspot.x)?;
          writer.write_u32::<LittleEndian>(hotspot.y)?;
          writer.write_u32::<LittleEndian>(duration)?;

          writer.write_all(&pixels)?;
        }
      }
    }

    Ok(data_pos as usize)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    let f = |acc: usize, chunk: &XcursorChunk| {
      let chunk_size = match chunk {
        XcursorChunk::Comment { value, .. } => {
          COMMENT_HEADER_SIZE as usize + value.len()
        }
        XcursorChunk::Image { pixels, .. } => {
          IMAGE_HEADER_SIZE as usize + pixels.len()
        }
      };

      acc + TOC_ENTRY_SIZE as usize + chunk_size
    };

    self.chunks.iter().fold(FILE_HEADER_SIZE as usize, f)
  }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum XcursorCommentKind {
  Copyright = 1,
  License = 2,
  Other = 3,
}
