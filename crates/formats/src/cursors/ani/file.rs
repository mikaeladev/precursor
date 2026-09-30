use std::io::{self, Read, Seek, Write};

use thiserror::Error;

use crate::ani::acon::{AconChunk, FromChunkError};
use crate::ani::header::AconHeader;
use crate::containers::riff::ChunkValue;
use crate::cur::CurFile;

#[derive(Debug, Error)]
pub enum ReadError {
  #[error("{0}")]
  IoError(#[from] io::Error),

  #[error("{0}")]
  FromChunkError(#[from] FromChunkError),
}

pub type ReadResult<T> = Result<T, ReadError>;

pub struct AniFile(AconChunk);

impl AniFile {
  /// Constructs a new `AconChunk`.
  ///
  /// # Panics
  ///
  /// Panics if there are more `icons` than indices in `sequence`.
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
  /// This method returns a [`ReadError`] that wraps the following:
  ///
  /// - [`ChunkValue::read`]
  /// - [`AconChunk::from_chunk`]
  pub fn read<R: Read>(reader: &mut R) -> ReadResult<Self> {
    let chunk = ChunkValue::read(reader)?;
    let acon = AconChunk::from_chunk(chunk)?;
    Ok(Self(acon))
  }

  /// Writes an ANI file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  pub fn write<W: Write + Seek>(self, writer: &mut W) -> io::Result<usize> {
    let chunk = self.into_chunk();
    chunk.write(writer)
  }

  // TODO
  pub fn into_frames(self) -> Vec<CurFile> {
    let Self(acon) = self;
    acon.frames
  }

  /// Converts this value into a [`ChunkValue`].
  pub(crate) fn into_chunk(self) -> ChunkValue {
    let Self(acon) = self;
    acon.into_chunk()
  }
}
