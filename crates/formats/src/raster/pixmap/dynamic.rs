use super::{
  IndexedPixmap, IntoPixmap, LumaAlphaPixmap, LumaPixmap, Pixmap,
  RgbAlphaPixmap, RgbPixmap,
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
  /// Returns the number of pixels in the pixmap.
  pub const fn pixels_len(&self) -> usize {
    match self {
      Self::Luma(v) => v.len(),
      Self::LumaAlpha(v) => v.len(),
      Self::Rgb(v) => v.len(),
      Self::RgbAlpha(v) => v.len(),
      Self::Indexed(v) => v.pixels.len(),
    }
  }
}

impl Pixmap for DynamicPixmap {
  fn concat(&self) -> Vec<u8> {
    match self {
      Self::Luma(v) => v.concat(),
      Self::LumaAlpha(v) => v.concat(),
      Self::Rgb(v) => v.concat(),
      Self::RgbAlpha(v) => v.concat(),
      Self::Indexed(v) => v.concat(),
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
