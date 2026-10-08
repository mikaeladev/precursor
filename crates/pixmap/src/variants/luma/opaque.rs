use crate_pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use crate_pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct LumaPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<LumaPixel>,
}

impl_pixmap_new!(LumaPixmap);

impl_from_paletted_for_paletted!(
  LumaAlphaPixmap,
  LumaPixmap,
  "Converts a [`LumaAlphaPixmap`] into a [`LumaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbPixmap,
  LumaPixmap,
  "Converts an [`RgbPixmap`] into a [`LumaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbAlphaPixmap,
  LumaPixmap,
  "Converts an [`RgbAlphaPixmap`] into a [`LumaPixmap`]."
);

impl_from_indexed_for_paletted_opaque!(
  IndexedPixmap,
  LumaPixmap,
  "Converts an [`IndexedPixmap`] into a [`LumaPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq)]
pub struct LumaPixel {
  pub y: u8,
}

impl FromPixel<LumaAlphaPixel> for LumaPixel {
  /// Converts a `LumaAlphaPixel` into a `LumaPixel`.
  fn from_pixel(pixel: LumaAlphaPixel) -> Self {
    Self { y: pixel.y }
  }
}

impl FromPixel<RgbPixel> for LumaPixel {
  /// Converts an `RgbPixel` into a `LumaPixel`.
  fn from_pixel(pixel: RgbPixel) -> Self {
    Self {
      y: (pixel.r as f32 * 0.2126
        + pixel.g as f32 * 0.7152
        + pixel.b as f32 * 0.0722) as u8,
    }
  }
}

impl FromPixel<RgbAlphaPixel> for LumaPixel {
  /// Converts an `RgbAlphaPixel` into a `LumaPixel`.
  fn from_pixel(pixel: RgbAlphaPixel) -> Self {
    Self::from_pixel(RgbPixel::from_pixel(pixel))
  }
}

#[cfg(test)]
mod pixel_tests {
  use super::*;

  #[test]
  fn from_luma_alpha() {
    // alpha gets droppped
    assert_eq!(
      LumaPixel::from_pixel(LumaAlphaPixel {
        y: u8::MAX,
        a: u8::MAX
      }),
      LumaPixel { y: u8::MAX }
    );
  }

  #[test]
  fn from_rgb() {
    // rgb -> greyscale
    assert_eq!(LumaPixel::from_pixel(INDIAN_RED), LumaPixel { y: 116 });
    assert_eq!(LumaPixel::from_pixel(WHITE), LumaPixel { y: u8::MAX });
  }

  #[test]
  fn from_rgb_alpha() {
    // rgb -> greyscale, alpha gets dropped
    assert_eq!(
      LumaPixel::from_pixel(RgbAlphaPixel {
        r: INDIAN_RED.r,
        g: INDIAN_RED.g,
        b: INDIAN_RED.b,
        a: u8::MAX,
      }),
      LumaPixel { y: 116 }
    );
  }
}
