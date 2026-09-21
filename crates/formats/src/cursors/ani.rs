use std::io::{self, Seek, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::containers::riff::{ChunkId, ChunkValue};
use crate::cursors::cur::CurFile;

pub struct AniFile(ChunkValue);

impl AniFile {
  /// Constructs a new `AniFile`.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  ///
  /// # Panics
  ///
  /// Panics if there are more `icons` than indices in `sequence`.
  ///
  /// [`Write::write_all`]: Write::write_all
  pub fn new(
    icons: Vec<CurFile>,
    rates: Vec<u32>,
    sequence: Vec<u32>,
  ) -> io::Result<Self> {
    let icons_len = icons.len() as u32;
    let sequence_len = sequence.len() as u32;

    assert!(
      icons_len <= sequence_len,
      "icons_len should be ≤ sequence_len"
    );

    Ok(Self(ChunkValue::riff(
      ChunkId(*b"ACON"),
      vec![
        header_chunk(icons_len, sequence_len)?,
        rates_chunk(rates)?,
        sequence_chunk(sequence)?,
        frames_chunk(icons)?,
      ],
    )))
  }

  /// Writes an ANI file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  ///
  /// [`Write::write_all`]: Write::write_all
  pub fn write<W: Write + Seek>(self, writer: &mut W) -> io::Result<usize> {
    self.0.write(writer)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    self.0.exact_size()
  }
}

fn header_chunk(icons_len: u32, sequence_len: u32) -> io::Result<ChunkValue> {
  let mut buffer = Vec::with_capacity(40);

  buffer.write_all(b"anih")?;
  buffer.write_u32::<LittleEndian>(36)?; // header size
  buffer.write_u32::<LittleEndian>(icons_len)?;
  buffer.write_u32::<LittleEndian>(sequence_len)?;
  buffer.write_u32::<LittleEndian>(0)?; // width (unused)
  buffer.write_u32::<LittleEndian>(0)?; // height (unused)
  buffer.write_u32::<LittleEndian>(0)?; // colour depth (unused)
  buffer.write_u32::<LittleEndian>(0)?; // num planes (unused)
  buffer.write_u32::<LittleEndian>(0)?; // default rate (unused)
  buffer.write_u32::<LittleEndian>(1)?; // sequence flag

  Ok(ChunkValue::Raw(buffer))
}

fn rates_chunk(rates: Vec<u32>) -> io::Result<ChunkValue> {
  let mut buffer = Vec::with_capacity(4 * (1 + rates.len()));

  buffer.write_all(b"rate")?;
  for rate in rates {
    buffer.write_u32::<LittleEndian>(rate)?;
  }

  Ok(ChunkValue::Raw(buffer))
}

fn sequence_chunk(sequence: Vec<u32>) -> io::Result<ChunkValue> {
  let mut buffer = Vec::with_capacity(4 * (1 + sequence.len()));

  buffer.write_all(b"seq ")?;
  for index in sequence {
    buffer.write_u32::<LittleEndian>(index)?;
  }

  Ok(ChunkValue::Raw(buffer))
}

fn frames_chunk(icons: Vec<CurFile>) -> io::Result<ChunkValue> {
  let mut subchunks = Vec::with_capacity(icons.len());

  for icon in icons {
    let mut buffer = Vec::with_capacity(4 + icon.exact_size());

    buffer.write_all(b"icon")?;
    icon.write(&mut buffer)?;

    subchunks.push(ChunkValue::Raw(buffer));
  }

  Ok(ChunkValue::list(ChunkId(*b"fram"), subchunks))
}
