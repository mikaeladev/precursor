use std::io::{Result as IoResult, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{CurFile, WriteTo};

pub struct AniFile {
  header: HeaderChunk,
  rates: RatesChunk,
  sequence: SequenceChunk,
  frames: FramesChunk,
}

const ID_SIZE: usize = 4;
const LEN_SIZE: usize = 4;

impl AniFile {
  /// Creates a new `AniFile`.
  pub const fn new(
    icons: Vec<CurFile>,
    rates: Vec<u32>,
    sequence: Vec<u32>,
  ) -> Self {
    Self {
      header: HeaderChunk::new(icons.len() as u32, sequence.len() as u32),
      rates: RatesChunk(rates),
      sequence: SequenceChunk(sequence),
      frames: FramesChunk(icons),
    }
  }

  /// Returns the formatted data size in bytes.
  pub const fn size(&self) -> usize {
    ID_SIZE + LEN_SIZE + self.inner_size()
  }

  /// Returns the inner formatted data size in bytes.
  const fn inner_size(&self) -> usize {
    ID_SIZE
      + HeaderChunk::SIZE
      + self.rates.size()
      + self.sequence.size()
      + ID_SIZE
      + LEN_SIZE
      + self.frames.size()
  }
}

impl WriteTo for AniFile {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    let riff_size = (self.inner_size()) as u32;

    writer.write_all(b"RIFF")?;
    writer.write_u32::<LittleEndian>(riff_size)?;

    writer.write_all(b"ACON")?;
    self.header.write_to(&mut writer)?;
    self.rates.write_to(&mut writer)?;
    self.sequence.write_to(&mut writer)?;

    let list_size = self.frames.size() as u32;

    writer.write_all(b"LIST")?;
    writer.write_u32::<LittleEndian>(list_size)?;

    self.frames.write_to(writer)?;

    Ok(())
  }
}

struct HeaderChunk {
  /// Number of unique images in the animation.
  icon_count: u32,
  /// Number of frames in the animation.
  frame_count: u32,
}

impl HeaderChunk {
  /// Formatted data size in bytes.
  const SIZE: usize = ID_SIZE + Self::INNER_SIZE;

  /// Inner formatted data size in bytes.
  const INNER_SIZE: usize = 36;

  /// Creates a new `HeaderChunk`.
  const fn new(icon_count: u32, frame_count: u32) -> Self {
    assert!(
      icon_count <= frame_count,
      "icon_count should be ≤ frame_count"
    );

    Self {
      icon_count,
      frame_count,
    }
  }
}

impl WriteTo for HeaderChunk {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_all(b"anih")?;

    writer.write_u32::<LittleEndian>(Self::INNER_SIZE as u32)?;
    writer.write_u32::<LittleEndian>(self.icon_count)?;
    writer.write_u32::<LittleEndian>(self.frame_count)?;
    writer.write_u32::<LittleEndian>(0)?; // width (unused)
    writer.write_u32::<LittleEndian>(0)?; // height (unused)
    writer.write_u32::<LittleEndian>(0)?; // colour depth (unused)
    writer.write_u32::<LittleEndian>(0)?; // num planes (unused)
    writer.write_u32::<LittleEndian>(0)?; // default rate (unused)
    writer.write_u32::<LittleEndian>(1)?; // sequence flag

    Ok(())
  }
}

struct RatesChunk(Vec<u32>);

impl RatesChunk {
  /// Returns the formatted data size in bytes.
  const fn size(&self) -> usize {
    ID_SIZE + self.0.len()
  }
}

impl WriteTo for RatesChunk {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_all(b"rate")?;

    for rate in self.0.into_iter() {
      writer.write_u32::<LittleEndian>(rate)?;
    }

    Ok(())
  }
}

struct SequenceChunk(Vec<u32>);

impl SequenceChunk {
  /// Returns the formatted data size in bytes.
  const fn size(&self) -> usize {
    ID_SIZE + self.0.len()
  }
}

impl WriteTo for SequenceChunk {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_all(b"seq ")?;

    for seq in self.0.into_iter() {
      writer.write_u32::<LittleEndian>(seq)?;
    }

    Ok(())
  }
}

struct FramesChunk(Vec<CurFile>);

impl FramesChunk {
  /// Returns the formatted data size in bytes.
  const fn size(&self) -> usize {
    ID_SIZE + self.inner_size()
  }

  /// Returns the inner formatted data size in bytes.
  const fn inner_size(&self) -> usize {
    let slice = self.0.as_slice();
    let len = slice.len();

    let mut index = 0;
    let mut acc = 0;

    loop {
      index += 1;

      if index > len {
        break acc;
      }

      acc += ID_SIZE + slice[index].size()
    }
  }
}

impl WriteTo for FramesChunk {
  fn write_to<W: Write>(self, mut writer: W) -> IoResult<()> {
    writer.write_all(b"fram")?;

    for icon in self.0 {
      writer.write_all(b"icon")?;
      icon.write_to(&mut writer)?;
    }

    Ok(())
  }
}
