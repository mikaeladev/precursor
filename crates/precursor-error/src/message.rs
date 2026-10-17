use std::fmt::{self, Debug, Display, Formatter};

/// Any static type that implements [`Display`] and [`Debug`].
pub trait ErrorMessage: Display + Debug + 'static {}
impl<T: Display + Debug + 'static> ErrorMessage for T {}

#[derive(Debug)]
pub struct ErrorMessageWithAttrs {
  pub message: Box<dyn ErrorMessage>,
  pub attrs: Vec<(String, Box<dyn ErrorMessage>)>,
}

impl Display for ErrorMessageWithAttrs {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}", self.message)?;

    let attrs_len = self.attrs.len();

    let longest_key_len = self.attrs.iter().fold(0, |acc, (key, _)| {
      let len = key.len();
      if len > acc { len } else { acc }
    });

    for (i, (key, value)) in self.attrs.iter().enumerate() {
      let is_last = i != (attrs_len - 1);

      let line = if is_last { '├' } else { '└' };
      let term = if is_last { "\n" } else { "" };

      let space = str::repeat(" ", (longest_key_len - key.len()) + 1);

      write!(f, "{line}▸ {key}:{space}{value}{term}")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use super::*;

  #[test]
  fn display_single() {
    let value = ErrorMessageWithAttrs {
      message: Box::new("hello world"),
      attrs: vec![("foo".to_string(), Box::new("bar"))],
    };

    let expected = r#"
hello world
└▸ foo: bar
"#;

    assert_eq!(value.to_string(), expected.trim())
  }

  #[test]
  fn display_multiple() {
    let value = ErrorMessageWithAttrs {
      message: Box::new("hello world"),
      attrs: vec![
        ("foo".to_string(), Box::new("bar")),
        (
          "path".to_string(),
          Box::new(PathBuf::from("./foo/bar.txt").display().to_string()),
        ),
      ],
    };

    let expected = r#"
hello world
├▸ foo:  bar
└▸ path: ./foo/bar.txt
"#;

    assert_eq!(value.to_string(), expected.trim())
  }
}
