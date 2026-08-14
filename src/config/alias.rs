use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::de::{Error as DeError, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq)]
pub enum PlatformAlias {
  Linux(String),
  Macos(String),
  Windows(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePlatformAliasError;

const ERROR_MSG: &str =
  "string prepended by either 'linux:', 'macos:', or 'windows:'";

impl StdError for ParsePlatformAliasError {}

impl Display for ParsePlatformAliasError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    formatter.write_str(&format!("expected a {ERROR_MSG}"))
  }
}

impl FromStr for PlatformAlias {
  type Err = ParsePlatformAliasError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(s) = s.strip_prefix("linux:") {
      Ok(Self::Linux(s.to_string()))
    } else if let Some(s) = s.strip_prefix("macos:") {
      Ok(Self::Macos(s.to_string()))
    } else if let Some(s) = s.strip_prefix("windows:") {
      Ok(Self::Windows(s.to_string()))
    } else {
      Err(ParsePlatformAliasError)
    }
  }
}

impl<'de> Deserialize<'de> for PlatformAlias {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct PlatformAliasVisitor;

    impl<'de> Visitor<'de> for PlatformAliasVisitor {
      type Value = PlatformAlias;

      fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str(ERROR_MSG)
      }

      fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        PlatformAlias::from_str(value).or_else(|err| Err(E::custom(err)))
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
    assert_eq!(
      PlatformAlias::from_str("test"),
      Err(ParsePlatformAliasError)
    );
  }
}
