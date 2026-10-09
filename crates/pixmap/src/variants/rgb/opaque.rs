use precursor_pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use precursor_pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct RgbPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<RgbPixel>,
}

impl_pixmap_new!(RgbPixmap);

impl_from_paletted_for_paletted!(
  LumaPixmap,
  RgbPixmap,
  "Converts a [`LumaPixmap`] into an [`RgbPixmap`]."
);

impl_from_paletted_for_paletted!(
  LumaAlphaPixmap,
  RgbPixmap,
  "Converts a [`LumaAlphaPixmap`] into an [`RgbPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbAlphaPixmap,
  RgbPixmap,
  "Converts an [`RgbAlphaPixmap`] into an [`RgbPixmap`]."
);

impl_from_indexed_for_paletted_opaque!(
  IndexedPixmap,
  RgbPixmap,
  "Converts an [`IndexedPixmap`] into an [`RgbPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbPixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl FromPixel<LumaPixel> for RgbPixel {
  /// Converts a `LumaPixel` into an `RgbPixel`.
  fn from_pixel(pixel: LumaPixel) -> Self {
    Self {
      r: pixel.y,
      g: pixel.y,
      b: pixel.y,
    }
  }
}

impl FromPixel<LumaAlphaPixel> for RgbPixel {
  /// Converts a `LumaAlphaPixel` into an `RgbPixel`.
  fn from_pixel(pixel: LumaAlphaPixel) -> Self {
    Self::from_pixel(LumaPixel::from_pixel(pixel))
  }
}

impl FromPixel<RgbAlphaPixel> for RgbPixel {
  /// Converts an `RgbAlphaPixel` into an `RgbPixel`.
  fn from_pixel(pixel: RgbAlphaPixel) -> Self {
    Self {
      r: pixel.r,
      g: pixel.g,
      b: pixel.b,
    }
  }
}

#[cfg(test)]
mod pixel_tests {
  use super::*;

  pub const WHITE: RgbPixel = RgbPixel {
    r: u8::MAX,
    g: u8::MAX,
    b: u8::MAX,
  };

  pub const INDIAN_RED: RgbPixel = RgbPixel {
    r: 205,
    g: 92,
    b: 92,
  };

  #[test]
  fn from_luma() {
    // y gets propagated
    assert_eq!(RgbPixel::from_pixel(LumaPixel { y: u8::MAX }), WHITE);
  }

  #[test]
  fn from_luma_alpha() {
    // alpha gets dropped
    assert_eq!(
      RgbPixel::from_pixel(LumaAlphaPixel {
        y: u8::MAX,
        a: u8::MAX
      }),
      WHITE
    );
  }

  #[test]
  fn from_rgb_alpha() {
    // alpha gets dropped
    assert_eq!(
      RgbPixel::from_pixel(RgbAlphaPixel {
        r: INDIAN_RED.r,
        g: INDIAN_RED.g,
        b: INDIAN_RED.b,
        a: u8::MAX,
      }),
      INDIAN_RED
    );
  }
}

#[cfg(test)]
pub(crate) use pixel_tests::{INDIAN_RED, WHITE};
