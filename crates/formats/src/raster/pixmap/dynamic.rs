use crate::raster::Pixmap;

use super::{
  IndexedPixmap, IntoPixmap, LumaAlphaPixmap, LumaPixmap, RgbAlphaPixmap,
  RgbPixmap,
};

#[derive(Debug, Clone)]
pub enum DynamicPixmap {
  Luma(LumaPixmap),
  LumaAlpha(LumaAlphaPixmap),
  Rgb(RgbPixmap),
  RgbAlpha(RgbAlphaPixmap),
  Indexed(IndexedPixmap),
}

impl DynamicPixmap {
  /// Returns the width of the pixmap.
  pub const fn width(&self) -> u32 {
    match self {
      Self::Luma(p) => p.width,
      Self::LumaAlpha(p) => p.width,
      Self::Rgb(p) => p.width,
      Self::RgbAlpha(p) => p.width,
      Self::Indexed(p) => p.width,
    }
  }

  /// Returns the height of the pixmap.
  pub const fn height(&self) -> u32 {
    match self {
      Self::Luma(p) => p.height,
      Self::LumaAlpha(p) => p.height,
      Self::Rgb(p) => p.height,
      Self::RgbAlpha(p) => p.height,
      Self::Indexed(p) => p.height,
    }
  }

  /// Copies and concatenates the pixels into a new `Vec<u8>`.
  pub fn pixels_concat(&self) -> Vec<u8> {
    match self {
      Self::Luma(p) => p.pixels_concat(),
      Self::LumaAlpha(p) => p.pixels_concat(),
      Self::Rgb(p) => p.pixels_concat(),
      Self::RgbAlpha(p) => p.pixels_concat(),
      Self::Indexed(p) => p.pixels_concat(),
    }
  }
}

impl IntoPixmap for DynamicPixmap {
  fn into_luma(self) -> LumaPixmap {
    match self {
      Self::Luma(v) => v.into_luma(),
      Self::LumaAlpha(v) => v.into_luma(),
      Self::Rgb(v) => v.into_luma(),
      Self::RgbAlpha(v) => v.into_luma(),
      Self::Indexed(v) => v.into_luma(),
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    match self {
      Self::Luma(v) => v.into_luma_alpha(),
      Self::LumaAlpha(v) => v.into_luma_alpha(),
      Self::Rgb(v) => v.into_luma_alpha(),
      Self::RgbAlpha(v) => v.into_luma_alpha(),
      Self::Indexed(v) => v.into_luma_alpha(),
    }
  }

  fn into_rgb(self) -> RgbPixmap {
    match self {
      Self::Luma(v) => v.into_rgb(),
      Self::LumaAlpha(v) => v.into_rgb(),
      Self::Rgb(v) => v.into_rgb(),
      Self::RgbAlpha(v) => v.into_rgb(),
      Self::Indexed(v) => v.into_rgb(),
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    match self {
      Self::Luma(v) => v.into_rgb_alpha(),
      Self::LumaAlpha(v) => v.into_rgb_alpha(),
      Self::Rgb(v) => v.into_rgb_alpha(),
      Self::RgbAlpha(v) => v.into_rgb_alpha(),
      Self::Indexed(v) => v.into_rgb_alpha(),
    }
  }

  fn into_indexed(self) -> IndexedPixmap {
    match self {
      Self::Luma(v) => v.into_indexed(),
      Self::LumaAlpha(v) => v.into_indexed(),
      Self::Rgb(v) => v.into_indexed(),
      Self::RgbAlpha(v) => v.into_indexed(),
      Self::Indexed(v) => v.into_indexed(),
    }
  }
}
