use std::io::Cursor;

use precursor_cur::CurFile;
use precursor_riff::{ChunkId, ChunkValue};

use crate::error::{FromChunkError, FromChunkResult};
use crate::header::AconHeader;

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

impl AconChunk {
  /// Constructs a new `AconChunk` from a [`ChunkValue`].
  ///
  /// # Errors
  ///
  /// Fails with a [`FromChunkError`] if the chunk is not valid [`ACON`].
  ///
  /// [`ACON`]: ChunkId::ACON
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
          Some(bytes) => Some(vec8_to_vec32(
            ChunkId::RATE,
            header.step_count as usize,
            bytes,
          )?),
        };

        let sequence = match sequence_data {
          None => None,
          Some(bytes) => Some(vec8_to_vec32(
            ChunkId::SEQ_,
            header.step_count as usize,
            bytes,
          )?),
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

  /// Converts this type into a [`ChunkValue`].
  pub fn into_chunk(self) -> ChunkValue {
    let mut chunks = Vec::with_capacity(self.calc_subchunks_capacity());

    chunks.push(self.header.into_chunk());

    if let Some(rates) = self.rates {
      chunks.push(ChunkValue::Child(ChunkId::RATE, vec32_to_vec8(rates)));
    }

    if let Some(sequence) = self.sequence {
      chunks.push(ChunkValue::Child(ChunkId::SEQ_, vec32_to_vec8(sequence)));
    }

    chunks.push(make_frames_chunk(self.frames));

    ChunkValue::Parent(ChunkId::RIFF, ChunkId(*b"ACON"), chunks)
  }

  /// Calculates the capacity for subchunks needed to convert this into a
  /// [`ChunkValue`].
  ///
  /// As the [`anih`] and [`fram`] subchunks are required in any [`ACON`] chunk,
  /// this will always return at least `2`.
  ///
  /// [`anih`]: ChunkId::ANIH
  /// [`fram`]: ChunkId::FRAM
  /// [`ACON`]: ChunkId::ACON
  const fn calc_subchunks_capacity(&self) -> usize {
    2 + (if self.rates.is_some() { 1 } else { 0 })
      + (if self.sequence.is_some() { 1 } else { 0 })
  }
}

fn vec8_to_vec32(
  chunk_id: ChunkId,
  length: usize,
  vec: Vec<u8>,
) -> FromChunkResult<Vec<u32>> {
  const N: usize = size_of::<u32>();

  if let expected_len = length * N
    && let actual_len = vec.len()
    && actual_len != expected_len
  {
    return Err(FromChunkError::WrongSize(
      chunk_id,
      expected_len,
      actual_len,
    ));
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

fn make_frames_chunk(icons: Vec<CurFile>) -> ChunkValue {
  let f = |file: CurFile| {
    let mut buf = Vec::with_capacity(file.exact_size());
    file.write(&mut buf).expect("exact size should be accurate");

    ChunkValue::Child(ChunkId::ICON, buf)
  };

  ChunkValue::Parent(
    ChunkId::LIST,
    ChunkId::FRAM,
    icons.into_iter().map(f).collect(),
  )
}
