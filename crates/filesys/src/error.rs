use std::fmt::{self, Debug, Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ErrorMessageWithPath {
  message: String,
  path: PathBuf,
}

impl ErrorMessageWithPath {
  /// Constructs a new `ErrorMessageWithPath`.
  pub fn new(message: impl Into<String>, path: impl Into<PathBuf>) -> Self {
    Self {
      message: message.into(),
      path: path.into(),
    }
  }
}

impl Display for ErrorMessageWithPath {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}\n↳ path: {}", self.message, self.path.display())
  }
}

impl From<ErrorMessageWithPath> for String {
  fn from(value: ErrorMessageWithPath) -> Self {
    value.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn message_with_path() {
    let message =
      ErrorMessageWithPath::new("hello world", PathBuf::from("foo/bar.txt"));

    let expected = r#"
hello world;
↳ path: foo/bar.txt
"#;

    assert_eq!(message.to_string(), expected.trim())
  }
}
