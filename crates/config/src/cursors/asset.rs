use std::fmt::{Formatter, Result as FmtResult};
use std::path::PathBuf;

use serde::de::{Error as DeError, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Default, PartialEq)]
pub struct AssetConfig {
  /// Path to the image file.
  pub path: PathBuf,
  /// Whether the image should be horizontally flipped.
  pub flip: Option<bool>,
  /// Whether the image should be vertically flipped.
  pub flop: Option<bool>,
  /// How much the image should be rotated.
  pub rotate: Option<RotateValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RotateValue {
  Ninety = 90,
  OneEighty = 180,
  TwoSeventy = 270,
}

impl From<PathBuf> for AssetConfig {
  fn from(value: PathBuf) -> Self {
    Self {
      path: value,
      ..Default::default()
    }
  }
}

impl<'de> Deserialize<'de> for AssetConfig {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct ConfigVisitor;

    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum ConfigField {
      Path,
      Flip,
      Flop,
      Rotate,
    }

    impl<'de> Visitor<'de> for ConfigVisitor {
      type Value = AssetConfig;

      fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("path string or config struct")
      }

      fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        let path = PathBuf::deserialize(v.into_deserializer())?;

        Ok(Self::Value {
          path,
          flip: None,
          flop: None,
          rotate: None,
        })
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
              if path.is_some() {
                return Err(DeError::duplicate_field("path"));
              }
              path = Some(map.next_value()?);
            }
            Flip => {
              if flip.is_some() {
                return Err(DeError::duplicate_field("flip"));
              }
              flip = Some(map.next_value()?);
            }
            Flop => {
              if flop.is_some() {
                return Err(DeError::duplicate_field("flop"));
              }
              flop = Some(map.next_value()?);
            }
            Rotate => {
              if rotate.is_some() {
                return Err(DeError::duplicate_field("rotate"));
              }

              let value: u16 = map.next_value()?;

              rotate = match value {
                90 => Some(RotateValue::Ninety),
                180 => Some(RotateValue::OneEighty),
                270 => Some(RotateValue::TwoSeventy),
                _ => {
                  return Err(DeError::custom(
                    "rotate must be one of 90, 180, or 270",
                  ));
                }
              };
            }
          }
        }

        let path = path.ok_or_else(|| DeError::missing_field("path"))?;

        Ok(Self::Value {
          path,
          flip,
          flop,
          rotate,
        })
      }
    }

    de.deserialize_any(ConfigVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde::de::Error as DeError;
  use toml::Value;

  #[test]
  fn from_path_buf() {
    assert_eq!(AssetConfig::from(PathBuf::new()), Default::default());
  }

  #[test]
  fn deserialize_from_string() {
    let raw_value = r#""/foo/bar""#;

    let de = raw_value.parse::<Value>().unwrap();
    let value = AssetConfig::deserialize(de).unwrap();

    assert_eq!(value, PathBuf::from("/foo/bar").into())
  }

  #[test]
  fn deserialize_from_table() {
    let raw_value = r#"{ path = "/foo/bar" }"#;

    let de = raw_value.parse::<Value>().unwrap();
    let value = AssetConfig::deserialize(de).unwrap();

    assert_eq!(value, PathBuf::from("/foo/bar").into())
  }

  #[test]
  fn deserialize_from_table_with_transforms() {
    let raw_value =
      r#"{ path = "/foo/bar", flip = true, flop = true, rotate = 180 }"#;

    let de = raw_value.parse::<Value>().unwrap();
    let value = AssetConfig::deserialize(de).unwrap();

    assert_eq!(
      value,
      AssetConfig {
        path: PathBuf::from("/foo/bar"),
        flip: Some(true),
        flop: Some(true),
        rotate: Some(RotateValue::OneEighty),
      }
    )
  }

  #[test]
  fn deserialize_from_table_with_transforms_err() {
    let raw_value =
      r#"{ path = "/foo/bar", flip = true, flop = true, rotate = 365 }"#;

    let de = raw_value.parse::<Value>().unwrap();
    let value = AssetConfig::deserialize(de);

    assert_eq!(
      value,
      Err(DeError::custom("rotate must be one of 90, 180, or 270"))
    )
  }
}
