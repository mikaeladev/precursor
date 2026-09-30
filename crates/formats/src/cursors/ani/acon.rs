use std::io::Cursor;

use thiserror::Error;

use crate::ani::header::AconHeader;
use crate::containers::ico::ReadError;
use crate::containers::riff::{ChunkId, ChunkValue};
use crate::cur::CurFile;

pub trait ChunkIdAconExt {
  const ACON: ChunkId = ChunkId(*b"ACON");
  const ANIH: ChunkId = ChunkId(*b"anih");
  const RATE: ChunkId = ChunkId(*b"rate");
  const SEQ_: ChunkId = ChunkId(*b"seq ");
  const FRAM: ChunkId = ChunkId(*b"fram");
  const ICON: ChunkId = ChunkId(*b"icon");
}

impl ChunkIdAconExt for ChunkId {}

pub struct AconChunk {
  pub header: AconHeader,
  pub rates: Option<Vec<u32>>,
  pub sequence: Option<Vec<u32>>,
  pub frames: Vec<CurFile>,
}

// pub struct AconInfo {
//   title: Option<String>,
//   author: Option<String>,
// }

/// The error type for [`AconChunk::from_chunk`].
#[derive(Debug, Error)]
pub enum FromChunkError {
  /// Error reading an embedded ICO frame.
  #[error("{0}")]
  ReadError(#[from] ReadError),

  /// Chunk was not a [`Parent`][ChunkValue::Parent].
  #[error("not a parent chunk")]
  NotAParent,

  /// Chunk was not a [`Child`][ChunkValue::Child].
  #[error("not a child chunk")]
  NotAChild,

  /// Missing a specific chunk.
  #[error("missing \"{0}\" chunk")]
  MissingChunk(ChunkId),

  /// Expected a specific chunk and received another.
  #[error(r#"unexpected chunk; expected "{0}", received "{1}""#)]
  UnexpectedChunk(ChunkId, ChunkId),

  /// Expected a specific number of bytes and received more or less.
  #[error("invalid chunk length; expected {0} bytes, received {1}")]
  InvalidChunkLength(usize, usize),

  /// Header size field was not `36`.
  #[error("invalid header size field; expected 36, received {0}")]
  InvalidHeaderSize(u32),
}

pub type FromChunkResult<T> = Result<T, FromChunkError>;

impl AconChunk {
  /// Constructs a new `AconChunk` from a [`ChunkValue`].
  ///
  /// # Errors
  ///
  /// Fails with a [`FromChunkError`] in the following situations:
  ///
  /// - Chunk is not a [`Parent`][ChunkValue::Parent];
  /// - Chunk kind is not [`RIFF`];
  /// - Chunk ID is not [`ACON`];
  /// - Required chunks are missing ([`anih`], [`fram`]);
  /// - The [`anih`] chunk has the incorrect size;
  /// - The [`fram`] chunk has non-[`icon`] children;
  /// - Any [`icon`] chunk fails to decode.
  ///
  /// [`RIFF`]: ChunkId::RIFF
  /// [`ACON`]: ChunkId::ACON
  /// [`anih`]: ChunkId::ANIH
  /// [`fram`]: ChunkId::FRAM
  /// [`icon`]: ChunkId::ICON
  pub fn from_chunk(chunk: ChunkValue) -> FromChunkResult<Self> {
    match chunk {
      ChunkValue::Parent(kind, _, _) if kind != ChunkId::RIFF => {
        Err(FromChunkError::UnexpectedChunk(ChunkId::RIFF, kind))
      }
      ChunkValue::Parent(_, id, _) if id != ChunkId::ACON => {
        Err(FromChunkError::UnexpectedChunk(ChunkId::ACON, id))
      }
      ChunkValue::Parent(_, _, subchunks) => {
        let mut header_data = None;
        let mut rates_data = None;
        let mut sequence_data = None;
        let mut frame_chunks = None;

        for subchunk in subchunks {
          match subchunk {
            ChunkValue::Parent(kind, _, _) if kind != ChunkId::LIST => {
              return Err(FromChunkError::UnexpectedChunk(ChunkId::LIST, kind));
            }
            ChunkValue::Parent(_, id, subchunks) => match id {
              ChunkId::FRAM => frame_chunks = Some(subchunks),
              _ => continue,
            },
            ChunkValue::Child(id, data) => match id {
              ChunkId::ANIH => header_data = Some(data),
              ChunkId::RATE => rates_data = Some(data),
              ChunkId::SEQ_ => sequence_data = Some(data),
              _ => continue,
            },
          }
        }

        let header = match header_data {
          Some(bytes) => Ok(AconHeader::from_bytes(&bytes)?),
          None => Err(FromChunkError::MissingChunk(ChunkId::ANIH)),
        }?;

        let rates = match rates_data {
          None => None,
          Some(bytes) => {
            Some(vec8_to_vec32(header.step_count as usize, bytes)?)
          }
        };

        let sequence = match sequence_data {
          None => None,
          Some(bytes) => {
            Some(vec8_to_vec32(header.step_count as usize, bytes)?)
          }
        };

        let frame_chunks =
          frame_chunks.ok_or(FromChunkError::MissingChunk(ChunkId::FRAM))?;

        let mut frames = Vec::with_capacity(frame_chunks.len());

        for chunk in frame_chunks {
          match chunk {
            ChunkValue::Parent(_, _, _) => {
              return Err(FromChunkError::NotAChild);
            }
            ChunkValue::Child(id, _) if id != ChunkId::ICON => {
              return Err(FromChunkError::UnexpectedChunk(ChunkId::ICON, id));
            }
            ChunkValue::Child(_, data) => {
              frames.push(CurFile::read(&mut Cursor::new(data))?);
            }
          }
        }

        Ok(Self {
          header,
          rates,
          sequence,
          frames,
        })
      }
      _ => Err(FromChunkError::NotAParent),
    }
  }

