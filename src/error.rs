use std::fmt::{Display, Formatter};

pub use std::error::Error as StdError;
pub use std::fmt::{Error as FmtError, Result as FmtResult};
pub use std::io::{Error as IoError, ErrorKind as IoErrorKind};

pub use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum Error {
  Io(IoError),
  Fmt(FmtError),
  Toml(TomlError),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str(&match self {
      Self::Io(err) => format!("OS Error: {err}"),
      Self::Fmt(err) => format!("Error formatting string: {err}"),
      Self::Toml(err) => format!("Error parsing config: {err}"),
    })
  }
}

impl StdError for Error {}

impl From<IoError> for Error {
  fn from(value: IoError) -> Self {
    Self::Io(value)
  }
}

impl From<IoErrorKind> for Error {
  fn from(value: IoErrorKind) -> Self {
    Self::Io(IoError::from(value))
  }
}

impl From<FmtError> for Error {
  fn from(value: FmtError) -> Self {
    Self::Fmt(value)
  }
}

impl From<TomlError> for Error {
  fn from(value: TomlError) -> Self {
    Self::Toml(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_fmt() {
    // TODO: more tests
    assert_eq!(
      Error::from(IoErrorKind::NotADirectory).to_string(),
      "OS Error: not a directory",
    );
  }
}
