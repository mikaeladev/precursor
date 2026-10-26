use std::fmt::{self, Debug, Display, Formatter};

use crate::Error;

#[derive(Debug, Default, Clone)]
pub struct ErrorMessage {
  heading: String,
  subheading: Option<String>,
  help: Option<String>,
  context: Vec<(String, String)>,
}

impl ErrorMessage {
  /// Constructs a new `ErrorMessage` from `heading`.
  ///
  /// All other attributes are set to their defaults and must be configured with
  /// their respective setters.
  pub fn new(heading: impl ToString) -> Self {
    Self {
      heading: heading.to_string(),
      ..Default::default()
    }
  }

  /// Returns itself with the subheading set to the post-conversion value.
  ///
  /// # Examples
  ///
  /// This method can be chained together, like so:
  ///
  /// ```
  /// # use precursor_error::ErrorMessage;
  /// #
  /// ErrorMessage::new("io error")
  ///   .with_subheading("file not found") // this method !!
  ///   .with_help("make sure it exists first")
  ///   .with_context([("path", "./foo/bar")]);
  /// ```
  pub fn with_subheading<T: ToString>(
    mut self,
    value: impl Into<Option<T>>,
  ) -> Self {
    let subheading = value.into().and_then(|v| Some(v.to_string()));
    self.subheading = subheading;
    self
  }

  /// Returns itself with the help message set to the post-conversion value.
  ///
  /// # Examples
  ///
  /// This method can be chained together, like so:
  ///
  /// ```
  /// # use precursor_error::ErrorMessage;
  /// #
  /// ErrorMessage::new("io error")
  ///   .with_subheading("file not found")
  ///   .with_help("make sure it exists first") // this method !!
  ///   .with_context([("path", "./foo/bar")]);
  /// ```
  pub fn with_help<T: ToString>(mut self, value: impl Into<Option<T>>) -> Self {
    let help = value.into().and_then(|v| Some(v.to_string()));
    self.help = help;
    self
  }

  /// Returns itself with the context set to the post-conversion value.
  ///
  /// # Examples
  ///
  /// This method can be chained together, like so:
  ///
  /// ```
  /// # use precursor_error::ErrorMessage;
  /// #
  /// ErrorMessage::new("io error")
  ///   .with_subheading("file not found")
  ///   .with_help("make sure it exists first")
  ///   .with_context([("path", "./foo/bar")]); // this method !!
  /// ```
  pub fn with_context<const N: usize, K: ToString, V: ToString>(
    mut self,
    value: [(K, V); N],
  ) -> Self {
    let context = value
      .into_iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect();
    self.context = context;
    self
  }

  /// Converts this type into an [`Error`].
  pub const fn into_error(self) -> Error {
    Error {
      message: self,
      source: None,
    }
  }

  /// Returns a reference to the heading string.
  pub const fn heading(&self) -> &str {
    self.heading.as_str()
  }

  /// Returns some reference to the subheading.
  pub const fn subheading(&self) -> Option<&str> {
    match &self.subheading {
      Some(subheading) => Some(subheading.as_str()),
      None => None,
    }
  }

  /// Returns some reference to the help message.
  pub const fn help(&self) -> Option<&str> {
    match &self.help {
      Some(help) => Some(help.as_str()),
      None => None,
    }
  }

  /// Returns a slice containing the entire vector of context entries.
  pub const fn context(&self) -> &[(String, String)] {
    self.context.as_slice()
  }
}

impl Display for ErrorMessage {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.heading)?;

    if let Some(subheading) = &self.subheading {
      write!(f, "; {subheading}")?;
    }

    if !self.context.is_empty() {
      let context_len = self.context.len();

      let longest_key_len = self.context.iter().fold(0, |acc, (key, _)| {
        let len = key.len();
        if len > acc { len } else { acc }
      });

      for (i, (key, value)) in self.context.iter().enumerate() {
        write!(f, "\n")?;

        let line = if i != (context_len - 1) { '├' } else { '└' };
        let spacing = str::repeat(" ", (longest_key_len - key.len()) + 1);

        write!(f, "{line}▸ {key}:{spacing}{value}")?;
      }
    }

    if let Some(help) = &self.help {
      write!(f, "\n\nhelp: {help}")?;
    }

    Ok(())
  }
}

impl From<ErrorMessage> for Error {
  fn from(value: ErrorMessage) -> Self {
    value.into_error()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display_min() {
    let msg = ErrorMessage::new("teeny tiiny error");

    assert_eq!(msg.to_string(), msg.heading())
  }

  #[test]
  fn display_max() {
    let msg = ErrorMessage::new("extra extra")
      .with_subheading("read all about it")
      .with_help("try again?")
      .with_context([("foo", "bar"), ("lorem", "ipsum")]);

    let expected = r#"
extra extra; read all about it
├▸ foo:   bar
└▸ lorem: ipsum

help: try again?
"#;

    assert_eq!(msg.to_string(), expected.trim())
  }
}
