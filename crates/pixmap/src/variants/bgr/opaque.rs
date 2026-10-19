use pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct BgrPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<BgrPixel>,
}

impl_pixmap_new!(BgrPixmap);

impl_from_paletted_for_paletted!(
  BgrAlphaPixmap,
  BgrPixmap,
  "Converts a [`BgrAlphaPixmap`] into a [`BgrPixmap`]."
);

impl_from_paletted_for_paletted!(
  LumaPixmap,
  BgrPixmap,
  "Converts a [`LumaPixmap`] into a [`BgrPixmap`]."
);

impl_from_paletted_for_paletted!(
  LumaAlphaPixmap,
  BgrPixmap,
  "Converts a [`LumaAlphaPixmap`] into a [`BgrPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbPixmap,
  BgrPixmap,
  "Converts an [`RgbPixmap`] into a [`BgrPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbAlphaPixmap,
  BgrPixmap,
  "Converts an [`RgbAlphaPixmap`] into a [`BgrPixmap`]."
);

impl_from_indexed_for_paletted_opaque!(
  IndexedPixmap,
  BgrPixmap,
  "Converts an [`IndexedPixmap`] into a [`BgrPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BgrPixel {
  pub b: u8,
  pub g: u8,
  pub r: u8,
}

impl FromPixel<BgrAlphaPixel> for BgrPixel {
  /// Converts a `BgrAlphaPixel` into a `BgrPixel`.
  fn from_pixel(BgrAlphaPixel { b, g, r, a: _ }: BgrAlphaPixel) -> Self {
    Self { b, g, r }
  }
}

impl FromPixel<LumaPixel> for BgrPixel {
  /// Converts a `LumaPixel` into a `BgrPixel`.
  fn from_pixel(LumaPixel { y }: LumaPixel) -> Self {
    Self { b: y, g: y, r: y }
  }
}

impl FromPixel<LumaAlphaPixel> for BgrPixel {
  /// Converts a `LumaAlphaPixel` into a `BgrPixel`.
  fn from_pixel(LumaAlphaPixel { y, a: _ }: LumaAlphaPixel) -> Self {
    Self { b: y, g: y, r: y }
  }
}

impl FromPixel<RgbPixel> for BgrPixel {
  /// Converts an `RgbPixel` into a `BgrPixel`.
  fn from_pixel(RgbPixel { r, g, b }: RgbPixel) -> Self {
    Self { b, g, r }
  }
}

impl FromPixel<RgbAlphaPixel> for BgrPixel {
  /// Converts an `RgbAlphaPixel` into a `BgrPixel`.
  fn from_pixel(RgbAlphaPixel { r, g, b, a: _ }: RgbAlphaPixel) -> Self {
    Self { b, g, r }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pixel_from_bgr_opaque() {
    assert_eq!(BgrPixel::from_pixel(bgr::RED_OPAQUE), bgr::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_bgr_alpha_solid() {
    assert_eq!(BgrPixel::from_pixel(bgr::RED_ALPHA_SOLID), bgr::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_bgr_alpha_trans() {
    assert_eq!(BgrPixel::from_pixel(bgr::RED_ALPHA_TRANS), bgr::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_luma_opaque() {
    assert_eq!(BgrPixel::from_pixel(luma::GREY_OPAQUE), bgr::GREY_OPAQUE)
  }

  #[test]
  fn pixel_from_luma_alpha_solid() {
    assert_eq!(
      BgrPixel::from_pixel(luma::GREY_ALPHA_SOLID),
      bgr::GREY_OPAQUE
    )
  }

  #[test]
  fn pixel_from_luma_alpha_trans() {
    assert_eq!(
      BgrPixel::from_pixel(luma::GREY_ALPHA_TRANS),
      bgr::GREY_OPAQUE
    )
  }

  #[test]
  fn pixel_from_rgb_opaque() {
    assert_eq!(BgrPixel::from_pixel(rgb::RED_OPAQUE), bgr::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_rgb_alpha_solid() {
    assert_eq!(BgrPixel::from_pixel(rgb::RED_ALPHA_SOLID), bgr::RED_OPAQUE)
  }

  #[test]
  fn pixel_from_rgb_alpha_trans() {
    assert_eq!(BgrPixel::from_pixel(rgb::RED_ALPHA_TRANS), bgr::RED_OPAQUE)
  }
}
