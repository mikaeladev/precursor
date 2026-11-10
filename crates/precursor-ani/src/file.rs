use std::io::{Read, Seek, Write};

use precursor_cur::CurFile;
use precursor_error::{ErrorMessage, Result};
use precursor_riff::ChunkValue;

use crate::acon::AconChunk;
use crate::header::AconHeader;

pub struct AniFile(AconChunk);

impl AniFile {
  /// Constructs a new `AconChunk`.
  ///
  /// # Panics
  ///
  /// Panics if there are more images in `frames` than indexes in `sequence`.
  pub fn new(
    frames: Vec<CurFile>,
    rates: Vec<u32>,
    sequence: Vec<u32>,
  ) -> Self {
    let frame_count = frames.len() as u32;
    let step_count = sequence.len() as u32;

    assert!(
      frame_count <= step_count,
      "frame_count should be ≤ step_count"
    );

    let header = AconHeader {
      frame_count,
      step_count,
      jif_rate: 0,
    };

    Self(AconChunk {
      header,
      rates: Some(rates),
      sequence: Some(sequence),
      frames,
    })
  }

  /// Reads an ANI file from `reader`, returning the constructed `AniFile`.
  ///
  /// # Errors
  ///
  /// Fails with a user-friendly [`Error`].
  ///
  /// [`Error`]: precursor_error::Error
  pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
    let chunk = ChunkValue::read(reader).map_err(|src| {
      ErrorMessage::new("failed to read ANI file")
        .with_subheading(src.kind())
        .into_error()
        .with_source(src)
    })?;

    Ok(Self(AconChunk::from_chunk(chunk)?))
  }

  /// Writes an ANI file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// Fails with a user-friendly [`Error`].
  ///
  /// [`Error`]: precursor_error::Error
  pub fn write<W: Write + Seek>(self, writer: &mut W) -> Result<usize> {
    let chunk = self.into_chunk();

    chunk.write(writer).map_err(|src| {
      ErrorMessage::new("failed to write ANI file")
        .with_subheading(src.kind())
        .into_error()
        .with_source(src)
    })
  }

  // TODO: doc
  pub fn into_frames(self) -> Vec<CurFile> {
    let Self(acon) = self;
    acon.frames
  }

  /// Converts this value into a [`ChunkValue`].
  fn into_chunk(self) -> ChunkValue {
    let Self(acon) = self;
    acon.into_chunk()
  }
}
