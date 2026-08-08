mod alias;
mod locale;

use std::convert::Infallible;
use std::fmt::{Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::num::NonZero;
use std::path::PathBuf;
use std::str::FromStr;

use serde::de::value::MapAccessDeserializer;
use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::config::alias::PlatformAlias;
use crate::config::locale::LocaleString;

#[derive(Deserialize)]
pub struct PackageConfig {
  /// Short name for the cursor theme.
  pub name: LocaleString,
  /// Long description for the cursor theme.
  pub comment: LocaleString,
  /// Whether to hide the cursor theme in UIs, usually enabled for fallback
  /// themes. Only applies to Linux packages.
  #[serde(default)]
  pub hidden: Option<bool>,
  /// Name of a specific cursor to use as an example for the wider theme. Only
  /// applies to Linux packages.
  #[serde(default)]
  pub example: Option<String>,
}

#[derive(Default)]
pub struct AssetConfig {
  /// Path to the image file.
  pub path: PathBuf,
  /// Whether the image should be horizontally flipped.
  pub flip: bool,
  /// Whether the image should be vertically flipped.
  pub flop: bool,
  /// How much the image should be rotated.
  pub rotate: Option<i32>,
}

impl FromStr for AssetConfig {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      path: s.into(),
      ..Self::default()
    })
  }
}

impl<'de> Deserialize<'de> for AssetConfig {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    string_or_struct(de)
  }
}

#[derive(Deserialize)]
pub struct FrameConfig {
  /// Path to the image file.
  pub asset: AssetConfig,
  /// How long this frame should last (in milliseconds).
  pub duration: Option<NonZero<u32>>,
  /// Cordinates of the cursor tip.
  pub hotspot: (u32, u32),
  /// Nominal size of the cursor.
  pub size: NonZero<u32>,
}

pub struct CursorConfig {
  pub name: String,
  pub frames: Vec<FrameConfig>,
  pub aliases: Vec<PlatformAlias>,
}

#[derive(Deserialize)]
pub struct Config {
  pub package: PackageConfig,
}

// https://serde.rs/string-or-struct.html
fn string_or_struct<
  'de,
  T: Deserialize<'de> + FromStr<Err = Infallible>,
  D: Deserializer<'de>,
>(
  deserializer: D,
) -> Result<T, D::Error> {
  struct StringOrStruct<T>(PhantomData<fn() -> T>);

  impl<'de, T: Deserialize<'de> + FromStr<Err = Infallible>> Visitor<'de>
    for StringOrStruct<T>
  {
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
      formatter.write_str("string or map")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<T, E> {
      Ok(FromStr::from_str(value).unwrap())
    }

    fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<T, M::Error> {
      Deserialize::deserialize(MapAccessDeserializer::new(map))
    }
  }

  deserializer.deserialize_any(StringOrStruct(PhantomData))
}
