use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

use crate::ErrorMessage;

#[derive(Debug)]
pub struct Error {
  pub(crate) message: ErrorMessage,
  pub(crate) source: Option<Box<dyn StdError + 'static>>,
}

impl Error {
  /// Constructs a new `Error` from `message`.
  ///
  /// All other attributes are set to their defaults and must be configured with
  /// their respective setters.
  pub const fn new(message: ErrorMessage) -> Self {
    Self {
      message,
      source: None,
    }
  }

  /// Returns itself with the source set to the post-conversion value.
  pub fn with_source<E: StdError + 'static>(
    mut self,
    value: impl Into<Option<E>>,
  ) -> Self {
    let source = value
      .into()
      .and_then(|err| Some(Box::new(err) as Box<dyn StdError>));
    self.source = source;
    self
  }

  /// Returns [`Some`] reference to the source error if it exists and is of type
  /// `T`, or [`None`] otherwise.
  pub fn downcast<T: StdError + 'static>(&self) -> Option<&T> {
    self.source().and_then(|src| src.downcast_ref())
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    self.source.as_deref()
  }
}

pub type Result<T> = std::result::Result<T, Error>;
