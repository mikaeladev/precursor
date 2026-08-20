use std::io::{Result as IoResult, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::WriteTo;

pub struct XcursorFile<'c> {
  chunks: Vec<XcursorChunk<'c>>,
}

impl<'c> XcursorFile<'c> {
  const HEADER_SIZE: usize = 16;
  const HEADER_VERSION: u32 = 0x10000;

  /// Creates a new `XcursorFile`.
  ///
  /// # Panics
  ///
  /// Panics if `chunks` length exceeds `u32::MAX`.
  pub fn new(chunks: Vec<impl Into<XcursorChunk<'c>>>) -> Self {
    assert!(
      chunks.len() <= u32::MAX as usize,
      "chunks length should be ≤ u32::MAX"
    );

    let chunks: Vec<_> = chunks.into_iter().map(|c| c.into()).collect();

    Self { chunks }
  }

  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    let slice = self.chunks.as_slice();
    let len = slice.len();

    let mut index = 0;
    let mut acc = Self::HEADER_SIZE + XcursorTocEntry::SIZE * len;

    loop {
      index += 1;

      if index > len {
        break acc;
      }

      acc += slice[index].size();
    }
  }
}

impl WriteTo for XcursorFile<'_> {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    let num_chunks = self.chunks.len();

    writer.write_all(b"Xcur")?;
    writer.write_u32::<LittleEndian>(Self::HEADER_SIZE as u32)?;
    writer.write_u32::<LittleEndian>(Self::HEADER_VERSION)?;
    writer.write_u32::<LittleEndian>(num_chunks as u32)?;

    let mut data_offset =
      Self::HEADER_SIZE + XcursorTocEntry::SIZE * num_chunks;

    for chunk in &self.chunks {
      let entry: XcursorTocEntry = chunk.into();

      writer.write_u32::<LittleEndian>(entry.r#type)?;
      writer.write_u32::<LittleEndian>(entry.subtype)?;
      writer.write_u32::<LittleEndian>(data_offset as u32)?;

      data_offset += chunk.size();
    }

    for chunk in self.chunks {
      chunk.write_to(&mut writer)?;
    }

    Ok(())
  }
}

struct XcursorTocEntry {
  r#type: u32,
  subtype: u32,
}

impl XcursorTocEntry {
  const SIZE: usize = 12;
}

pub enum XcursorChunk<'s> {
  Comment(XcursorCommentChunk<'s>),
  Image(XcursorImageChunk),
}

impl XcursorChunk<'_> {
  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    match self {
      Self::Comment(c) => c.size(),
      Self::Image(c) => c.size(),
    }
  }
}

impl WriteTo for XcursorChunk<'_> {
  fn write_to<W: Write>(self, writer: W) -> IoResult<()> {
    match self {
      Self::Comment(c) => c.write_to(writer),
      Self::Image(c) => c.write_to(writer),
    }
  }
}

impl From<&XcursorChunk<'_>> for XcursorTocEntry {
  fn from(value: &XcursorChunk<'_>) -> Self {
    match value {
      XcursorChunk::Comment(c) => Self {
        r#type: XcursorCommentChunk::HEADER_TYPE,
        subtype: c.subtype as u32,
      },
      XcursorChunk::Image(c) => Self {
        r#type: XcursorImageChunk::HEADER_TYPE,
        subtype: c.nominal,
      },
    }
  }
}

pub struct XcursorCommentChunk<'s> {
  subtype: XcursorCommentSubtype,
  string: &'s str,
}

impl<'s> XcursorCommentChunk<'s> {
  const HEADER_SIZE: usize = 20;
  const HEADER_TYPE: u32 = 0xfffe0001;
  const HEADER_VERSION: u32 = 1;

  /// Creates a new `XcursorCommentChunk`.
  ///
  /// # Panics
  ///
  /// Panics if `string` length exceeds `u32::MAX`.
  pub const fn new(subtype: XcursorCommentSubtype, string: &'s str) -> Self {
    assert!(
      string.len() <= u32::MAX as usize,
      "string length should be ≤ u32::MAX"
    );

    Self { subtype, string }
  }

  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    Self::HEADER_SIZE + self.string.len()
  }
}

impl WriteTo for XcursorCommentChunk<'_> {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_u32::<LittleEndian>(Self::HEADER_SIZE as u32)?;
    writer.write_u32::<LittleEndian>(Self::HEADER_TYPE)?;
    writer.write_u32::<LittleEndian>(self.subtype as u32)?;
    writer.write_u32::<LittleEndian>(Self::HEADER_VERSION)?;
    writer.write_u32::<LittleEndian>(self.string.len() as u32)?;

    writer.write_all(self.string.as_bytes())
  }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum XcursorCommentSubtype {
  Copyright = 1,
  License = 2,
  Other = 3,
}

impl<'c> From<XcursorCommentChunk<'c>> for XcursorChunk<'c> {
  fn from(value: XcursorCommentChunk<'c>) -> Self {
    Self::Comment(value)
  }
}

pub struct XcursorImageChunk {
  nominal: u32,
  width: u32,
  height: u32,
  hotspot_x: u32,
  hotspot_y: u32,
  delay: Option<u32>,
  pixels: Vec<u8>,
}

impl XcursorImageChunk {
  const HEADER_SIZE: usize = 36;
  const HEADER_TYPE: u32 = 0xfffd0002;
  const HEADER_VERSION: u32 = 1;

  /// Creates a new `XcursorImageChunk`.
  ///
  /// # Panics
  ///
  /// Panics if `width` or `height` are zero, or if `hotspot_x` or `hotspot_y`
  /// are out of bounds.
  pub const fn new(
    nominal: u32,
    width: u32,
    height: u32,
    hotspot_x: u32,
    hotspot_y: u32,
    delay: Option<u32>,
    pixels: Vec<u8>,
  ) -> Self {
    assert!(width != 0, "width should be > 0");
    assert!(height != 0, "height should be > 0");

    assert!(hotspot_x <= width, "hotspot_x should be ≤ width");
    assert!(hotspot_y <= height, "hotspot_y should be ≤ height");

    Self {
      nominal,
      width,
      height,
      hotspot_x,
      hotspot_y,
      delay,
      pixels,
    }
  }

  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    Self::HEADER_SIZE + self.pixels.len()
  }
}

impl WriteTo for XcursorImageChunk {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_u32::<LittleEndian>(Self::HEADER_SIZE as u32)?;
    writer.write_u32::<LittleEndian>(Self::HEADER_TYPE)?;
    writer.write_u32::<LittleEndian>(self.nominal)?;
    writer.write_u32::<LittleEndian>(Self::HEADER_VERSION)?;
    writer.write_u32::<LittleEndian>(self.width)?;
    writer.write_u32::<LittleEndian>(self.height)?;
    writer.write_u32::<LittleEndian>(self.hotspot_x)?;
    writer.write_u32::<LittleEndian>(self.hotspot_y)?;
    writer.write_u32::<LittleEndian>(self.delay.unwrap_or(0))?;

    writer.write_all(&self.pixels)
  }
}

impl From<XcursorImageChunk> for XcursorChunk<'_> {
  fn from(value: XcursorImageChunk) -> Self {
    Self::Image(value)
  }
}
