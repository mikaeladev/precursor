use std::io::{Result as IoResult, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::cursors::{CursorDuration, CursorImage};

pub struct StaticWindowsCursor {
  images: Vec<CursorImage>,
}

impl StaticWindowsCursor {
  /// Creates a new static windows cursor.
  ///
  /// # Panics
  /// Panics if the number of images exceeds `u16::MAX` bytes.
  pub const fn new(images: Vec<CursorImage>) -> Self {
    if images.len() > u16::MAX as usize {
      panic!("images length should not exceed u16::MAX")
    }

    Self { images }
  }

  /// Writes a CUR file to the writer.
  pub fn write<W: Write>(self, mut writer: W) -> IoResult<()> {
    let num_images = self.images.len();

    writer.write_u16::<LittleEndian>(0)?; // reserved
    writer.write_u16::<LittleEndian>(2)?; // type
    writer.write_u16::<LittleEndian>(num_images as u16)?;

    let mut buffer_offset = 6 + 16 * (num_images as u32);

    for image in self.images.iter() {
      let hotspot = image.hotspot();
      let size = image.size();

      // a zero value indicates a size >= 256
      let size = if size > 255 { 0 } else { size as u8 };

      writer.write_u8(size)?; // width
      writer.write_u8(size)?; // height
      writer.write_u8(0)?; // ignored
      writer.write_u8(0)?; // reserved
      writer.write_u16::<LittleEndian>(hotspot.0)?;
      writer.write_u16::<LittleEndian>(hotspot.1)?;

      let buffer_len = image.buffer().len() as u32;

      writer.write_u32::<LittleEndian>(buffer_len)?;
      writer.write_u32::<LittleEndian>(buffer_offset)?;

      buffer_offset += buffer_len;
    }

    for image in self.images {
      writer.write_all(&image.into_buffer())?;
    }

    Ok(())
  }
}

pub struct AnimatedWindowsCursor {
  frames: Vec<StaticWindowsCursor>,
  steps: Vec<(u32, CursorDuration)>,
}

impl AnimatedWindowsCursor {
  // chunk identifiers in order of appearance
  const RIFF: &[u8] = b"RIFF";
  const ACON: &[u8] = b"ACON";
  const ANIH: &[u8] = b"anih";
  const RATE: &[u8] = b"rate";
  const SEQU: &[u8] = b"seq ";
  const LIST: &[u8] = b"LIST";
  const FRAM: &[u8] = b"fram";
  const ICON: &[u8] = b"icon";

  /// Creates a new animated windows cursor.
  ///
  /// # Panics
  /// Panics if the number of frames or steps exceeds `u32::MAX` bytes, or if
  /// the number of frames is greater than the number of steps.
  pub const fn new(
    frames: Vec<StaticWindowsCursor>,
    steps: Vec<(u32, CursorDuration)>,
  ) -> Self {
    let frames_len = frames.len() as u32;
    let steps_len = steps.len() as u32;

    if frames_len > steps_len {
      panic!("should not be more frames than steps")
    }

    Self { frames, steps }
  }

  /// Writes an ANI file to the writer.
  pub fn write<W: Write>(self, mut writer: W) -> IoResult<()> {
    let header_chunk =
      Self::header_chunk(self.frames.len() as u32, self.steps.len() as u32)?;

    let rates_chunk =
      Self::rates_chunk(self.steps.iter().map(|s| s.1).collect())?;

    let sequence_chunk =
      Self::sequence_chunk(self.steps.iter().map(|s| s.0).collect())?;

    let frames_chunk = Self::frames_chunk(self.frames)?;

    let data_len = 4
      + header_chunk.len()
      + rates_chunk.len()
      + sequence_chunk.len()
      + frames_chunk.len();

    writer.write_all(Self::RIFF)?;
    writer.write_u32::<LittleEndian>(data_len as u32)?;

    writer.write_all(Self::ACON)?;
    writer.write_all(&header_chunk)?;
    writer.write_all(&rates_chunk)?;
    writer.write_all(&sequence_chunk)?;
    writer.write_all(&frames_chunk)?;

    Ok(())
  }

  /// Creates a header chunk.
  fn header_chunk(num_frames: u32, num_steps: u32) -> IoResult<Vec<u8>> {
    let mut chunk = Vec::with_capacity(40);

    chunk.write_all(Self::ANIH)?;
    chunk.write_u32::<LittleEndian>(36)?; // header size
    chunk.write_u32::<LittleEndian>(num_frames)?;
    chunk.write_u32::<LittleEndian>(num_steps)?;
    chunk.write_u32::<LittleEndian>(0)?; // width (unused)
    chunk.write_u32::<LittleEndian>(0)?; // height (unused)
    chunk.write_u32::<LittleEndian>(0)?; // colour depth (unused)
    chunk.write_u32::<LittleEndian>(0)?; // num planes (unused)
    chunk.write_u32::<LittleEndian>(0)?; // default rate
    chunk.write_u32::<LittleEndian>(1)?; // sequence flag

    Ok(chunk)
  }

  /// Creates a rate chunk.
  fn rates_chunk(rates: Vec<CursorDuration>) -> IoResult<Vec<u8>> {
    let mut chunk = Vec::with_capacity(8 + 4 * rates.len());

    chunk.write_all(b"rate")?;

    for rate in rates {
      chunk.write_u32::<LittleEndian>(rate.jiffies())?;
    }

    Ok(chunk)
  }

  /// Creates a sequence chunk.
  fn sequence_chunk(sequence: Vec<u32>) -> IoResult<Vec<u8>> {
    let mut chunk = Vec::with_capacity(8 + 4 * sequence.len());

    chunk.write_all(Self::SEQU)?;

    for seq in sequence {
      chunk.write_u32::<LittleEndian>(seq)?;
    }

    Ok(chunk)
  }

  /// Creates a frames chunk.
  fn frames_chunk(frames: Vec<StaticWindowsCursor>) -> IoResult<Vec<u8>> {
    let mut chunk = Vec::new();

    let mut icon_buffers = Vec::with_capacity(frames.len());

    for frame in frames {
      let mut icon_buf = Vec::new();
      frame.write(&mut icon_buf)?;
      icon_buffers.push(icon_buf);
    }

    let data_len = icon_buffers.iter().fold(4, |acc, buf| acc + 4 + buf.len());

    chunk.write_all(Self::LIST)?;
    chunk.write_u32::<LittleEndian>(data_len as u32)?;

    chunk.write_all(Self::FRAM)?;

    for icon_buf in icon_buffers {
      chunk.write_all(Self::ICON)?;
      chunk.write_all(&icon_buf)?;
    }

    Ok(chunk)
  }
}
