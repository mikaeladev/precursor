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
        ChunkValue::anih(icons_len, sequence_len)?,
        ChunkValue::rate(rates)?,
        ChunkValue::seq_(sequence)?,
        ChunkValue::fram(icons)?,
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
    self.0.size()
  }
}

trait ChunkValueAniExt {
  fn anih(icons_len: u32, sequence_len: u32) -> io::Result<ChunkValue> {
    let mut header_buf = Vec::with_capacity(40);

    header_buf.write_all(b"anih")?;
    header_buf.write_u32::<LittleEndian>(36)?; // header size
    header_buf.write_u32::<LittleEndian>(icons_len as u32)?;
    header_buf.write_u32::<LittleEndian>(sequence_len as u32)?;
    header_buf.write_u32::<LittleEndian>(0)?; // width (unused)
    header_buf.write_u32::<LittleEndian>(0)?; // height (unused)
    header_buf.write_u32::<LittleEndian>(0)?; // colour depth (unused)
    header_buf.write_u32::<LittleEndian>(0)?; // num planes (unused)
    header_buf.write_u32::<LittleEndian>(0)?; // default rate (unused)
    header_buf.write_u32::<LittleEndian>(1)?; // sequence flag

    Ok(ChunkValue::Raw(header_buf))
  }

  fn rate(rates: Vec<u32>) -> io::Result<ChunkValue> {
    let mut rates_buf = Vec::with_capacity(4 * (1 + rates.len()));

    rates_buf.write_all(b"rate")?;
    for rate in rates {
      rates_buf.write_u32::<LittleEndian>(rate)?;
    }

    Ok(ChunkValue::Raw(rates_buf))
  }

  fn seq_(sequence: Vec<u32>) -> io::Result<ChunkValue> {
    let mut sequence_buf = Vec::with_capacity(4 * (1 + sequence.len()));

    sequence_buf.write_all(b"seq ")?;
    for index in sequence {
      sequence_buf.write_u32::<LittleEndian>(index)?;
    }

    Ok(ChunkValue::Raw(sequence_buf))
  }

  fn fram(icons: Vec<CurFile>) -> io::Result<ChunkValue> {
    let mut frame_chunks = Vec::with_capacity(icons.len());

    for icon in icons {
      let mut buf = Vec::with_capacity(4 + icon.exact_size());

      buf.write_all(b"icon")?;
      icon.write(&mut buf)?;

      frame_chunks.push(ChunkValue::Raw(buf));
    }

    Ok(ChunkValue::list(ChunkId(*b"fram"), frame_chunks))
  }
}

impl ChunkValueAniExt for ChunkValue {}
