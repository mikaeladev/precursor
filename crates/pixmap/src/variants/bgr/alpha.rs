use pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct BgrAlphaPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<BgrAlphaPixel>,
}

impl_pixmap_new!(BgrAlphaPixmap);

impl_from_paletted_for_paletted!(
  BgrPixmap,
  BgrAlphaPixmap,
  "Converts a [`BgrPixmap`] into a [`BgrAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  LumaPixmap,
  BgrAlphaPixmap,
  "Converts a [`LumaPixmap`] into a [`BgrAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  LumaAlphaPixmap,
  BgrAlphaPixmap,
  "Converts a [`LumaAlphaPixmap`] into a [`BgrAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbPixmap,
  BgrAlphaPixmap,
  "Converts an [`RgbPixmap`] into a [`BgrAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  RgbAlphaPixmap,
  BgrAlphaPixmap,
  "Converts an [`RgbAlphaPixmap`] into a [`BgrAlphaPixmap`]."
);

impl_from_indexed_for_paletted_alpha!(
  IndexedPixmap,
  BgrAlphaPixmap,
  "Converts an [`IndexedPixmap`] into a [`BgrAlphaPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BgrAlphaPixel {
  pub b: u8,
  pub g: u8,
  pub r: u8,
  pub a: u8,
}

impl FromPixel<BgrPixel> for BgrAlphaPixel {
  /// Converts a `BgrPixel` into a `BgrAlphaPixel`.
  fn from_pixel(BgrPixel { b, g, r }: BgrPixel) -> Self {
    let a = u8::MAX;
    Self { b, g, r, a }
  }
}

impl FromPixel<LumaPixel> for BgrAlphaPixel {
  /// Converts a `LumaPixel` into a `BgrAlphaPixel`.
  fn from_pixel(LumaPixel { y }: LumaPixel) -> Self {
    Self {
      b: y,
      g: y,
      r: y,
      a: u8::MAX,
    }
  }
}

impl FromPixel<LumaAlphaPixel> for BgrAlphaPixel {
  /// Converts a `LumaAlphaPixel` into a `BgrAlphaPixel`.
  fn from_pixel(LumaAlphaPixel { y, a }: LumaAlphaPixel) -> Self {
    Self {
      b: y,
      g: y,
      r: y,
      a,
    }
  }
}

impl FromPixel<RgbPixel> for BgrAlphaPixel {
  /// Converts an `RgbPixel` into a `BgrAlphaPixel`.
  fn from_pixel(RgbPixel { r, g, b }: RgbPixel) -> Self {
    let a = u8::MAX;
    Self { b, g, r, a }
  }
}

impl FromPixel<RgbAlphaPixel> for BgrAlphaPixel {
  /// Converts an `RgbAlphaPixel` into a `BgrAlphaPixel`.
  fn from_pixel(RgbAlphaPixel { r, g, b, a }: RgbAlphaPixel) -> Self {
    Self { b, g, r, a }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pixel_from_bgr_opaque() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(bgr::RED_OPAQUE),
      bgr::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_bgr_alpha_solid() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(bgr::RED_ALPHA_SOLID),
      bgr::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_bgr_alpha_trans() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(bgr::RED_ALPHA_TRANS),
      bgr::RED_ALPHA_TRANS
    )
  }

  #[test]
  fn pixel_from_luma_opaque() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(luma::GREY_OPAQUE),
      bgr::GREY_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_luma_alpha_solid() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(luma::GREY_ALPHA_SOLID),
      bgr::GREY_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_luma_alpha_trans() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(luma::GREY_ALPHA_TRANS),
      bgr::GREY_ALPHA_TRANS
    )
  }

  #[test]
  fn pixel_from_rgb_opaque() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(rgb::RED_OPAQUE),
      bgr::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_rgb_alpha_solid() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(rgb::RED_ALPHA_SOLID),
      bgr::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_rgb_alpha_trans() {
    assert_eq!(
      BgrAlphaPixel::from_pixel(rgb::RED_ALPHA_TRANS),
      bgr::RED_ALPHA_TRANS
    )
  }
}
