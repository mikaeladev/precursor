use std::io::{self, Write};

use precursor_config::Theme;
use precursor_error::{ErrorMessage, Result};

use crate::macros::action_failed;

// TODO: doc
pub fn write_theme_file(writer: &mut impl Write, theme: Theme) -> Result<()> {
  let write_file = || -> io::Result<()> {
    writeln!(writer, "[Icon Theme]")?;

    writeln!(writer, "Name={}", theme.name)?;
    if let Some(value) = &theme.locales {
      for (locale, config) in value {
        if let Some(value) = &config.name {
          writeln!(writer, "Name[{locale}]={value}")?;
        }
      }
    }

    writeln!(writer, "Comment={}", theme.comment)?;
    if let Some(value) = &theme.locales {
      for (locale, config) in value {
        if let Some(value) = &config.comment {
          writeln!(writer, "Comment[{locale}]={value}")?;
        }
      }
    }

    if let Some(value) = theme.example {
      writeln!(writer, "Example={value}")?;
    }

    if let Some(value) = theme.hidden {
      writeln!(writer, "Hidden={value}")?;
    }

    Ok(())
  };

  write_file().map_err(|src| {
    action_failed!("write the theme index")
      .with_subheading(src.kind())
      .into_error()
      .with_source(src)
  })
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;

  use precursor_config::ThemeLocaleValue;

  use super::*;

  #[test]
  fn write_minimal() {
    let config = Theme {
      name: String::from("name"),
      comment: String::from("comment"),
      example: None,
      hidden: None,
      locales: None,
    };

    let mut raw_value = Vec::new();

    write_theme_file(&mut raw_value, config).unwrap();

    let raw_expected = r#"[Icon Theme]
Name=name
Comment=comment
"#;

    let value = String::from_utf8(raw_value).unwrap();
    let expected = String::from(raw_expected);

    assert_eq!(value, expected);
  }

  #[test]
  fn write_full() {
    let theme = Theme {
      name: String::from("name"),
      comment: String::from("comment"),
      example: Some(String::from("example")),
      hidden: Some(false),
      locales: Some(BTreeMap::from([
        (
          String::from("en_US"),
          ThemeLocaleValue {
            name: Some(String::from("foo")),
            comment: Some(String::from("bar")),
          },
        ),
        (
          String::from("en_GB"),
          ThemeLocaleValue {
            name: Some(String::from("baz")),
            comment: Some(String::from("qux")),
          },
        ),
      ])),
    };

    let mut raw_value = Vec::new();

    write_theme_file(&mut raw_value, theme).unwrap();

    let raw_expected = r#"[Icon Theme]
Name=name
Name[en_GB]=baz
Name[en_US]=foo
Comment=comment
Comment[en_GB]=qux
Comment[en_US]=bar
Example=example
Hidden=false
"#;

    let value = String::from_utf8(raw_value).unwrap();
    let expected = String::from(raw_expected);

    assert_eq!(value, expected);
  }
}
