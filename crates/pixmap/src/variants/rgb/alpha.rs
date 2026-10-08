use crate_pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use crate_pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct RgbAlphaPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<RgbAlphaPixel>,
}

impl_pixmap_new!(RgbAlphaPixmap);

impl_from_paletted_for_paletted!(
  LumaPixmap,
  RgbAlphaPixmap,
  "Converts a [`LumaPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  LumaAlphaPixmap,
  RgbAlphaPixmap,
  "Converts a [`LumaAlphaPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbPixmap,
  RgbAlphaPixmap,
  "Converts an [`RgbPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_indexed_for_paletted_alpha!(
  IndexedPixmap,
  RgbAlphaPixmap,
  "Converts an [`IndexedPixmap`] into an [`RgbAlphaPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq)]
pub struct RgbAlphaPixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl FromPixel<LumaPixel> for RgbAlphaPixel {
  /// Converts a `LumaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(pixel: LumaPixel) -> Self {
    Self::from_pixel(RgbPixel::from_pixel(pixel))
  }
}

impl FromPixel<LumaAlphaPixel> for RgbAlphaPixel {
  /// Converts a `LumaAlphaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(pixel: LumaAlphaPixel) -> Self {
    Self {
      r: pixel.y,
      g: pixel.y,
      b: pixel.y,
      a: pixel.a,
    }
  }
}

impl FromPixel<RgbPixel> for RgbAlphaPixel {
  /// Converts an `RgbPixel` into an `RgbAlphaPixel`.
  fn from_pixel(pixel: RgbPixel) -> Self {
    Self {
      r: pixel.r,
      g: pixel.g,
      b: pixel.b,
      a: u8::MAX,
    }
  }
}

#[cfg(test)]
mod pixel_tests {
  use crate::variants::rgb::INDIAN_RED;

  use super::*;

  #[test]
  fn from_luma() {
    // y gets propagated, alpha defaults to 255
    assert_eq!(
      RgbAlphaPixel::from_pixel(LumaPixel { y: 123 }),
      RgbAlphaPixel {
        r: 123,
        g: 123,
        b: 123,
        a: 255,
      }
    );
  }

  #[test]
  fn from_luma_alpha() {
    // y gets propagated, alpha gets inherited
    assert_eq!(
      RgbAlphaPixel::from_pixel(LumaAlphaPixel { y: 123, a: 123 }),
      RgbAlphaPixel {
        r: 123,
        g: 123,
        b: 123,
        a: 123,
      }
    );
  }

  #[test]
  fn from_rgb() {
    // rgb gets inherited, alpha defaults to 255
    assert_eq!(
      RgbAlphaPixel::from_pixel(INDIAN_RED),
      RgbAlphaPixel {
        r: INDIAN_RED.r,
        g: INDIAN_RED.g,
        b: INDIAN_RED.b,
        a: 255,
      }
    );
  }
}
