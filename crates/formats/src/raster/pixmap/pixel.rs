use std::array::IntoIter;

pub trait Pixel {
  /// Number of channels in the pixel.
  const NUM_CHANNELS: usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LumaPixel {
  pub y: u8,
}

impl Pixel for LumaPixel {
  /// 1 channel for luminence.
  const NUM_CHANNELS: usize = 1;
}

impl From<u8> for LumaPixel {
  fn from(y: u8) -> Self {
    Self { y }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LumaAlphaPixel {
  pub y: u8,
  pub a: u8,
}

impl Pixel for LumaAlphaPixel {
  /// 2 channels for luminence and alpha.
  const NUM_CHANNELS: usize = 2;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbPixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Pixel for RgbPixel {
  /// 3 channels for red, green, and blue.
  const NUM_CHANNELS: usize = 3;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbAlphaPixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Pixel for RgbAlphaPixel {
  /// 4 channels for red, green, blue, and alpha.
  const NUM_CHANNELS: usize = 4;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaletteIndex {
  pub i: u8,
}

impl Pixel for PaletteIndex {
  /// 1 channel for the index.
  const NUM_CHANNELS: usize = 1;
}

impl From<u8> for PaletteIndex {
  fn from(i: u8) -> Self {
    Self { i }
  }
}

pub trait IntoNonPalettePixel {
  /// Converts the value to a `LumaPixel`.
  fn into_luma(self) -> LumaPixel;

  /// Converts the value to a `LumaAlphaPixel`.
  fn into_luma_alpha(self) -> LumaAlphaPixel;

  /// Converts the value to an `RgbPixel`.
  fn into_rgb(self) -> RgbPixel;

  /// Converts the value to an `RgbAlphaPixel`.
  fn into_rgb_alpha(self) -> RgbAlphaPixel;
}

impl IntoNonPalettePixel for LumaPixel {
  fn into_luma(self) -> Self {
    self
  }

  fn into_luma_alpha(self) -> LumaAlphaPixel {
    LumaAlphaPixel {
      y: self.y,
      a: u8::MAX,
    }
  }

  fn into_rgb(self) -> RgbPixel {
    RgbPixel {
      r: self.y,
      g: self.y,
      b: self.y,
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixel {
    RgbAlphaPixel {
      r: self.y,
      g: self.y,
      b: self.y,
      a: u8::MAX,
    }
  }
}

impl IntoNonPalettePixel for LumaAlphaPixel {
  fn into_luma(self) -> LumaPixel {
    LumaPixel { y: self.y }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixel {
    self
  }

  fn into_rgb(self) -> RgbPixel {
    RgbPixel {
      r: self.y,
      g: self.y,
      b: self.y,
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixel {
    RgbAlphaPixel {
      r: self.y,
      g: self.y,
      b: self.y,
      a: self.a,
    }
  }
}

impl IntoNonPalettePixel for RgbPixel {
  fn into_luma(self) -> LumaPixel {
    let [r, g, b] = [self.r as f32, self.b as f32, self.g as f32];

    LumaPixel {
      y: (r * 0.2126 + g * 0.7152 + b * 0.0722) as u8,
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixel {
    LumaAlphaPixel {
      y: Self::into_luma(self).y,
      a: u8::MAX,
    }
  }

  fn into_rgb(self) -> Self {
    self
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixel {
    RgbAlphaPixel {
      r: self.r,
      g: self.g,
      b: self.b,
      a: u8::MAX,
    }
  }
}

impl IntoNonPalettePixel for RgbAlphaPixel {
  fn into_luma(self) -> LumaPixel {
    LumaPixel {
      y: self.into_rgb().into_luma().y,
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixel {
    LumaAlphaPixel {
      a: self.a,
      y: self.into_luma().y,
    }
  }

  fn into_rgb(self) -> RgbPixel {
    RgbPixel {
      r: self.r,
      g: self.g,
      b: self.b,
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixel {
    self
  }
}
