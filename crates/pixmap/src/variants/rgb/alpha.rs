use pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

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
  BgrPixmap,
  RgbAlphaPixmap,
  "Converts a [`BgrPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_paletted_for_paletted!(
  BgrAlphaPixmap,
  RgbAlphaPixmap,
  "Converts a [`BgrAlphaPixmap`] into an [`RgbAlphaPixmap`]."
);

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

impl FromPixel<BgrPixel> for RgbAlphaPixel {
  /// Converts a `BgrPixel` into an `RgbAlphaPixel`.
  fn from_pixel(BgrPixel { b, g, r }: BgrPixel) -> Self {
    let a = u8::MAX;
    Self { r, g, b, a }
  }
}

impl FromPixel<BgrAlphaPixel> for RgbAlphaPixel {
  /// Converts a `BgrAlphaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(BgrAlphaPixel { b, g, r, a }: BgrAlphaPixel) -> Self {
    Self { r, g, b, a }
  }
}

impl FromPixel<LumaPixel> for RgbAlphaPixel {
  /// Converts a `LumaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(LumaPixel { y }: LumaPixel) -> Self {
    Self {
      r: y,
      g: y,
      b: y,
      a: u8::MAX,
    }
  }
}

impl FromPixel<LumaAlphaPixel> for RgbAlphaPixel {
  /// Converts a `LumaAlphaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(LumaAlphaPixel { y, a }: LumaAlphaPixel) -> Self {
    Self {
      r: y,
      g: y,
      b: y,
      a,
    }
  }
}

impl FromPixel<RgbPixel> for RgbAlphaPixel {
  /// Converts an `RgbPixel` into an `RgbAlphaPixel`.
  fn from_pixel(RgbPixel { r, g, b }: RgbPixel) -> Self {
    let a = u8::MAX;
    Self { r, g, b, a }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pixel_from_bgr_opaque() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(bgr::RED_OPAQUE),
      rgb::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_bgr_alpha_solid() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(bgr::RED_ALPHA_SOLID),
      rgb::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_bgr_alpha_trans() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(bgr::RED_ALPHA_TRANS),
      rgb::RED_ALPHA_TRANS
    )
  }

  #[test]
  fn pixel_from_luma_opaque() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(luma::GREY_OPAQUE),
      rgb::GREY_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_luma_alpha_solid() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(luma::GREY_ALPHA_SOLID),
      rgb::GREY_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_luma_alpha_trans() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(luma::GREY_ALPHA_TRANS),
      rgb::GREY_ALPHA_TRANS
    )
  }

  #[test]
  fn pixel_from_rgb_opaque() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(rgb::RED_OPAQUE),
      rgb::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_rgb_alpha_solid() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(rgb::RED_ALPHA_SOLID),
      rgb::RED_ALPHA_SOLID
    )
  }

  #[test]
  fn pixel_from_rgb_alpha_trans() {
    assert_eq!(
      RgbAlphaPixel::from_pixel(rgb::RED_ALPHA_TRANS),
      rgb::RED_ALPHA_TRANS
    )
  }
}
