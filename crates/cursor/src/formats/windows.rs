use std::io::{Result as IoResult, Write};

use crate_formats::PngImage;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::Cursor;

pub struct WindowsCursor<'c>(&'c Cursor);

impl WindowsCursor<'_> {
  pub fn write<W: Write>(self, writer: W) -> IoResult<()> {
    if self.0.frames.len() == 1 {
      self.write_cur(writer)
    } else {
      self.write_ani(writer)
    }
  }

  /// Writes a CUR file to the writer.
  fn write_cur<W: Write>(self, mut writer: W) -> IoResult<()> {
    let WindowsCursor(Cursor {
      frames,
      metadata: _,
    }) = self;

    let frame = frames.first().expect("first frame should exist");

    let num_images = frame.images.len();

    writer.write_u16::<LittleEndian>(0)?; // reserved
    writer.write_u16::<LittleEndian>(2)?; // type
    writer.write_u16::<LittleEndian>(num_images as u16)?;

    let mut buffer_offset = 6 + 16 * (num_images as u32);

    let mut image_bufs = Vec::with_capacity(num_images);

    for image in frame.images.iter() {
      let nominal = image.nominal;
      let nominal = if nominal > 255 { 0 } else { nominal as u8 };

      writer.write_u8(nominal)?;
      writer.write_u8(nominal)?;

      writer.write_u8(0)?; // ignored
      writer.write_u8(0)?; // reserved

      writer.write_u16::<LittleEndian>(image.hotspot.x)?;
      writer.write_u16::<LittleEndian>(image.hotspot.y)?;

      let mut image_buf = // FIXME: 92 is magic
        Vec::with_capacity(image.raster.pixels().len() * 4 + 92);

      image_buf.write_all(&image.raster.encode_png()?)?;

      let buffer_len = image_buf.len() as u32;

      image_bufs.push(image_buf);

      writer.write_u32::<LittleEndian>(buffer_len)?;
      writer.write_u32::<LittleEndian>(buffer_offset)?;

      buffer_offset += buffer_len;
    }

    for image_buf in image_bufs {
      writer.write_all(&image_buf)?;
    }

    Ok(())
  }

  /// Writes an ANI file to the writer.
  fn write_ani<W: Write>(self, mut writer: W) -> IoResult<()> {
    // chunk identifiers in order of appearance
    const RIFF: &'static [u8] = b"RIFF";
    const ACON: &'static [u8] = b"ACON";
    const ANIH: &'static [u8] = b"anih";
    const RATE: &'static [u8] = b"rate";
    const SEQU: &'static [u8] = b"seq ";
    const LIST: &'static [u8] = b"LIST";
    const FRAM: &'static [u8] = b"fram";
    const ICON: &'static [u8] = b"icon";

    let WindowsCursor(Cursor { frames, metadata }) = self;

    let num_frames = frames.len();

    let mut icon_buffers = Vec::with_capacity(num_frames);

    for frame in frames {
      let cursor = WindowsCursor(&Cursor {
        frames: vec![frame.clone()],
        metadata: metadata.clone(),
      });

      let mut icon_buf = Vec::new();
      cursor.write_cur(&mut icon_buf)?;

      icon_buffers.push(icon_buf);
    }

    let list_data_len =
      icon_buffers.iter().fold(4, |acc, buf| acc + 4 + buf.len());

    let data_len = 68 + (8 * num_frames) + list_data_len;

    writer.write_all(RIFF)?;
    writer.write_u32::<LittleEndian>(data_len as u32)?;

    writer.write_all(ACON)?;

    writer.write_all(ANIH)?;
    writer.write_u32::<LittleEndian>(36)?; // header size
    writer.write_u32::<LittleEndian>(num_frames as u32)?;
    writer.write_u32::<LittleEndian>(num_frames as u32)?;
    writer.write_u32::<LittleEndian>(0)?; // width (unused)
    writer.write_u32::<LittleEndian>(0)?; // height (unused)
    writer.write_u32::<LittleEndian>(0)?; // colour depth (unused)
    writer.write_u32::<LittleEndian>(0)?; // num planes (unused)
    writer.write_u32::<LittleEndian>(0)?; // default rate
    writer.write_u32::<LittleEndian>(1)?; // sequence flag

    writer.write_all(RATE)?;

    for frame in frames.iter() {
      let duration = frame.duration.expect("duration should not be None");
      writer.write_u32::<LittleEndian>(duration.jiffies())?;
    }

    writer.write_all(SEQU)?;

    for index in 0..=num_frames {
      writer.write_u32::<LittleEndian>(index as u32)?;
    }

    writer.write_all(LIST)?;
    writer.write_u32::<LittleEndian>(list_data_len as u32)?;

    writer.write_all(FRAM)?;

    for icon_buf in icon_buffers {
      writer.write_all(ICON)?;
      writer.write_all(&icon_buf)?;
    }

    Ok(())
  }
}

impl<'c> From<&'c Cursor> for WindowsCursor<'c> {
  fn from(value: &'c Cursor) -> Self {
    Self(value)
  }
}
