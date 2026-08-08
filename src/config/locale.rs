use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt::{Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

pub struct LocaleString(pub BTreeMap<String, String>);

impl FromStr for LocaleString {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let map = BTreeMap::from([("".to_owned(), s.to_owned())]);

    Ok(Self(map))
  }
}

impl<'de> Deserialize<'de> for LocaleString {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    struct StringOrMapVisitor;

    impl<'de> Visitor<'de> for StringOrMapVisitor {
      type Value = LocaleString;

      fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("string or map")
      }

      fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        Ok(LocaleString::from_str(value).unwrap())
      }

      fn visit_map<M: MapAccess<'de>>(
        self,
        mut access: M,
      ) -> Result<Self::Value, M::Error> {
        let mut map = BTreeMap::new();

        while let Some((key, value)) = access.next_entry()? {
          map.insert(key, value);
        }

        Ok(LocaleString(map))
      }
    }

    de.deserialize_any(StringOrMapVisitor)
  }
}
