mod index;

use std::num::NonZero;
use std::path::PathBuf;

use point::Point;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

pub use index::*;

#[derive(Debug, Deserialize)]
pub struct AssetConfig {
  pub path: PathBuf,
  pub nominal: NonZero<u32>,
  pub hotspot: Point<u32>,
  pub transforms: Option<TransformsConfig>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct TransformsConfig {
  pub flip: bool,
  pub flop: bool,
  pub rotate: RotateValue,
}

#[derive(Debug, Default, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RotateValue {
  #[default]
  Zero = 0,
  Ninety = 90,
  OneEighty = 180,
  TwoSeventy = 270,
}
