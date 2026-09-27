use std::fmt::{Formatter, Result as FmtResult};

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::AssetValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorVariant {
  ScaledStatic {
    icon: CursorIcon,
  },
  ScaledAnimated {
    nominal: u32,
    hotspot: (u32, u32),
    sequence: Vec<ScaledFrame>,
  },
  VerboseStatic {
    icons: Vec<CursorIcon>,
  },
  VerboseAnimated {
    sequence: Vec<VerboseFrame>,
  },
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CursorIcon {
  pub asset: AssetValue,
  pub nominal: u32,
  pub hotspot: (u32, u32),
}

pub type ScaledFrame = (AssetValue, u32);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VerboseFrame {
  pub icons: Vec<CursorIcon>,
  pub duration: u32,
}

const MISSING_FIELDS: &str =
  "missing fields, expected one of `asset`, `icons`, or `sequence`";

impl<'de> Deserialize<'de> for CursorVariant {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct CursorVariantVisitor;

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
          return Err(de::Error::duplicate_field($field));
        }
      };
    }

    macro_rules! return_if_none {
      ($var:expr, $field:literal) => {
        if $var.is_none() {
          return Err(de::Error::missing_field($field));
        }
      };
    }

    macro_rules! return_if_some {
      ($var:expr, $field:literal, $expected:expr) => {
        if $var.is_some() {
          return Err(de::Error::unknown_field($field, $expected));
        }
      };
    }

    impl<'de> Visitor<'de> for CursorVariantVisitor {
      type Value = CursorVariant;

      fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a cursor variant")
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

          Ok(CursorVariant::ScaledStatic {
            icon: CursorIcon {
              asset,
              nominal: nominal.unwrap(),
              hotspot: hotspot.unwrap(),
            },
          })
        } else if let Some(sequence) = scaled_sequence {
          let expected = &["nominal", "hotspot", "sequence"];

          return_if_some!(icons, "icons", expected);

          Ok(CursorVariant::ScaledAnimated {
            nominal: nominal.unwrap(),
            hotspot: hotspot.unwrap(),
            sequence,
          })
        } else if let Some(icons) = icons {
          let expected = &["icons"];

          return_if_some!(nominal, "nominal", expected);
          return_if_some!(hotspot, "hotspot", expected);
          return_if_some!(verbose_sequence, "sequence", expected);

          Ok(CursorVariant::VerboseStatic { icons })
        } else if let Some(sequence) = verbose_sequence {
          let expected = &["sequence"];

          return_if_some!(nominal, "nominal", expected);
          return_if_some!(hotspot, "hotspot", expected);

          Ok(CursorVariant::VerboseAnimated { sequence })
        } else {
          Err(de::Error::custom(MISSING_FIELDS))
        }
      }
    }

    de.deserialize_map(CursorVariantVisitor)
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use toml::toml;

  use super::*;

  #[test]
  fn deserialize_with_missing_fields() {
    let toml_value = toml! {
      nominal = 12
      hotspot = [2, 2]
    };

    let value = CursorVariant::deserialize(toml_value).unwrap_err();
    let expected = de::Error::custom(MISSING_FIELDS);

    assert_eq!(value, expected)
  }

  #[test]
  fn deserialize_scaled_static() {
    let toml_value = toml! {
      asset = "test.png"
      nominal = 12
      hotspot = [2, 2]
    };

    let value = CursorVariant::deserialize(toml_value).unwrap();

    let expected = CursorVariant::ScaledStatic {
      icon: CursorIcon {
        asset: AssetValue::Short(PathBuf::from("test.png")),
        nominal: 12,
        hotspot: (2, 2),
      },
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_scaled_static_err() {
    let expected_fields = &["asset", "nominal", "hotspot"];

    let exprs = [
      (
        toml! { asset = "test.png" },
        de::Error::missing_field(expected_fields[1]),
      ),
      (
        toml! {
          asset = "test.png"
          nominal = 12
        },
        de::Error::missing_field(expected_fields[2]),
      ),
      (
        toml! {
          asset = "test.png"
          nominal = 12
          hotspot = [2, 2]
          icons = []
        },
        de::Error::unknown_field("icons", expected_fields),
      ),
      (
        toml! {
          asset = "test.png"
          nominal = 12
          hotspot = [2, 2]
          sequence = []
        },
        de::Error::unknown_field("sequence", expected_fields),
      ),
    ];

    for (toml_value, expected) in exprs {
      let value = CursorVariant::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }

  #[test]
  fn deserialize_scaled_animated() {
    let toml_value = toml! {
      nominal = 12
      hotspot = [2, 2]
      sequence = [
        ["test-01.png", 200],
        ["test-02.png", 200],
        ["test-03.png", 200],
      ]
    };

    let value = CursorVariant::deserialize(toml_value).unwrap();

    let expected = CursorVariant::ScaledAnimated {
      nominal: 12,
      hotspot: (2, 2),
      sequence: vec![
        (AssetValue::Short(PathBuf::from("test-01.png")), 200),
        (AssetValue::Short(PathBuf::from("test-02.png")), 200),
        (AssetValue::Short(PathBuf::from("test-03.png")), 200),
      ],
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_scaled_animated_err() {
    let toml_value = toml! {
      nominal = 12
      hotspot = [2, 2]
      sequence = []
      icons = []
    };

    let value = CursorVariant::deserialize(toml_value).unwrap_err();

    let expected =
      de::Error::unknown_field("icons", &["nominal", "hotspot", "sequence"]);

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_verbose_static() {
    let toml_value = toml! {
      [[icons]]
      asset = "x1/test.png"
      nominal = 12
      hotspot = [2, 2]
    };

    let value = CursorVariant::deserialize(toml_value).unwrap();

    let expected = CursorVariant::VerboseStatic {
      icons: vec![CursorIcon {
        asset: AssetValue::Short(PathBuf::from("x1/test.png")),
        nominal: 12,
        hotspot: (2, 2),
      }],
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_verbose_static_err() {
    let expected_fields = &["icons"];

    let exprs = [
      (
        toml! {
          nominal = 12
          icons = []
        },
        de::Error::unknown_field("nominal", expected_fields),
      ),
      (
        toml! {
          hotspot = [2, 2]
          icons = []
        },
        de::Error::unknown_field("hotspot", expected_fields),
      ),
      (
        toml! {
          sequence = []
          icons = []
        },
        de::Error::unknown_field("sequence", expected_fields),
      ),
    ];

    for (toml_value, expected) in exprs {
      let value = CursorVariant::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }

  #[test]
  fn deserialize_verbose_animated() {
    let toml_value = toml! {
      [[sequence]]
      duration = 200

      [[sequence.icons]]
      asset = "x1/test-01.png"
      nominal = 12
      hotspot = [2, 2]
    };

    let value = CursorVariant::deserialize(toml_value).unwrap();

    let expected = CursorVariant::VerboseAnimated {
      sequence: vec![VerboseFrame {
        duration: 200,
        icons: vec![CursorIcon {
          asset: AssetValue::Short(PathBuf::from("x1/test-01.png")),
          nominal: 12,
          hotspot: (2, 2),
        }],
      }],
    };

    assert_eq!(value, expected);
  }

  #[test]
  fn deserialize_verbose_animated_err() {
    let expected_fields = &["sequence"];

    let exprs = [
      (
        toml! {
          nominal = 12
          sequence = []
        },
        de::Error::unknown_field("nominal", expected_fields),
      ),
      (
        toml! {
          hotspot = [2, 2]
          sequence = []
        },
        de::Error::unknown_field("hotspot", expected_fields),
      ),
    ];

    for (toml_value, expected) in exprs {
      let value = CursorVariant::deserialize(toml_value).unwrap_err();

      assert_eq!(value, expected);
    }
  }
}
