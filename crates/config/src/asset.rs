use std::fmt::{Formatter, Result as FmtResult};
use std::path::PathBuf;

use serde::de::{Error as DeError, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

pub struct AssetConfig {
  /// Path to the image file.
  pub path: PathBuf,
  /// Whether the image should be horizontally flipped.
  pub flip: Option<bool>,
  /// Whether the image should be vertically flipped.
  pub flop: Option<bool>,
  /// How much the image should be rotated.
  pub rotate: Option<i32>,
}

impl<'de> Deserialize<'de> for AssetConfig {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct AssetConfigVisitor;

    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum AssetConfigField {
      Path,
      Flip,
      Flop,
      Rotate,
    }

    impl<'de> Visitor<'de> for AssetConfigVisitor {
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
          use AssetConfigField::*;

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
              rotate = Some(map.next_value()?);
            }
          }
        }

        Ok(Self::Value {
          path: path.ok_or_else(|| DeError::missing_field("path"))?,
          flip: flip.ok_or_else(|| DeError::missing_field("flip"))?,
          flop: flop.ok_or_else(|| DeError::missing_field("flop"))?,
          rotate: rotate.ok_or_else(|| DeError::missing_field("rotate"))?,
        })
      }
    }

    de.deserialize_any(AssetConfigVisitor)
  }
}
