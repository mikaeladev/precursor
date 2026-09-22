use std::io;

use thiserror::Error;

/// The error type for ICO reads.
#[derive(Debug, Error)]
pub enum ReadError {
  #[error("missing reserved byte in file header")]
  HeaderMissingReserved,

  #[error("expected resource type to be {0}, was {1}")]
  InvalidResourceType(u16, u16),

  #[error("missing reserved byte in entry {0}")]
  EntryMissingReserved(u16),

  #[error("{0}")]
  IoError(#[from] io::Error),
}

pub type ReadResult<T> = Result<T, ReadError>;
