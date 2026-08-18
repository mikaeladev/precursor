use std::convert::Infallible;
use std::fmt::{Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::de::{Error as DeError, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq)]
pub enum PlatformAlias {
  Global(String),
  Linux(String),
  Macos(String),
  Windows(String),
}

impl FromStr for PlatformAlias {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(s) = s.strip_prefix("linux:") {
      Ok(Self::Linux(s.to_string()))
    } else if let Some(s) = s.strip_prefix("macos:") {
      Ok(Self::Macos(s.to_string()))
    } else if let Some(s) = s.strip_prefix("windows:") {
      Ok(Self::Windows(s.to_string()))
    } else {
      Ok(Self::Global(s.to_string()))
    }
  }
}

impl<'de> Deserialize<'de> for PlatformAlias {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct PlatformAliasVisitor;

    impl<'de> Visitor<'de> for PlatformAliasVisitor {
      type Value = PlatformAlias;

      fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("alias string")
      }

      fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        Ok(PlatformAlias::from_str(value).unwrap())
      }
    }

    de.deserialize_str(PlatformAliasVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_platform_alias_from_str() {
    assert_eq!(
      PlatformAlias::from_str("test"),
      Ok(PlatformAlias::Global("test".into())),
    );
    assert_eq!(
      PlatformAlias::from_str("linux:test"),
      Ok(PlatformAlias::Linux("test".into())),
    );
    assert_eq!(
      PlatformAlias::from_str("macos:test"),
      Ok(PlatformAlias::Macos("test".into())),
    );
    assert_eq!(
      PlatformAlias::from_str("windows:test"),
      Ok(PlatformAlias::Windows("test".into())),
    );
  }

  #[test]
  fn test_platform_alias_deserialize() {
    use toml::Value;

    let exprs = [
      (r#""test""#, PlatformAlias::Global("test".into())),
      (r#""linux:test""#, PlatformAlias::Linux("test".into())),
      (r#""macos:test""#, PlatformAlias::Macos("test".into())),
      (r#""windows:test""#, PlatformAlias::Windows("test".into())),
    ];

    for (raw_value, expected) in exprs {
      let de = raw_value.parse::<Value>().unwrap();
      let value = PlatformAlias::deserialize(de).unwrap();

      assert_eq!(value, expected);
    }
  }
}
