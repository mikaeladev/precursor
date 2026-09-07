use std::io::Write;

use crate_pixmap::{DynamicPixmap, Pixmap};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::cursors::Hotspot;
use crate::png::PngImage;
use crate::rasters::RasterResult;
use crate::write::{WriteResult, WriteTo};

pub struct CurFile {
  entries: Vec<CurIconEntry>,
  pngs: Vec<Vec<u8>>,
}

impl CurFile {
  const HEADER_SIZE: usize = 6;

  /// Attempts to construct a new `CurFile`.
  ///
  /// # Errors
  ///
  /// Fails with a `RasterError` if any icon pixmap in `icons` is malformed.
  pub fn new(icons: Vec<CurIcon>) -> RasterResult<Self> {
    let num_icons = icons.len();

    let mut entries = Vec::with_capacity(num_icons);
    let mut pngs = Vec::with_capacity(num_icons);

    let mut data_offset =
      (Self::HEADER_SIZE + CurIconEntry::SIZE * num_icons) as u32;

    for CurIcon { hotspot, pixmap } in icons {
      let (width, height) = pixmap.dimensions();

      let png = pixmap.encode_png()?;
      let png_size = png.len() as u32;

      let entry = CurIconEntry {
        width: if width > 255 { 0 } else { width as u8 },
        height: if height > 255 { 0 } else { height as u8 },
        hotspot_x: hotspot.x as u16,
        hotspot_y: hotspot.y as u16,
        data_size: png_size,
        data_offset: data_offset,
      };

      data_offset += png_size;

      entries.push(entry);
      pngs.push(png);
    }

    Ok(Self { entries, pngs })
  }

  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    let slice = self.pngs.as_slice();
    let len = slice.len();

    let mut index = 0;
    let mut acc = Self::HEADER_SIZE + CurIconEntry::SIZE * len;

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
  fn write_to<W: Write>(self, mut writer: W) -> WriteResult {
    writer.write_u16::<LittleEndian>(0)?; // reserved
    writer.write_u16::<LittleEndian>(2)?; // magic type
    writer.write_u16::<LittleEndian>(self.entries.len() as u16)?;

    for entry in self.entries {
      entry.write_to(&mut writer)?;
    }

    for png in self.pngs {
      writer.write_all(&png)?;
    }

    Ok(())
  }
}

pub struct CurIcon {
  hotspot: Hotspot,
  pixmap: DynamicPixmap,
}

impl CurIcon {
  /// Constructs a new `CurIcon`.
  ///
  /// # Panics
  ///
  /// Panics if `hotspot` is out of bounds.
  pub fn new(hotspot: Hotspot, pixmap: DynamicPixmap) -> Self {
    let (width, height) = pixmap.dimensions();

    assert!(hotspot.x <= width, "hotspot.x should be ≤ pixmap width");
    assert!(hotspot.y <= height, "hotspot.y should be ≤ pixmap height");

    Self { hotspot, pixmap }
  }
}

struct CurIconEntry {
  width: u8,
  height: u8,
  hotspot_x: u16,
  hotspot_y: u16,
  data_size: u32,
  data_offset: u32,
}

impl CurIconEntry {
  /// Formatted data size in bytes.
  pub const SIZE: usize = 16;
}

impl WriteTo for CurIconEntry {
  fn write_to<W: Write>(self, mut writer: W) -> WriteResult {
    writer.write_u8(self.width)?;
    writer.write_u8(self.height)?;

    writer.write_u8(0)?; // colour count (0 for 8-bit)
    writer.write_u8(0)?; // reserved

    writer.write_u16::<LittleEndian>(self.hotspot_x)?;
    writer.write_u16::<LittleEndian>(self.hotspot_y)?;

    writer.write_u32::<LittleEndian>(self.data_size)?;
    writer.write_u32::<LittleEndian>(self.data_offset)?;

    Ok(())
  }
}
