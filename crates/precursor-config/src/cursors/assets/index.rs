use std::fmt::{self, Formatter};
use std::num::NonZero;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

const DEFAULT_SCALE: NonZero<usize> = unsafe {
  // SAFETY: 1 is, in fact, not 0
  NonZero::new_unchecked(1)
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetIndex {
  pub index: usize,
  pub scale: NonZero<usize>,
}

impl Default for AssetIndex {
  fn default() -> Self {
    Self {
      index: 0,
      scale: DEFAULT_SCALE,
    }
  }
}

impl From<usize> for AssetIndex {
  fn from(index: usize) -> Self {
    AssetIndex {
      index,
      ..Default::default()
    }
  }
}

impl<'de> Deserialize<'de> for AssetIndex {
  fn deserialize<D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<Self, D::Error> {
    struct AssetIndexVisitor;

    impl<'de> Visitor<'de> for AssetIndexVisitor {
      type Value = AssetIndex;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "either an integer or a struct")
      }

      fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(AssetIndex::from(v as usize))
      }

      fn visit_map<A: MapAccess<'de>>(
        self,
        mut map: A,
      ) -> Result<Self::Value, A::Error> {
        let mut index = None;
        let mut scale = None;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
          Index,
          Scale,
        }

        while let Some(key) = map.next_key()? {
          match key {
            Field::Index => {
              if index.is_some() {
                return Err(de::Error::duplicate_field("index"));
              }
              index = Some(map.next_value()?)
            }
            Field::Scale => {
              if scale.is_some() {
                return Err(de::Error::duplicate_field("scale"));
              }
              scale = Some(map.next_value()?)
            }
          }
        }

        let index = index.ok_or(de::Error::missing_field("index"))?;
        let scale = scale.unwrap_or(DEFAULT_SCALE);

        Ok(AssetIndex { index, scale })
      }
    }

    deserializer.deserialize_any(AssetIndexVisitor)
  }
}

pub(crate) type AssetIndexes = Vec<AssetIndex>;
