use std::io::{Result as IoResult, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use num_enum::IntoPrimitive;

use crate::cursors::{CursorDuration, CursorImage};

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u32)]
pub enum XCursorChunkType {
  Comment = 0xfffe0001,
  Image = 0xfffd0002,
}

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u32)]
pub enum XCursorCommentType {
  Copyright = 1,
  License = 2,
  Other = 3,
}

pub struct XCursor {
  comments: Vec<(XCursorCommentType, Box<str>)>,
  images: Vec<CursorImage>,
}

impl XCursor {
  /// Creates a new x cursor.
  ///
  /// # Panics
  /// Panics if the combined number of comments and images exceeds `u32::MAX`
  /// bytes.
  pub const fn new(
    comments: Vec<(XCursorCommentType, Box<str>)>,
    images: Vec<CursorImage>,
  ) -> Self {
    if images.len() > u32::MAX as usize {
      panic!(
        "combined number of comments and images should not exceed u32::MAX"
      )
    }

    Self { comments, images }
  }

  /// Signature of the Xcursor file.
  pub const FILE_SIGNATURE: &[u8] = b"Xcur";
  /// Length of the file header in bytes.
  pub const FILE_HEADER: u32 = 16;
  /// Version of the Xcursor file.
  pub const FILE_VERSION: u32 = 0x10000;

  /// Writes an Xcursor file to the writer.
  pub fn write<W: Write>(self, mut writer: W) -> IoResult<()> {
    let num_comments = self.comments.len();
    let num_images = self.images.len();

    let mut chunks = Vec::with_capacity(num_comments + num_images);

    for (subtype, value) in self.comments {
      chunks.push((
        XCursorChunkType::Comment,
        subtype.into(),
        Self::comment_chunk(subtype, value)?,
      ));
    }

    for image in self.images {
      chunks.push((
        XCursorChunkType::Image,
        image.size() as u32,
        Self::image_chunk(image, None)?,
      ));
    }

    let num_chunks = chunks.len() as u32;

    writer.write_all(Self::FILE_SIGNATURE)?;
    writer.write_u32::<LittleEndian>(Self::FILE_HEADER)?;
    writer.write_u32::<LittleEndian>(Self::FILE_VERSION)?;
    writer.write_u32::<LittleEndian>(num_chunks)?;

    let mut buffer_offset = Self::FILE_HEADER + 12 * num_chunks;

    for (chunk_type, chunk_subtype_or_size, chunk) in chunks.iter() {
      writer.write_u32::<LittleEndian>((*chunk_type).into())?;
      writer.write_u32::<LittleEndian>(*chunk_subtype_or_size)?;
      writer.write_u32::<LittleEndian>(buffer_offset)?;

      buffer_offset += chunk.len() as u32;
    }

    for (_, _, chunk) in chunks {
      writer.write_all(&chunk)?;
    }

    Ok(())
  }

  /// Length of the comment chunk header in bytes.
  pub const COMMENT_HEADER: u32 = 20;
  /// Version of the comment chunk.
  pub const COMMENT_VERSION: u32 = 1;

  /// Creates a comment chunk.
  fn comment_chunk(
    subtype: XCursorCommentType,
    value: Box<str>,
  ) -> IoResult<Vec<u8>> {
    let value_len = value.len();

    let mut chunk =
      Vec::with_capacity(Self::COMMENT_HEADER as usize + value_len);

    chunk.write_u32::<LittleEndian>(Self::COMMENT_HEADER)?;
    chunk.write_u32::<LittleEndian>(XCursorChunkType::Comment.into())?;
    chunk.write_u32::<LittleEndian>(subtype.into())?;
    chunk.write_u32::<LittleEndian>(Self::COMMENT_VERSION)?;
    chunk.write_u32::<LittleEndian>(value_len as u32)?;

    chunk.write_all(&value.into_boxed_bytes())?;

    Ok(chunk)
  }

  /// Length of the image chunk header in bytes.
  const IMAGE_HEADER: u32 = 36;
  /// Version of the image chunk.
  const IMAGE_VERSION: u32 = 1;

  /// Creates an image chunk.
  fn image_chunk(
    image: CursorImage,
    duration: impl Into<Option<CursorDuration>>,
  ) -> IoResult<Vec<u8>> {
    let mut chunk = Vec::with_capacity(Self::IMAGE_HEADER as usize);

    let image_size = image.size() as u32;
    let image_hotspot = image.hotspot();
    let image_duration = duration.into().unwrap_or(CursorDuration::ZERO);

    chunk.write_u32::<LittleEndian>(Self::IMAGE_HEADER)?;
    chunk.write_u32::<LittleEndian>(XCursorChunkType::Image.into())?;
    chunk.write_u32::<LittleEndian>(image_size)?; // nominal
    chunk.write_u32::<LittleEndian>(Self::IMAGE_VERSION)?;
    chunk.write_u32::<LittleEndian>(image_size)?; // width
    chunk.write_u32::<LittleEndian>(image_size)?; // height
    chunk.write_u32::<LittleEndian>(image_hotspot.0 as u32)?;
    chunk.write_u32::<LittleEndian>(image_hotspot.1 as u32)?;
    chunk.write_u32::<LittleEndian>(image_duration.milliseconds())?;

    chunk.write_all(&image.into_bgra())?;

    Ok(chunk)
  }
}
