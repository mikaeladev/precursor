use std::io::{self, Seek, Write};

use crate::containers::riff::{ChunkId, ChunkValue};
use crate::cursors::cur::CurFile;

pub struct AniFile(ChunkValue);

impl AniFile {
  /// Constructs a new `AniFile`.
  ///
  /// # Panics
  ///
  /// Panics if there are more `icons` than indices in `sequence`.
  ///
  /// [`Write::write_all`]: Write::write_all
  pub fn new(icons: Vec<CurFile>, rates: Vec<u32>, sequence: Vec<u32>) -> Self {
    let icons_len = icons.len() as u32;
    let sequence_len = sequence.len() as u32;

    assert!(
      icons_len <= sequence_len,
      "icons_len should be ≤ sequence_len"
    );

    Self(ChunkValue::Parent(
      ChunkId::RIFF,
      ChunkId(*b"ACON"),
      vec![
        header_chunk(icons_len, sequence_len),
        rates_chunk(rates),
        sequence_chunk(sequence),
        frames_chunk(icons),
      ],
    ))
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

fn header_chunk(icons_len: u32, sequence_len: u32) -> ChunkValue {
  ChunkValue::Child(
    ChunkId(*b"anih"),
    [
      36_u32.to_le_bytes(),       // header size
      icons_len.to_le_bytes(),    // num icons
      sequence_len.to_le_bytes(), // num frames
      [0; 4],                     // width (unused)
      [0; 4],                     // height (unused)
      [0; 4],                     // colour depth (unused)
      [0; 4],                     // num planes (unused)
      [0; 4],                     // default rate (unused)
      1_u32.to_le_bytes(),        // sequence flag
    ]
    .concat(),
  )
}

fn rates_chunk(rates: Vec<u32>) -> ChunkValue {
  ChunkValue::Child(
    ChunkId(*b"rate"),
    rates.into_iter().flat_map(|r| r.to_le_bytes()).collect(),
  )
}

fn sequence_chunk(sequence: Vec<u32>) -> ChunkValue {
  ChunkValue::Child(
    ChunkId(*b"seq "),
    sequence.into_iter().flat_map(|i| i.to_le_bytes()).collect(),
  )
}

fn frames_chunk(icons: Vec<CurFile>) -> ChunkValue {
  ChunkValue::Parent(
    ChunkId::LIST,
    ChunkId(*b"fram"),
    icons
      .into_iter()
      .map(|icon| {
        let mut buffer = Vec::with_capacity(icon.exact_size());
        icon.write(&mut buffer).unwrap(); // writing to memory, should be fine?

        ChunkValue::Child(ChunkId(*b"icon"), buffer)
      })
      .collect(),
  )
}
