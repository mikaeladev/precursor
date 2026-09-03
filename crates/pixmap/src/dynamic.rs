use crate::{
  IndexedPixmap, IntoPixmap, LumaAlphaPixmap, LumaPixmap, Pixmap,
  RgbAlphaPixmap, RgbPixmap,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

  /// Returns the width and height of the pixmap as a tuple.
  pub const fn dimensions(&self) -> (u32, u32) {
    (self.width(), self.height())
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

impl IntoPixmap<LumaPixmap> for DynamicPixmap {
  /// Converts the value into a `LumaPixmap`.
  fn into_pixmap(self) -> LumaPixmap {
    match self {
      Self::Luma(v) => v.into_pixmap(),
      Self::LumaAlpha(v) => v.into_pixmap(),
      Self::Rgb(v) => v.into_pixmap(),
      Self::RgbAlpha(v) => v.into_pixmap(),
      Self::Indexed(v) => v.into_pixmap(),
    }
  }
}

impl IntoPixmap<LumaAlphaPixmap> for DynamicPixmap {
  /// Converts the value into a `LumaAlphaPixmap`.
  fn into_pixmap(self) -> LumaAlphaPixmap {
    match self {
      Self::Luma(v) => v.into_pixmap(),
      Self::LumaAlpha(v) => v.into_pixmap(),
      Self::Rgb(v) => v.into_pixmap(),
      Self::RgbAlpha(v) => v.into_pixmap(),
      Self::Indexed(v) => v.into_pixmap(),
    }
  }
}

impl IntoPixmap<RgbPixmap> for DynamicPixmap {
  /// Converts the value into a `RgbPixmap`.
  fn into_pixmap(self) -> RgbPixmap {
    match self {
      Self::Luma(v) => v.into_pixmap(),
      Self::LumaAlpha(v) => v.into_pixmap(),
      Self::Rgb(v) => v.into_pixmap(),
      Self::RgbAlpha(v) => v.into_pixmap(),
      Self::Indexed(v) => v.into_pixmap(),
    }
  }
}

impl IntoPixmap<RgbAlphaPixmap> for DynamicPixmap {
  /// Converts the value into a `RgbAlphaPixmap`.
  fn into_pixmap(self) -> RgbAlphaPixmap {
    match self {
      Self::Luma(v) => v.into_pixmap(),
      Self::LumaAlpha(v) => v.into_pixmap(),
      Self::Rgb(v) => v.into_pixmap(),
      Self::RgbAlpha(v) => v.into_pixmap(),
      Self::Indexed(v) => v.into_pixmap(),
    }
  }
}

impl IntoPixmap<IndexedPixmap> for DynamicPixmap {
  /// Converts the value into a `IndexedPixmap`.
  fn into_pixmap(self) -> IndexedPixmap {
    match self {
      Self::Luma(v) => v.into_pixmap(),
      Self::LumaAlpha(v) => v.into_pixmap(),
      Self::Rgb(v) => v.into_pixmap(),
      Self::RgbAlpha(v) => v.into_pixmap(),
      Self::Indexed(v) => v.into_pixmap(),
    }
  }
}
