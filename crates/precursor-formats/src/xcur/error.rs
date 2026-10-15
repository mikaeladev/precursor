use std::io;
use std::string::FromUtf8Error;

use thiserror::Error;

use super::chunk::XcursorChunk;
use super::file::XcursorFile;

const FILE_MAGIC: &[u8] = XcursorFile::MAGIC;
const FILE_HEADER_SIZE: usize = XcursorFile::HEADER_SIZE;
const FILE_VERSION: u32 = XcursorFile::VERSION;

const CHUNK_VERSION: u32 = XcursorChunk::VERSION;

const COMMENT_HEADER_SIZE: usize = XcursorChunk::COMMENT_HEADER_SIZE;
const COMMENT_TYPE: u32 = XcursorChunk::COMMENT_TYPE;

const IMAGE_HEADER_SIZE: usize = XcursorChunk::IMAGE_HEADER_SIZE;
const IMAGE_TYPE: u32 = XcursorChunk::IMAGE_TYPE;

#[derive(Debug, Error)]
pub enum ReadError {
  #[error("{0}")]
  IoError(#[from] io::Error),

  #[error("invalid comment value; {0}")]
  FromUtf8Error(#[from] FromUtf8Error),

  #[error(
    "invalid file header size; expected {FILE_HEADER_SIZE}, received {0}"
  )]
  InvalidFileHeaderSize(usize),

  #[error(
    "invalid comment header size; expected {COMMENT_HEADER_SIZE}, received {0}"
  )]
  InvalidCommentHeaderSize(usize),

  #[error(
    "invalid image header size; expected {IMAGE_HEADER_SIZE}, received {0}"
  )]
  InvalidImageHeaderSize(usize),

  #[error(
    "missing magic number; expected first four bytes to be {FILE_MAGIC:?}, received {0:?}"
  )]
  MissingMagicNumber([u8; 4]),

  #[error(
    "unrecognised chunk type; expected either of {COMMENT_TYPE:#x} or {IMAGE_TYPE:#x}, received {0:#x}"
  )]
  UnrecognisedChunkType(u32),

  #[error(
    "unrecognised comment type; expected either of 1 or 2 or 3, received {0}"
  )]
  UnrecognisedCommentType(u32),

  #[error(
    "unsupported file version; expected version {FILE_VERSION:#x}, received {0:#x}"
  )]
  UnsupportedFileVersion(u32),

  #[error(
    "unsupported chunk version; expected version {CHUNK_VERSION}, received {0}"
  )]
  UnsupportedChunkVersion(u32),
}

pub type ReadResult<T> = Result<T, ReadError>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn io_error() {
    fn inner_err() -> io::Error {
      io::Error::new(io::ErrorKind::Unsupported, "unsupported")
    }

    assert_eq!(
      ReadError::IoError(inner_err()).to_string(),
      inner_err().to_string()
    )
  }

  #[test]
  fn from_utf8_error() {
    let inner_err = String::from_utf8(vec![0xC0]).unwrap_err();

    assert_eq!(
      ReadError::FromUtf8Error(inner_err.clone()).to_string(),
      format!("invalid comment value; {inner_err}")
    )
  }

  #[test]
  fn invalid_file_header_size() {
    assert_eq!(
      ReadError::InvalidFileHeaderSize(12).to_string(),
      "invalid file header size; expected 16, received 12"
    )
  }

  #[test]
  fn invalid_comment_header_size() {
    assert_eq!(
      ReadError::InvalidCommentHeaderSize(16).to_string(),
      "invalid comment header size; expected 20, received 16"
    )
  }

  #[test]
  fn invalid_image_header_size() {
    assert_eq!(
      ReadError::InvalidImageHeaderSize(32).to_string(),
      "invalid image header size; expected 36, received 32"
    )
  }

  #[test]
  fn missing_magic_number() {
    assert_eq!(
      ReadError::MissingMagicNumber([1, 2, 3, 4]).to_string(),
      "missing magic number; expected first four bytes to be [88, 99, 117, 114], received [1, 2, 3, 4]"
    )
  }

  #[test]
  fn unrecognised_chunk_type() {
    assert_eq!(
      ReadError::UnrecognisedChunkType(0xfffc0003).to_string(),
      "unrecognised chunk type; expected either of 0xfffe0001 or 0xfffd0002, received 0xfffc0003"
    )
  }

  #[test]
  fn unrecognised_comment_type() {
    assert_eq!(
      ReadError::UnrecognisedCommentType(4).to_string(),
      "unrecognised comment type; expected either of 1 or 2 or 3, received 4"
    )
  }

  #[test]
  fn unsupported_file_version() {
    assert_eq!(
      ReadError::UnsupportedFileVersion(1).to_string(),
      "unsupported file version; expected version 0x10000, received 0x1"
    )
  }

  #[test]
  fn unsupported_chunk_version() {
    assert_eq!(
      ReadError::UnsupportedChunkVersion(2).to_string(),
      "unsupported chunk version; expected version 1, received 2"
    )
  }
}
