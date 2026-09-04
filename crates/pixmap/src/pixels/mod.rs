mod convert;

pub use convert::*;

pub trait Pixel: Clone + Copy + PartialEq + Eq {
  /// Number of channels in the pixel.
  const CHANNELS: usize;
}

pub trait FromBytes<const N: usize> {
  /// Converts a byte array into this type.
  fn from_bytes(bytes: [u8; N]) -> Self;
}

pub trait IntoBytes<const N: usize> {
  /// Converts this type into a byte array.
  fn into_bytes(self) -> [u8; N];
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

impl FromBytes<1> for LumaPixel {
  fn from_bytes([y]: [u8; 1]) -> Self {
    Self { y }
  }
}

impl IntoBytes<1> for LumaPixel {
  fn into_bytes(self) -> [u8; 1] {
    [self.y]
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

impl FromBytes<2> for LumaAlphaPixel {
  fn from_bytes([y, a]: [u8; 2]) -> Self {
    Self { y, a }
  }
}

impl IntoBytes<2> for LumaAlphaPixel {
  fn into_bytes(self) -> [u8; 2] {
    let Self { y, a } = self;
    [y, a]
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

impl FromBytes<3> for RgbPixel {
  fn from_bytes([r, g, b]: [u8; 3]) -> Self {
    Self { r, g, b }
  }
}

impl IntoBytes<3> for RgbPixel {
  fn into_bytes(self) -> [u8; 3] {
    let Self { r, g, b } = self;
    [r, g, b]
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

impl FromBytes<4> for RgbAlphaPixel {
  fn from_bytes([r, g, b, a]: [u8; 4]) -> Self {
    Self { r, g, b, a }
  }
}

impl IntoBytes<4> for RgbAlphaPixel {
  fn into_bytes(self) -> [u8; 4] {
    let Self { r, g, b, a } = self;
    [r, g, b, a]
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

impl FromBytes<1> for IndexedPixel {
  fn from_bytes([i]: [u8; 1]) -> Self {
    Self { i }
  }
}

impl IntoBytes<1> for IndexedPixel {
  fn into_bytes(self) -> [u8; 1] {
    [self.i]
  }
}
