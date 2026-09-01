use std::fmt::{Formatter, Result as FmtResult};

use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::AssetValue;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CursorConfig {
  pub name: String,
  pub targets: Option<CursorTargets>,

  #[serde(flatten)]
  pub subconfig: CursorSubconfig,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct CursorTargets {
  pub linux: Option<LinuxSpecificConfig>,
  pub windows: Option<WindowsSpecificConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct LinuxSpecificConfig {
  pub name: Option<String>,
  pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct WindowsSpecificConfig {
  pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorSubconfig {
  ScaledStatic {
    icon: CursorIconConfig,
  },
  ScaledAnimated {
    nominal: u32,
    hotspot: (u32, u32),
    sequence: Vec<ScaledFrame>,
  },
  VerboseStatic {
    icons: Vec<CursorIconConfig>,
  },
  VerboseAnimated {
    sequence: Vec<VerboseFrame>,
  },
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CursorIconConfig {
  pub asset: AssetValue,
  pub nominal: u32,
  pub hotspot: (u32, u32),
}

pub type ScaledFrame = (AssetValue, u32);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VerboseFrame {
  pub icons: Vec<CursorIconConfig>,
  pub duration: u32,
}

const MISSING_FIELDS: &str =
  "missing fields, expected one of `asset`, `icons`, or `sequence`";

impl<'de> Deserialize<'de> for CursorSubconfig {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct CursorSubconfigVisitor;

    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum SubconfigField {
      Asset,
      Nominal,
      Hotspot,
      Icons,
      Sequence,
    }

    macro_rules! return_if_duplicate {
      ($var:expr, $field:literal) => {
        if $var.is_some() {
          return Err(DeError::duplicate_field($field));
        }
      };
    }

    macro_rules! return_if_none {
      ($var:expr, $field:literal) => {
        if $var.is_none() {
          return Err(DeError::missing_field($field));
        }
      };
    }

    macro_rules! return_if_some {
      ($var:expr, $field:literal, $expected:expr) => {
        if $var.is_some() {
          return Err(DeError::unknown_field($field, $expected));
        }
      };
    }

    impl<'de> Visitor<'de> for CursorSubconfigVisitor {
      type Value = CursorSubconfig;

      fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a subconfig")
      }

      fn visit_map<A: MapAccess<'de>>(
        self,
        mut map: A,
      ) -> Result<Self::Value, A::Error> {
        let mut asset = None;
        let mut nominal = None;
        let mut hotspot = None;
        let mut icons = None;
        let mut scaled_sequence = None;
        let mut verbose_sequence = None;

        while let Some(key) = map.next_key()? {
          use SubconfigField::*;

          match key {
            Asset => {
              return_if_duplicate!(asset, "asset");
              asset = Some(map.next_value()?);
            }
            Nominal => {
              return_if_duplicate!(nominal, "nominal");
              nominal = Some(map.next_value()?);
            }
            Hotspot => {
              return_if_duplicate!(hotspot, "hotspot");
              hotspot = Some(map.next_value()?);
            }
            Icons => {
              return_if_duplicate!(icons, "icons");
              icons = Some(map.next_value()?);
            }
            Sequence => {
              return_if_duplicate!(scaled_sequence, "sequence");
              return_if_duplicate!(verbose_sequence, "sequence");

              if nominal.is_some() && hotspot.is_some() {
                scaled_sequence = Some(map.next_value()?);
              } else {
                verbose_sequence = Some(map.next_value()?);
              }
            }
          }
        }

        if let Some(asset) = asset {
          let expected = &["asset", "nominal", "hotspot"];

          return_if_none!(nominal, "nominal");
          return_if_none!(hotspot, "hotspot");

          return_if_some!(icons, "icons", expected);
          return_if_some!(scaled_sequence, "sequence", expected);
          return_if_some!(verbose_sequence, "sequence", expected);

          Ok(CursorSubconfig::ScaledStatic {
            icon: CursorIconConfig {
              asset,
              nominal: nominal.unwrap(),
              hotspot: hotspot.unwrap(),
            },
          })
        } else if let Some(sequence) = scaled_sequence {
          let expected = &["nominal", "hotspot", "sequence"];

          return_if_some!(icons, "icons", expected);

          Ok(CursorSubconfig::ScaledAnimated {
            nominal: nominal.unwrap(),
            hotspot: hotspot.unwrap(),
            sequence,
          })
        } else if let Some(icons) = icons {
          let expected = &["icons"];

          return_if_some!(nominal, "nominal", expected);
          return_if_some!(hotspot, "hotspot", expected);
          return_if_some!(verbose_sequence, "sequence", expected);

          Ok(CursorSubconfig::VerboseStatic { icons })
        } else if let Some(sequence) = verbose_sequence {
          let expected = &["sequence"];

          return_if_some!(nominal, "nominal", expected);
          return_if_some!(hotspot, "hotspot", expected);

          Ok(CursorSubconfig::VerboseAnimated { sequence })
        } else {
          Err(DeError::custom(MISSING_FIELDS))
        }
      }
    }

    de.deserialize_map(CursorSubconfigVisitor)
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use toml::Value;

  use super::*;

  #[derive(Deserialize)]
  struct TestDocument {
    value: CursorConfig,
  }

  #[test]
  fn deserialize_missing_fields_err() {
    let raw_value = "{ nominal = 12, hotspot = [2, 2] }";
    let toml_value: Value = raw_value.parse().unwrap();

    let value = CursorSubconfig::deserialize(toml_value).unwrap_err();
    let expected = DeError::custom(MISSING_FIELDS);

    assert_eq!(value, expected)
  }

  #[test]
  fn deserialize_scaled_static() {
    let raw_value = r#"
      [value]
      name = "test"
      asset = "test.png"
      nominal = 12
      hotspot = [2, 2]
    "#;

    let TestDocument { value } = toml::from_str(raw_value).unwrap();

    let expected = CursorConfig {
      name: String::from("test"),
      targets: None,
      subconfig: CursorSubconfig::ScaledStatic {
        icon: CursorIconConfig {
          asset: AssetValue::Short(PathBuf::from("test.png")),
          nominal: 12,
          hotspot: (2, 2),
        },
      },
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_scaled_static_err() {
    let expected_fields = &["asset", "nominal", "hotspot"];

    let exprs = [
      (
        r#"{ asset = "test.png" }"#,
        DeError::missing_field(expected_fields[1]),
      ),
      (
        r#"{ asset = "test.png", nominal = 12 }"#,
        DeError::missing_field(expected_fields[2]),
      ),
      (
        r#"{ asset = "test.png", nominal = 12, hotspot = [2, 2], icons = [] }"#,
        DeError::unknown_field("icons", expected_fields),
      ),
      (
        r#"{ asset = "test.png", nominal = 12, hotspot = [2, 2], sequence = [] }"#,
        DeError::unknown_field("sequence", expected_fields),
      ),
    ];

    for (raw_value, expected) in exprs {
      let toml_value: Value = raw_value.parse().unwrap();
      let value = CursorSubconfig::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }

  #[test]
  fn deserialize_scaled_animated() {
    let raw_value = r#"
      [value]
      name = "test"
      nominal = 12
      hotspot = [2, 2]
      sequence = [
        ["test-01.png", 200],
        ["test-02.png", 200],
        ["test-03.png", 200],
      ]
    "#;

    let TestDocument { value } = toml::from_str(raw_value).unwrap();

    let expected = CursorConfig {
      name: String::from("test"),
      targets: None,
      subconfig: CursorSubconfig::ScaledAnimated {
        nominal: 12,
        hotspot: (2, 2),
        sequence: vec![
          (AssetValue::Short(PathBuf::from("test-01.png")), 200),
          (AssetValue::Short(PathBuf::from("test-02.png")), 200),
          (AssetValue::Short(PathBuf::from("test-03.png")), 200),
        ],
      },
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_scaled_animated_err() {
    let expected_fields = &["nominal", "hotspot", "sequence"];

    let exprs = [(
      r#"{ nominal = 12, hotspot = [2, 2], sequence = [], icons = [] }"#,
      DeError::unknown_field("icons", expected_fields),
    )];

    for (raw_value, expected) in exprs {
      let toml_value: Value = raw_value.parse().unwrap();
      let value = CursorSubconfig::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }

  #[test]
  fn deserialize_verbose_static() {
    let raw_value = r#"
      [value]
      name = "test"

      [[value.icons]]
      asset = "x1/test.png"
      nominal = 12
      hotspot = [2, 2]
    "#;

    let TestDocument { value } = toml::from_str(raw_value).unwrap();

    let expected = CursorConfig {
      name: String::from("test"),
      targets: None,
      subconfig: CursorSubconfig::VerboseStatic {
        icons: vec![CursorIconConfig {
          asset: AssetValue::Short(PathBuf::from("x1/test.png")),
          nominal: 12,
          hotspot: (2, 2),
        }],
      },
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_verbose_static_err() {
    let expected_fields = &["icons"];

    let exprs = [
      (
        r#"{ nominal = 12, icons = [] }"#,
        DeError::unknown_field("nominal", expected_fields),
      ),
      (
        r#"{ hotspot = [2, 2], icons = [] }"#,
        DeError::unknown_field("hotspot", expected_fields),
      ),
      (
        r#"{ sequence = [], icons = [] }"#,
        DeError::unknown_field("sequence", expected_fields),
      ),
    ];

    for (raw_value, expected) in exprs {
      let toml_value: Value = raw_value.parse().unwrap();
      let value = CursorSubconfig::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }

  #[test]
  fn deserialize_verbose_animated() {
    let raw_value = r#"
      [value]
      name = "test"

      [[value.sequence]]
      duration = 200

      [[value.sequence.icons]]
      asset = "x1/test-01.png"
      nominal = 12
      hotspot = [2, 2]
    "#;

    let TestDocument { value } = toml::from_str(raw_value).unwrap();

    let expected = CursorConfig {
      name: String::from("test"),
      targets: None,
      subconfig: CursorSubconfig::VerboseAnimated {
        sequence: vec![VerboseFrame {
          duration: 200,
          icons: vec![CursorIconConfig {
            asset: AssetValue::Short(PathBuf::from("x1/test-01.png")),
            nominal: 12,
            hotspot: (2, 2),
          }],
        }],
      },
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_verbose_animated_err() {
    let expected_fields = &["sequence"];

    let exprs = [
      (
        r#"{ nominal = 12, sequence = [] }"#,
        DeError::unknown_field("nominal", expected_fields),
      ),
      (
        r#"{ hotspot = [2, 2], sequence = [] }"#,
        DeError::unknown_field("hotspot", expected_fields),
      ),
    ];

    for (raw_value, expected) in exprs {
      let toml_value: Value = raw_value.parse().unwrap();
      let value = CursorSubconfig::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }
}
