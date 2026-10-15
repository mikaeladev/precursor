use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};

pub trait ErrorMessage: Display + Debug + 'static {}
impl<T: Display + Debug + 'static> ErrorMessage for T {}

#[derive(Debug)]
pub struct Error {
  message: Box<dyn ErrorMessage>,
  source: Option<Box<dyn StdError + 'static>>,
}

impl Error {
  /// Constructs a new `Error`.
  pub fn new<E: StdError + 'static>(
    message: impl ErrorMessage,
    source: impl Into<Option<E>>,
  ) -> Self {
    Self {
      message: Box::new(message) as Box<dyn ErrorMessage>,
      source: source
        .into()
        .and_then(|err| Some(Box::new(err) as Box<dyn StdError>)),
    }
  }

  /// Returns `Some` reference to the source error if it is of type `T`, or
  /// `None` if it isn't (if any).
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
