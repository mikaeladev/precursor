use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
  /// Any kind of underlying error from the [PNG crate][png].
  #[error("failed to decode PNG ({0})")]
  DecodingError(#[from] png::DecodingError),

  /// Frame buffer would've exceeded `isize::MAX`.
  #[error("frame is too big")]
  FrameTooBig,

  /// Image was an APNG, which is unsupported.
  #[error("animated PNGs are unsupported")]
  IsAnimated,
}

pub type ReadResult<T> = Result<T, ReadError>;

#[derive(Debug, Error)]
#[error("failed to encode PNG ({0})")]
pub struct WriteError(#[from] png::EncodingError);

pub type WriteResult<T> = Result<T, WriteError>;
