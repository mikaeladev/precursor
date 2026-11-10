use precursor_error::{Error, ErrorMessage};
use precursor_ico as ico;
use precursor_riff::ChunkId;

use thiserror::Error;

/// The error type for [`AconChunk::from_chunk`].
///
/// [`AconChunk::from_chunk`]: crate::acon::AconChunk::from_chunk
#[derive(Debug, Error)]
pub enum FromChunkError {
  /// Error reading an embedded ICO frame.
  #[error("failed to read ICO frame")]
  ReadError(#[from] ico::ReadError),

  /// Chunk was not a [`Parent`][ChunkValue::Parent].
  #[error("not a parent chunk")]
  NotAParent,

  /// Chunk was not a [`Child`][ChunkValue::Child].
  #[error("not a child chunk")]
  NotAChild,

  /// Missing a specific chunk.
  #[error("missing '{0}' chunk")]
  MissingChunk(ChunkId),

  /// Expected a specific chunk and received another.
  #[error("expected '{0}' chunk, received '{1}'")]
  UnexpectedChunk(ChunkId, ChunkId),

  /// Expected a chunk to be a certain size.
  #[error("expected '{0}' chunk to be {1} bytes long, was {2}")]
  WrongSize(ChunkId, usize, usize),

  /// Expected the size field of a chunk to be a certain value.
  #[error("expected size field of '{0}' chunk to be {1}, was {2}")]
  WrongSizeField(ChunkId, u32, u32),
}

pub type FromChunkResult<T> = Result<T, FromChunkError>;

impl From<FromChunkError> for Error {
  fn from(value: FromChunkError) -> Self {
    let err = ErrorMessage::new("failed to read ACON chunk")
      .with_subheading(&value)
      .into_error();

    if let FromChunkError::ReadError(inner) = value
      && let ico::ReadError::IoError(src) = inner
    {
      err.with_source(src)
    } else {
      err
    }
  }
}
