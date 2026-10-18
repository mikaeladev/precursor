use pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

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

impl FromPixel<RgbAlphaPixel> for RgbPixel {
  /// Converts an `RgbAlphaPixel` into an `RgbPixel`.
  fn from_pixel(RgbAlphaPixel { r, g, b, a: _ }: RgbAlphaPixel) -> Self {
    Self { r, g, b }
  }
}

impl FromPixel<LumaPixel> for RgbPixel {
  /// Converts a `LumaPixel` into an `RgbPixel`.
  fn from_pixel(LumaPixel { y }: LumaPixel) -> Self {
    Self { r: y, g: y, b: y }
  }
}

impl FromPixel<LumaAlphaPixel> for RgbPixel {
  /// Converts a `LumaAlphaPixel` into an `RgbPixel`.
  fn from_pixel(LumaAlphaPixel { y, a: _ }: LumaAlphaPixel) -> Self {
    Self { r: y, g: y, b: y }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pixel_from_luma_opaque() {
    assert_eq!(RgbPixel::from_pixel(luma::GREY_OPAQUE), rgb::GREY_OPAQUE)
  }

  #[test]
  fn pixel_from_luma_alpha_solid() {
    assert_eq!(
      RgbPixel::from_pixel(luma::GREY_ALPHA_SOLID),
      rgb::GREY_OPAQUE
    )
  }

  #[test]
  fn pixel_from_luma_alpha_trans() {
    assert_eq!(
      RgbPixel::from_pixel(luma::GREY_ALPHA_TRANS),
      rgb::GREY_OPAQUE
    )
  }

  #[test]
  fn pixel_from_rgb_opaque() {
    assert_eq!(RgbPixel::from_pixel(rgb::RED_OPAQUE), rgb::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_rgb_alpha_solid() {
    assert_eq!(RgbPixel::from_pixel(rgb::RED_ALPHA_SOLID), rgb::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_rgb_alpha_trans() {
    assert_eq!(RgbPixel::from_pixel(rgb::RED_ALPHA_TRANS), rgb::RED_OPAQUE)
  }
}
