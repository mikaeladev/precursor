use std::{error, result};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct Error {
  message: String,
  source: Option<Box<dyn error::Error + 'static>>,
}

impl Error {
  /// Constructs a new `Error`.
  pub fn new<S: Into<Box<dyn error::Error>>>(
    message: impl Into<String>,
    source: impl Into<Option<S>>,
  ) -> Self {
    Self {
      message: message.into(),
      source: source.into().and_then(|err| Some(err.into())),
    }
  }

  /// Returns a reference to the source error wrapped by this error (if any).
  pub fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    self.source.as_deref()
  }

  /// Returns `Some` reference to the source error if it is of type `T`, or
  /// `None` if it isn’t (if any).
  pub fn downcast<T: error::Error + 'static>(&self) -> Option<&T> {
    self.source.as_ref().and_then(|src| src.downcast_ref())
  }

  /// Consumes the `Error`, returning its source error (if any).
  pub fn into_source(self) -> Option<Box<dyn error::Error + 'static>> {
    self.source
  }
}

pub type Result<T> = result::Result<T, Error>;
