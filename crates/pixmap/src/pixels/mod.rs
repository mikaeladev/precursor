mod convert;

use std::array::IntoIter;

pub use convert::*;

pub trait Pixel: Clone + Copy + PartialEq + Eq {
  /// Number of channels in the pixel.
  const CHANNELS: usize;
}

// -------------------------------------------------------------------------- //

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LumaPixel {
  pub y: u8,
}

impl Pixel for LumaPixel {
  /// 1 channel for luminence.
  const CHANNELS: usize = 1;
}

impl From<u8> for LumaPixel {
  fn from(y: u8) -> Self {
    Self { y }
  }
}

// -------------------------------------------------------------------------- //

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LumaAlphaPixel {
  pub y: u8,
  pub a: u8,
}

impl Pixel for LumaAlphaPixel {
  /// 2 channels for luminence and alpha.
  const CHANNELS: usize = 2;
}

impl From<[u8; 2]> for LumaAlphaPixel {
  fn from(ya: [u8; 2]) -> Self {
    Self { y: ya[0], a: ya[1] }
  }
}

impl IntoIterator for LumaAlphaPixel {
  type Item = u8;
  type IntoIter = IntoIter<Self::Item, 2>;

  fn into_iter(self) -> Self::IntoIter {
    [self.y, self.a].into_iter()
  }
}

// -------------------------------------------------------------------------- //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbPixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Pixel for RgbPixel {
  /// 3 channels for red, green, and blue.
  const CHANNELS: usize = 3;
}

impl From<[u8; 3]> for RgbPixel {
  fn from(rgb: [u8; 3]) -> Self {
    Self {
      r: rgb[0],
      g: rgb[1],
      b: rgb[2],
    }
  }
}

impl IntoIterator for RgbPixel {
  type Item = u8;
  type IntoIter = IntoIter<Self::Item, 3>;

  fn into_iter(self) -> Self::IntoIter {
    [self.r, self.g, self.b].into_iter()
  }
}

// -------------------------------------------------------------------------- //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbAlphaPixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Pixel for RgbAlphaPixel {
  /// 4 channels for red, green, blue, and alpha.
  const CHANNELS: usize = 4;
}

impl From<[u8; 4]> for RgbAlphaPixel {
  fn from(rgba: [u8; 4]) -> Self {
    Self {
      r: rgba[0],
      g: rgba[1],
      b: rgba[2],
      a: rgba[3],
    }
  }
}

impl IntoIterator for RgbAlphaPixel {
  type Item = u8;
  type IntoIter = IntoIter<Self::Item, 4>;

  fn into_iter(self) -> Self::IntoIter {
    [self.r, self.g, self.b, self.a].into_iter()
  }
}

// -------------------------------------------------------------------------- //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexedPixel {
  pub i: u8,
}

impl Pixel for IndexedPixel {
  /// 1 channel for the index.
  const CHANNELS: usize = 1;
}

impl From<u8> for IndexedPixel {
  fn from(i: u8) -> Self {
    Self { i }
  }
}
