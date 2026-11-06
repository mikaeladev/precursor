use std::cmp::Ordering;
use std::num::NonZero;
use std::path::PathBuf;

use point::Point;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

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

pub(crate) type AssetIndexes = Vec<AssetIndex>;

#[derive(Debug, Deserialize, Clone, Copy, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum AssetIndex {
  Integer(usize),
  Struct { index: usize, scale: NonZero<usize> },
}

impl AssetIndex {
  const DEFAULT_SCALE: NonZero<usize> = NonZero::new(1).unwrap();

  /// Returns the index.
  pub const fn index(&self) -> usize {
    match self {
      Self::Integer(index) => *index,
      Self::Struct { index, .. } => *index,
    }
  }

  /// Returns the scale (defaults to `1`).
  pub const fn scale(&self) -> NonZero<usize> {
    match self {
      Self::Integer(_) => Self::DEFAULT_SCALE,
      Self::Struct { scale, .. } => *scale,
    }
  }

  /// Converts this type into its [`Struct`][Self::Struct] variant.
  pub const fn into_struct(self) -> Self {
    match self {
      Self::Integer(index) => Self::Struct {
        index,
        scale: Self::DEFAULT_SCALE,
      },
      Self::Struct { .. } => self,
    }
  }
}

impl PartialEq for AssetIndex {
  fn eq(&self, other: &Self) -> bool {
    self.index() == other.index() && self.scale() == other.scale()
  }
}

impl PartialOrd for AssetIndex {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(
      if self.index() < other.index()
        || self.index() == other.index() && self.scale() < other.scale()
      {
        Ordering::Less
      } else if self.index() == other.index() && self.scale() == other.scale() {
        Ordering::Equal
      } else {
        Ordering::Greater
      },
    )
  }
}

impl From<usize> for AssetIndex {
  fn from(value: usize) -> Self {
    Self::Integer(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sort_indexes() {
    const ASSETS: [AssetIndex; 6] = [
      AssetIndex::Integer(0),
      AssetIndex::Integer(1),
      AssetIndex::Integer(2),
      AssetIndex::Struct {
        index: 2,
        scale: NonZero::new(2).unwrap(),
      },
      AssetIndex::Struct {
        index: 3,
        scale: NonZero::new(1).unwrap(),
      },
      AssetIndex::Struct {
        index: 3,
        scale: NonZero::new(2).unwrap(),
      },
    ];

    let mut value = [
      ASSETS[4], ASSETS[1], ASSETS[5], ASSETS[2], ASSETS[3], ASSETS[0],
    ];

    assert!(ASSETS.is_sorted());
    assert!(!value.is_sorted());

    value.sort();

    assert_eq!(value, ASSETS)
  }
}
