use pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

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
  BgrPixmap,
  LumaPixmap,
  "Converts a [`BgrPixmap`] into a [`LumaPixmap`]."
);

impl_from_paletted_for_paletted!(
  BgrAlphaPixmap,
  LumaPixmap,
  "Converts a [`BgrAlphaPixmap`] into a [`LumaPixmap`]."
);

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

impl FromPixel<BgrPixel> for LumaPixel {
  /// Converts a `BgrPixel` into a `LumaPixel`.
  fn from_pixel(BgrPixel { b, g, r }: BgrPixel) -> Self {
    let y = rgb_to_y(r, g, b);
    Self { y }
  }
}

impl FromPixel<BgrAlphaPixel> for LumaPixel {
  /// Converts a `BgrAlphaPixel` into a `LumaPixel`.
  fn from_pixel(BgrAlphaPixel { b, g, r, a: _ }: BgrAlphaPixel) -> Self {
    let y = rgb_to_y(r, g, b);
    Self { y }
  }
}

impl FromPixel<LumaAlphaPixel> for LumaPixel {
  /// Converts a `LumaAlphaPixel` into a `LumaPixel`.
  fn from_pixel(LumaAlphaPixel { y, a: _ }: LumaAlphaPixel) -> Self {
    Self { y }
  }
}

impl FromPixel<RgbPixel> for LumaPixel {
  /// Converts an `RgbPixel` into a `LumaPixel`.
  fn from_pixel(RgbPixel { r, g, b }: RgbPixel) -> Self {
    let y = rgb_to_y(r, g, b);
    Self { y }
  }
}

impl FromPixel<RgbAlphaPixel> for LumaPixel {
  /// Converts an `RgbAlphaPixel` into a `LumaPixel`.
  fn from_pixel(RgbAlphaPixel { r, g, b, a: _ }: RgbAlphaPixel) -> Self {
    let y = rgb_to_y(r, g, b);
    Self { y }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pixel_from_bgr_opaque() {
    assert_eq!(LumaPixel::from_pixel(bgr::RED_OPAQUE), luma::GREY_OPAQUE);
  }

  #[test]
  fn pixel_from_bgr_alpha_solid() {
    assert_eq!(
      LumaPixel::from_pixel(bgr::RED_ALPHA_SOLID),
      luma::GREY_OPAQUE
    );
  }

  #[test]
  fn pixel_from_bgr_alpha_trans() {
    assert_eq!(
      LumaPixel::from_pixel(bgr::RED_ALPHA_TRANS),
      luma::GREY_OPAQUE
    );
  }

  #[test]
  fn pixel_from_luma_opaque() {
    assert_eq!(LumaPixel::from_pixel(luma::GREY_OPAQUE), luma::GREY_OPAQUE);
  }

  #[test]
  fn pixel_from_luma_alpha_solid() {
    assert_eq!(
      LumaPixel::from_pixel(luma::GREY_ALPHA_SOLID),
      luma::GREY_OPAQUE
    );
  }

  #[test]
  fn pixel_from_luma_alpha_trans() {
    assert_eq!(
      LumaPixel::from_pixel(luma::GREY_ALPHA_TRANS),
      luma::GREY_OPAQUE
    );
  }

  #[test]
  fn pixel_from_rgb_opaque() {
    assert_eq!(LumaPixel::from_pixel(rgb::RED_OPAQUE), luma::GREY_OPAQUE);
  }

  #[test]
  fn pixel_from_rgb_alpha_solid() {
    assert_eq!(
      LumaPixel::from_pixel(rgb::RED_ALPHA_SOLID),
      luma::GREY_OPAQUE
    );
  }

  #[test]
  fn pixel_from_rgb_alpha_trans() {
    assert_eq!(
      LumaPixel::from_pixel(rgb::RED_ALPHA_TRANS),
      luma::GREY_OPAQUE
    );
  }
}
