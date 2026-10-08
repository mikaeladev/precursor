use crate_pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use crate_pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct LumaAlphaPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<LumaAlphaPixel>,
}

impl_pixmap_new!(LumaAlphaPixmap);

impl_from_paletted_for_paletted!(
  LumaPixmap,
  LumaAlphaPixmap,
  "Converts a [`LumaPixmap`] into a [`LumaAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbPixmap,
  LumaAlphaPixmap,
  "Converts an [`RgbPixmap`] into a [`LumaAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbAlphaPixmap,
  LumaAlphaPixmap,
  "Converts an [`RgbAlphaPixmap`] into a [`LumaAlphaPixmap`]."
);

impl_from_indexed_for_paletted_alpha!(
  IndexedPixmap,
  LumaAlphaPixmap,
  "Converts an [`IndexedPixmap`] into a [`LumaAlphaPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq)]
pub struct LumaAlphaPixel {
  pub y: u8,
  pub a: u8,
}

impl FromPixel<LumaPixel> for LumaAlphaPixel {
  /// Converts a `LumaPixel` into a `LumaAlphaPixel`.
  fn from_pixel(pixel: LumaPixel) -> Self {
    Self {
      y: pixel.y,
      a: u8::MAX,
    }
  }
}

impl FromPixel<RgbPixel> for LumaAlphaPixel {
  /// Converts an `RgbPixel` into a `LumaAlphaPixel`.
  fn from_pixel(pixel: RgbPixel) -> Self {
    Self {
      y: LumaPixel::from_pixel(pixel).y,
      a: u8::MAX,
    }
  }
}

impl FromPixel<RgbAlphaPixel> for LumaAlphaPixel {
  /// Converts an `RgbAlphaPixel` into a  `LumaAlphaPixel`.
  fn from_pixel(pixel: RgbAlphaPixel) -> Self {
    Self {
      y: LumaPixel::from_pixel(pixel).y,
      a: pixel.a,
    }
  }
}

#[cfg(test)]
mod pixel_tests {
  use crate::variants::rgb::INDIAN_RED;

  use super::*;

  #[test]
  fn from_luma() {
    // alpha defaults to 255
    assert_eq!(
      LumaAlphaPixel::from_pixel(LumaPixel { y: u8::MAX }),
      LumaAlphaPixel {
        y: u8::MAX,
        a: u8::MAX
      }
    );
  }

  #[test]
  fn from_rgb() {
    // rgb -> greyscale, alpha defaults to 255
    assert_eq!(
      LumaAlphaPixel::from_pixel(INDIAN_RED),
      LumaAlphaPixel { y: 116, a: u8::MAX }
    );
  }

  #[test]
  fn from_rgb_alpha() {
    // rgb -> greyscale, alpha gets inherited
    assert_eq!(
      LumaAlphaPixel::from_pixel(RgbAlphaPixel {
        r: INDIAN_RED.r,
        g: INDIAN_RED.g,
        b: INDIAN_RED.b,
        a: 123,
      }),
      LumaAlphaPixel { y: 116, a: 123 }
    );
  }
}
