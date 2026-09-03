use thiserror::Error;

#[derive(Debug, Error)]
pub enum PixmapError {
  #[error("wrong dimensions, expected {0} pixels got {1}")]
  WrongDimensions(usize, usize),
}
