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
pub struct PlatformAliasError;

const ERROR_MSG: &str =
  "string prepended by either 'linux:', 'macos:', or 'windows:'";

impl StdError for PlatformAliasError {}

impl Display for PlatformAliasError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    formatter.write_str(&format!("expected a {ERROR_MSG}"))
  }
}

impl FromStr for PlatformAlias {
  type Err = PlatformAliasError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("linux:") {
      Ok(Self::Linux(s.to_string()))
    } else if s.starts_with("macos:") {
      Ok(Self::Macos(s.to_string()))
    } else if s.starts_with("windows:") {
      Ok(Self::Windows(s.to_string()))
    } else {
      Err(PlatformAliasError)
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

    const VARIANTS: &[&str] = &["Linux", "Macos", "Windows"];

    de.deserialize_enum("PlatformAlias", VARIANTS, PlatformAliasVisitor)
  }
}