  /// Converts this value into a [`ChunkValue`].
  pub fn into_chunk(self) -> ChunkValue {
    let mut chunks = Vec::with_capacity(
      2 + (if self.rates.is_some() { 1 } else { 0 })
        + (if self.sequence.is_some() { 1 } else { 0 }),
    );

    chunks.push(self.header.into_chunk());

    if let Some(rates) = self.rates {
      chunks.push(ChunkValue::Child(ChunkId::RATE, vec32_to_vec8(rates)));
    }
    if let Some(sequence) = self.sequence {
      chunks.push(ChunkValue::Child(ChunkId::SEQ_, vec32_to_vec8(sequence)));
    }

    chunks.push(frames_chunk(self.frames));

    ChunkValue::Parent(ChunkId::RIFF, ChunkId(*b"ACON"), chunks)
  }
}

fn vec8_to_vec32(length: usize, vec: Vec<u8>) -> FromChunkResult<Vec<u32>> {
  const N: usize = size_of::<u32>();

  if let expected_len = length * N
    && let actual_len = vec.len()
    && actual_len != expected_len
  {
    return Err(FromChunkError::InvalidChunkLength(expected_len, actual_len));
  }

  let (chunks, []) = vec.as_chunks::<N>() else {
    unreachable!()
  };

  Ok(
    chunks
      .into_iter()
      .map(|slice| u32::from_le_bytes(*slice))
      .collect(),
  )
}

fn vec32_to_vec8(vec: Vec<u32>) -> Vec<u8> {
  vec.into_iter().flat_map(|int| int.to_le_bytes()).collect()
}

fn frames_chunk(icons: Vec<CurFile>) -> ChunkValue {
  ChunkValue::Parent(
    ChunkId::LIST,
    ChunkId::FRAM,
    icons
      .into_iter()
      .map(|icon| {
        let mut buffer = Vec::with_capacity(icon.exact_size());
        icon.write(&mut buffer).unwrap(); // writing to memory, should be fine?

        ChunkValue::Child(ChunkId::ICON, buffer)
      })
      .collect(),
  )
}
