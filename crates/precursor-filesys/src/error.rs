use std::fmt::{self, Debug, Display, Formatter};
use std::path::PathBuf;

use precursor_error::ErrorMessageWithAttrs;

#[derive(Debug)]
pub struct ErrorMessageWithPath(ErrorMessageWithAttrs);

impl ErrorMessageWithPath {
  /// Constructs a new `ErrorMessageWithPath`.
  pub fn new(message: impl Into<String>, path: impl Into<PathBuf>) -> Self {
    Self(ErrorMessageWithAttrs {
      message: Box::new(message.into()),
      attrs: vec![(
        "path".to_string(),
        Box::new(path.into().display().to_string()),
      )],
    })
  }
}

impl Display for ErrorMessageWithPath {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.0, f)
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
hello world
└▸ path: foo/bar.txt
"#;

    assert_eq!(message.to_string(), expected.trim())
  }
}
