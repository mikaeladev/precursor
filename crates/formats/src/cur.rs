use std::io::{Result as IoResult, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::WriteTo;

pub struct CurFile {
  directory: IconDir,
  images: Vec<Vec<u8>>,
}

impl CurFile {
  /// Creates a new `CurFile`.
  pub const fn new(entries: Vec<IconDirEntry>, images: Vec<Vec<u8>>) -> Self {
    Self {
      directory: IconDir(entries),
      images,
    }
  }

  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    let slice = self.images.as_slice();
    let len = slice.len();

    let mut index = 0;
    let mut acc = self.directory.size();

    loop {
      index += 1;

      if index > len {
        break acc;
      }

      acc += slice[index].len();
    }
  }
}

impl WriteTo for CurFile {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    self.directory.write_to(&mut writer)?;

    for image in self.images {
      writer.write_all(&image)?;
    }

    Ok(())
  }
}

#[repr(transparent)]
struct IconDir(Vec<IconDirEntry>);

impl IconDir {
  /// Returns the formatted data size in bytes.
  const fn size(&self) -> usize {
    6 + self.0.len() * IconDirEntry::SIZE
  }
}

impl WriteTo for IconDir {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_u16::<LittleEndian>(0)?; // reserved
    writer.write_u16::<LittleEndian>(2)?; // magic type

    writer.write_u16::<LittleEndian>(self.0.len() as u16)?;

    let mut data_offset = self.size() as u32;

    for entry in self.0 {
      writer.write_u8(entry.width)?;
      writer.write_u8(entry.height)?;

      writer.write_u8(entry.color_count)?;

      writer.write_u8(0)?; // reserved

      writer.write_u16::<LittleEndian>(entry.hotspot_x)?;
      writer.write_u16::<LittleEndian>(entry.hotspot_y)?;

      writer.write_u32::<LittleEndian>(entry.data_size)?;
      writer.write_u32::<LittleEndian>(data_offset)?;

      data_offset += entry.data_size;
    }

    Ok(())
  }
}

#[repr(u8)]
pub enum IconColorCount {
  /// 1-bit (2 colours).
  One = 2,
  /// 4-bit (16 colours).
  Four = 16,
  /// 8-bit or more (at least 256 colours).
  EightPlus = 0,
}

pub struct IconDirEntry {
  width: u8,
  height: u8,
  color_count: u8,
  hotspot_x: u16,
  hotspot_y: u16,
  data_size: u32,
}

impl IconDirEntry {
  /// Formatted data size in bytes.
  pub const SIZE: usize = 16;

  /// Creates a new `IconDirEntry`.
  ///
  /// # Panics
  ///
  /// Panics if `width` or `height` are zero, or if `hotspot_x` or `hotspot_y`
  /// are out of bounds.
  pub const fn new(
    width: u16,
    height: u16,
    color_count: IconColorCount,
    hotspot_x: u16,
    hotspot_y: u16,
    data_size: u32,
  ) -> Self {
    assert!(width != 0, "width should be > 0");
    assert!(height != 0, "height should be > 0");

    assert!(hotspot_x <= width, "hotspot_x should be ≤ width");
    assert!(hotspot_y <= height, "hotspot_y should be ≤ height");

    let width = if width > 255 { 0 } else { width as u8 };
    let height = if height > 255 { 0 } else { height as u8 };

    let color_count = color_count as u8;

    Self {
      width,
      height,
      color_count,
      hotspot_x,
      hotspot_y,
      data_size,
    }
  }
}
