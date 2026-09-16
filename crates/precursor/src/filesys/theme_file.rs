use std::io::{self, Write};

use crate_config::PackageConfig;

pub fn write_icon_theme_index(
  writer: &mut impl Write,
  config: PackageConfig,
) -> io::Result<()> {
  writeln!(writer, "[Icon Theme]")?;

  writeln!(writer, "Name={}", config.name)?;
  if let Some(value) = &config.linux.locales {
    for (locale, config) in value {
      if let Some(value) = &config.name {
        writeln!(writer, "Name[{locale}]={value}")?;
      }
    }
  }

  writeln!(writer, "Comment={}", config.linux.comment)?;
  if let Some(value) = &config.linux.locales {
    for (locale, config) in value {
      if let Some(value) = &config.comment {
        writeln!(writer, "Comment[{locale}]={value}")?;
      }
    }
  }

  if let Some(value) = config.linux.example {
    writeln!(writer, "Example={value}")?;
  }

  if let Some(value) = config.linux.hidden {
    writeln!(writer, "Hidden={value}")?;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;

  use crate_config::{LinuxPackageConfig, PackageLocaleConfig};

  use super::*;

  #[test]
  fn write_minimal() {
    let config = PackageConfig {
      name: String::from("name"),
      linux: LinuxPackageConfig {
        comment: String::from("comment"),
        example: None,
        hidden: None,
        locales: None,
      },
    };

    let mut raw_value = Vec::new();

    write_icon_theme_index(&mut raw_value, config).unwrap();

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
    let config = PackageConfig {
      name: String::from("name"),
      linux: LinuxPackageConfig {
        comment: String::from("comment"),
        example: Some(String::from("example")),
        hidden: Some(false),
        locales: Some(BTreeMap::from([
          (
            String::from("en_US"),
            PackageLocaleConfig {
              name: Some(String::from("foo")),
              comment: Some(String::from("bar")),
            },
          ),
          (
            String::from("en_GB"),
            PackageLocaleConfig {
              name: Some(String::from("baz")),
              comment: Some(String::from("qux")),
            },
          ),
        ])),
      },
    };

    let mut raw_value = Vec::new();

    write_icon_theme_index(&mut raw_value, config).unwrap();

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
