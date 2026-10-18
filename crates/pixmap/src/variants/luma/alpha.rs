use pixmap_core::{FromPixel, IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

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
  fn from_pixel(LumaPixel { y }: LumaPixel) -> Self {
    let a = u8::MAX;
    Self { y, a }
  }
}

impl FromPixel<RgbPixel> for LumaAlphaPixel {
  /// Converts an `RgbPixel` into a `LumaAlphaPixel`.
  fn from_pixel(RgbPixel { r, g, b }: RgbPixel) -> Self {
    let y = rgb_to_y(r, g, b);
    let a = u8::MAX;
    Self { y, a }
  }
}

impl FromPixel<RgbAlphaPixel> for LumaAlphaPixel {
  /// Converts an `RgbAlphaPixel` into a  `LumaAlphaPixel`.
  fn from_pixel(RgbAlphaPixel { r, g, b, a }: RgbAlphaPixel) -> Self {
    let y = rgb_to_y(r, g, b);
    Self { y, a }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pixel_from_luma_opaque() {
    assert_eq!(
      LumaAlphaPixel::from_pixel(luma::GREY_OPAQUE),
      luma::GREY_ALPHA_SOLID
    );
  }

  #[test]
  fn pixel_from_luma_alpha_solid() {
    assert_eq!(
      LumaAlphaPixel::from_pixel(luma::GREY_ALPHA_SOLID),
      luma::GREY_ALPHA_SOLID
    );
  }

  #[test]
  fn pixel_from_luma_alpha_trans() {
    assert_eq!(
      LumaAlphaPixel::from_pixel(luma::GREY_ALPHA_TRANS),
      luma::GREY_ALPHA_TRANS
    );
  }

  #[test]
  fn pixel_from_rgb_opaque() {
    assert_eq!(
      LumaAlphaPixel::from_pixel(rgb::RED_OPAQUE),
      luma::GREY_ALPHA_SOLID
    );
  }

  #[test]
  fn pixel_from_rgb_alpha_solid() {
    assert_eq!(
      LumaAlphaPixel::from_pixel(rgb::RED_ALPHA_SOLID),
      luma::GREY_ALPHA_SOLID
    );
  }

  #[test]
  fn pixel_from_rgb_alpha_trans() {
    assert_eq!(
      LumaAlphaPixel::from_pixel(rgb::RED_ALPHA_TRANS),
      luma::GREY_ALPHA_TRANS
    );
  }
}
