use std::fmt::{Formatter, Result as FmtResult};

use crate_cursors::{CursorDuration, CursorHotspot};

use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Default, PartialEq)]
pub struct ScaledCursorFrameConfig {
  /// Index of an asset in `super::assets`.
  pub asset: usize,
  /// Nominal size of the frame (overrides `super::nominal`).
  pub nominal: Option<u32>,
  /// Co-ordinates of the cursor tip (overrides `super::hotspot`).
  pub hotspot: Option<CursorHotspot>,
  /// Duration of this frame (conflicts with `super::durations`).
  pub duration: Option<CursorDuration>,
}

impl From<usize> for ScaledCursorFrameConfig {
  fn from(value: usize) -> Self {
    Self {
      asset: value,
      ..Default::default()
    }
  }
}

impl<'de> Deserialize<'de> for ScaledCursorFrameConfig {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct ConfigVisitor;

    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum ConfigField {
      Asset,
      Nominal,
      Hotspot,
      Duration,
    }

    impl<'de> Visitor<'de> for ConfigVisitor {
      type Value = ScaledCursorFrameConfig;

      fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("positive integer or frame config")
      }

      fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
        if v < 0 {
          return Err(E::custom("integer must be positive"));
        }
        Ok(Self::Value::from(v as usize))
      }

      fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Self::Value::from(v as usize))
      }

      fn visit_map<A: MapAccess<'de>>(
        self,
        mut map: A,
      ) -> Result<Self::Value, A::Error> {
        let mut asset = None;
        let mut nominal = None;
        let mut hotspot = None;
        let mut duration = None;

        while let Some(key) = map.next_key()? {
          use ConfigField::*;

          match key {
            Asset => {
              if asset.is_some() {
                return Err(DeError::duplicate_field("asset"));
              }
              asset = Some(map.next_value()?);
            }
            Nominal => {
              if nominal.is_some() {
                return Err(DeError::duplicate_field("nominal"));
              }
              nominal = Some(map.next_value()?);
            }
            Hotspot => {
              if hotspot.is_some() {
                return Err(DeError::duplicate_field("hotspot"));
              }
              hotspot = Some(map.next_value()?);
            }
            Duration => {
              if duration.is_some() {
                return Err(DeError::duplicate_field("duration"));
              }
              duration = Some(map.next_value()?);
            }
          }
        }

        let asset = asset.ok_or_else(|| DeError::missing_field("asset"))?;

        Ok(Self::Value {
          asset,
          nominal,
          hotspot,
          duration,
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
  fn test_from_usize() {
    assert_eq!(ScaledCursorFrameConfig::from(0), Default::default());
  }

  #[test]
  fn test_deserialize_from_int() {
    let de = "0".parse::<Value>().unwrap();
    let value = ScaledCursorFrameConfig::deserialize(de).unwrap();

    assert_eq!(value, Default::default())
  }

  #[test]
  fn test_deserialize_from_int_error() {
    let de = "-1".parse::<Value>().unwrap();
    let value = ScaledCursorFrameConfig::deserialize(de);

    assert_eq!(value, Err(DeError::custom("integer must be positive")))
  }

  #[test]
  fn test_deserialize_from_table() {
    let raw_value = r#"{ asset = 0 }"#;

    let de = raw_value.parse::<Value>().unwrap();
    let value = ScaledCursorFrameConfig::deserialize(de).unwrap();

    assert_eq!(value, Default::default())
  }

  #[test]
  fn test_deserialize_from_table_with_overrides() {
    let raw_value =
      r#"{ asset = 0, nominal = 24, hotspot = [4, 4], duration = 200 }"#;

    let de = raw_value.parse::<Value>().unwrap();
    let value = ScaledCursorFrameConfig::deserialize(de).unwrap();

    assert_eq!(
      value,
      ScaledCursorFrameConfig {
        asset: 0,
        nominal: Some(24),
        hotspot: Some(CursorHotspot { x: 4, y: 4 }),
        duration: Some(CursorDuration::from_milliseconds(200)),
      }
    )
  }
}
