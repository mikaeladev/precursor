use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::cursors::{CursorFile, Hotspot};

const FILE_HEADER_SIZE: u32 = 6;
const ICON_ENTRY_SIZE: u32 = 16;

pub struct CurFile {
  entries: Vec<CurIconEntry>,
  pngs: Vec<Box<[u8]>>,
}

impl CurFile {
  /// Constructs a new `CurFile`.
  ///
  /// # Panics
  ///
  /// Panics if any [hotspot] in `icons` cannot be coerced to `u16`.
  ///
  /// [hotspot]: Hotspot
  pub fn new(icons: Vec<CurIcon>) -> Self {
    let icons_len = icons.len();

    let mut entries = Vec::with_capacity(icons_len);
    let mut pngs = Vec::with_capacity(icons_len);

    let mut data_pos = FILE_HEADER_SIZE + ICON_ENTRY_SIZE * icons_len as u32;

    for CurIcon {
      width,
      height,
      hotspot_x,
      hotspot_y,
      buffer,
    } in icons
    {
      let data_size = buffer.len() as u32;

      let entry = CurIconEntry {
        width: if width > 255 { 0 } else { width as u8 },
        height: if height > 255 { 0 } else { height as u8 },
        hotspot_x,
        hotspot_y,
        data_size,
        data_pos,
      };

      data_pos += data_size;

      entries.push(entry);
      pngs.push(buffer);
    }

    Self { entries, pngs }
  }
}

impl CursorFile for CurFile {
  fn size(&self) -> usize {
    self.pngs.iter().fold(
      FILE_HEADER_SIZE as usize + ICON_ENTRY_SIZE as usize * self.pngs.len(),
      |acc, png| acc + png.len(),
    )
  }

  fn write<W: Write>(self, writer: &mut W) -> io::Result<()> {
    writer.write_u16::<LittleEndian>(0)?; // reserved
    writer.write_u16::<LittleEndian>(2)?; // magic type
    writer.write_u16::<LittleEndian>(self.entries.len() as u16)?;

    for entry in self.entries {
      writer.write_u8(entry.width)?;
      writer.write_u8(entry.height)?;

      writer.write_u8(0)?; // colour count (0 for 8-bit)
      writer.write_u8(0)?; // reserved

      writer.write_u16::<LittleEndian>(entry.hotspot_x)?;
      writer.write_u16::<LittleEndian>(entry.hotspot_y)?;

      writer.write_u32::<LittleEndian>(entry.data_size)?;
      writer.write_u32::<LittleEndian>(entry.data_pos)?;
    }

    for png in self.pngs {
      writer.write_all(&png)?;
    }

    Ok(())
  }
}

pub struct CurIcon {
  width: u16,
  height: u16,
  hotspot_x: u16,
  hotspot_y: u16,
  buffer: Box<[u8]>,
}

impl CurIcon {
  /// Constructs a new `CurIcon`.
  ///
  /// # Panics
  ///
  /// Panics if `hotspot` is out of bounds.
  pub const fn new(
    width: u16,
    height: u16,
    hotspot: Hotspot,
    buffer: Box<[u8]>,
  ) -> Self {
    let hotspot_x = hotspot.x as u16;
    let hotspot_y = hotspot.y as u16;

    assert!(hotspot_x <= width, "hotspot_x should be ≤ width");
    assert!(hotspot_y <= height, "hotspot_y should be ≤ height");

    Self {
      width,
      height,
      hotspot_x,
      hotspot_y,
      buffer,
    }
  }
}

struct CurIconEntry {
  width: u8,
  height: u8,
  hotspot_x: u16,
  hotspot_y: u16,
  data_size: u32,
  data_pos: u32,
}
