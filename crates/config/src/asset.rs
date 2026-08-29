use std::fmt::{Formatter, Result as FmtResult};
use std::path::PathBuf;

use serde::de::{Error as DeError, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

#[derive(Debug, PartialEq, Eq)]
pub enum AssetValue {
  Short(PathBuf),
  Verbose {
    path: PathBuf,
    flip: Option<bool>,
    flop: Option<bool>,
    rotate: Option<RotateValue>,
  },
}

impl AssetValue {
  /// Returns a reference to the underlying `PathBuf`.
  pub const fn path(&self) -> &PathBuf {
    match self {
      Self::Short(path) => path,
      Self::Verbose { path, .. } => path,
    }
  }
}

#[derive(Debug, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RotateValue {
  Ninety = 90,
  OneEighty = 180,
  TwoSeventy = 270,
}

impl<'de> Deserialize<'de> for AssetValue {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct ValueVisitor;

    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum ConfigField {
      Path,
      Flip,
      Flop,
      Rotate,
    }

    macro_rules! return_if_duplicate {
      ($var:expr, $field:literal) => {
        if $var.is_some() {
          return Err(DeError::duplicate_field($field));
        }
      };
    }

    impl<'de> Visitor<'de> for ValueVisitor {
      type Value = AssetValue;

      fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a path string or config struct")
      }

      fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        Ok(AssetValue::Short(PathBuf::deserialize(
          v.into_deserializer(),
        )?))
      }

      fn visit_map<A: MapAccess<'de>>(
        self,
        mut map: A,
      ) -> Result<Self::Value, A::Error> {
        let mut path = None;
        let mut flip = None;
        let mut flop = None;
        let mut rotate = None;

        while let Some(key) = map.next_key()? {
          use ConfigField::*;

          match key {
            Path => {
              return_if_duplicate!(path, "path");
              path = Some(map.next_value()?);
            }
            Flip => {
              return_if_duplicate!(flip, "flip");
              flip = Some(map.next_value()?);
            }
            Flop => {
              return_if_duplicate!(flop, "flop");
              flop = Some(map.next_value()?);
            }
            Rotate => {
              return_if_duplicate!(rotate, "rotate");
              rotate = Some(map.next_value()?);
            }
          }
        }

        if path.is_none() {
          return Err(DeError::missing_field("path"));
        }

        Ok(AssetValue::Verbose {
          path: path.unwrap(),
          flip,
          flop,
          rotate,
        })
      }
    }

    de.deserialize_any(ValueVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde::de::Error as DeError;
  use toml::Value;

  #[test]
  fn deserialize_from_string() {
    let raw_value = r#""/foo/bar""#;

    let toml_value: Value = raw_value.parse().unwrap();
    let value = AssetValue::deserialize(toml_value).unwrap();

    let expected = AssetValue::Short(PathBuf::from("/foo/bar"));

    assert_eq!(value, expected)
  }

  #[test]
  fn deserialize_from_table() {
    let raw_value = r#"{ path = "/foo/bar" }"#;

    let toml_value: Value = raw_value.parse().unwrap();
    let value = AssetValue::deserialize(toml_value).unwrap();

    let expected = AssetValue::Verbose {
      path: PathBuf::from("/foo/bar"),
      flip: None,
      flop: None,
      rotate: None,
    };

    assert_eq!(value, expected)
  }

  #[test]
  fn deserialize_from_table_err() {
    let raw_value = "{ }";

    let toml_value: Value = raw_value.parse().unwrap();
    let value = AssetValue::deserialize(toml_value);

    let expected = Err(DeError::custom("missing field `path`"));

    assert_eq!(value, expected)
  }

  #[test]
  fn deserialize_from_table_with_transforms() {
    let raw_value =
      r#"{ path = "/foo/bar", flip = true, flop = true, rotate = 180 }"#;

    let toml_value: Value = raw_value.parse().unwrap();
    let value = AssetValue::deserialize(toml_value).unwrap();

    let expected = AssetValue::Verbose {
      path: PathBuf::from("/foo/bar"),
      flip: Some(true),
      flop: Some(true),
      rotate: Some(RotateValue::OneEighty),
    };

    assert_eq!(value, expected);
  }
}
