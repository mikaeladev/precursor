use std::io::{Result as IoResult, Write};

use crate_images::RasterImage;

use byteorder::{LittleEndian, WriteBytesExt};
use num_enum::IntoPrimitive;

use crate::{Cursor, CursorDuration, CursorHotspot};

pub struct XCursor<'c>(&'c Cursor);

impl XCursor<'_> {
  /// Signature of the Xcursor file.
  pub const FILE_SIGNATURE: &'static [u8] = b"Xcur";
  /// Length of the file header in bytes.
  pub const FILE_HEADER: u32 = 16;
  /// Version of the Xcursor file.
  pub const FILE_VERSION: u32 = 0x10000;

  /// Writes an Xcursor file to the writer.
  pub fn write<W: Write>(self, mut writer: W) -> IoResult<()> {
    let Cursor {
      frames,
      metadata: _,
    } = self.0;

    let num_comment_chunks = 0;

    let num_image_chunks =
      frames.iter().fold(0, |acc, frame| acc + frame.images.len());

    let mut chunks = Vec::with_capacity(num_comment_chunks + num_image_chunks);

    for frame in frames {
      for image in frame.images.iter() {
        let chunk = Self::image_chunk(
          image.nominal,
          image.hotspot,
          frame.duration,
          &image.raster,
        )?;

        chunks.push((XCursorChunkType::Image, image.nominal, chunk));
      }
    }

    let num_chunks = chunks.len() as u32;

    writer.write_all(Self::FILE_SIGNATURE)?;
    writer.write_u32::<LittleEndian>(Self::FILE_HEADER)?;
    writer.write_u32::<LittleEndian>(Self::FILE_VERSION)?;
    writer.write_u32::<LittleEndian>(num_chunks)?;

    let mut buffer_offset = Self::FILE_HEADER + 12 * num_chunks;

    for (chunk_type, chunk_subtype_or_nominal, chunk) in chunks.iter() {
      writer.write_u32::<LittleEndian>((*chunk_type).into())?;
      writer.write_u32::<LittleEndian>(*chunk_subtype_or_nominal)?;
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
    nominal: u32,
    hotspot: CursorHotspot,
    duration: Option<CursorDuration>,
    raster: &RasterImage,
  ) -> IoResult<Vec<u8>> {
    let mut chunk = Vec::with_capacity(Self::IMAGE_HEADER as usize);

    let cursor_duration = duration.unwrap_or(CursorDuration::ZERO);

    chunk.write_u32::<LittleEndian>(Self::IMAGE_HEADER)?;
    chunk.write_u32::<LittleEndian>(XCursorChunkType::Image.into())?;
    chunk.write_u32::<LittleEndian>(nominal)?;
    chunk.write_u32::<LittleEndian>(Self::IMAGE_VERSION)?;
    chunk.write_u32::<LittleEndian>(raster.width())?;
    chunk.write_u32::<LittleEndian>(raster.height())?;
    chunk.write_u32::<LittleEndian>(hotspot.x as u32)?;
    chunk.write_u32::<LittleEndian>(hotspot.y as u32)?;
    chunk.write_u32::<LittleEndian>(cursor_duration.milliseconds())?;

    chunk.write_all(&raster.to_bgra())?;

    Ok(chunk)
  }
}

impl<'c> From<&'c Cursor> for XCursor<'c> {
  fn from(value: &'c Cursor) -> Self {
    Self(value)
  }
}

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u32)]
enum XCursorChunkType {
  Comment = 0xfffe0001,
  Image = 0xfffd0002,
}

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u32)]
enum XCursorCommentType {
  Copyright = 1,
  License = 2,
  Other = 3,
}
