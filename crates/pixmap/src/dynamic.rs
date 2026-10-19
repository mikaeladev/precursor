use pixmap_core::{IntoPixmap, Pixel, Pixmap};
use pixmap_derive::Pixel;

use crate::variants::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicPixmap {
  Bgr(BgrPixmap),
  BgrAlpha(BgrAlphaPixmap),
  Indexed(IndexedPixmap),
  Luma(LumaPixmap),
  LumaAlpha(LumaAlphaPixmap),
  Rgb(RgbPixmap),
  RgbAlpha(RgbAlphaPixmap),
}

impl DynamicPixmap {
  /// Flattens the underlying vector of pixels into vector of bytes.
  ///
  /// This is the same as calling `into_iter().collect::<Vec<u8>>()` on the
  /// inner pixmap.
  pub fn flatten(self) -> Vec<u8> {
    match self {
      Self::Bgr(p) => p.into_iter().collect(),
      Self::BgrAlpha(p) => p.into_iter().collect(),
      Self::Indexed(p) => p.into_iter().collect(),
      Self::Luma(p) => p.into_iter().collect(),
      Self::LumaAlpha(p) => p.into_iter().collect(),
      Self::Rgb(p) => p.into_iter().collect(),
      Self::RgbAlpha(p) => p.into_iter().collect(),
    }
  }
}

macro_rules! unimplemented_method {
  ($($tt:tt)*) => {
    /// Not implemented for dynamic pixmaps; convert to a real pixmap first.
    ///
    /// # Panics
    ///
    /// This method will always panic with a message of "not implemented".
    $($tt)* {
      unimplemented!()
    }
  };
}

/// Zero-sized [`Pixel`] type.
///
/// This only exists to satisfy the [`Pixmap`] trait for [`DynamicPixmap`] and
/// any attempts to get or set with this will panic.
#[derive(Debug, Pixel, Clone, Copy)]
pub struct PhantomPixel {}

impl Pixmap for DynamicPixmap {
  type Pixel = PhantomPixel;

  fn width(&self) -> u32 {
    match self {
      Self::Bgr(p) => p.width,
      Self::BgrAlpha(p) => p.width,
      Self::Indexed(p) => p.width,
      Self::Luma(p) => p.width,
      Self::LumaAlpha(p) => p.width,
      Self::Rgb(p) => p.width,
      Self::RgbAlpha(p) => p.width,
    }
  }

  fn height(&self) -> u32 {
    match self {
      Self::Bgr(p) => p.height,
      Self::BgrAlpha(p) => p.height,
      Self::Indexed(p) => p.height,
      Self::Luma(p) => p.height,
      Self::LumaAlpha(p) => p.height,
      Self::Rgb(p) => p.height,
      Self::RgbAlpha(p) => p.height,
    }
  }

  unimplemented_method! { fn pixels(&self) -> &[Self::Pixel] }

  unimplemented_method! { fn pixels_mut(&mut self) -> &mut [Self::Pixel] }

  unimplemented_method! { fn into_pixels(self) -> Vec<Self::Pixel> }

  unimplemented_method! {
    fn get_pixel(&self, _: u32, _: u32) -> Option<&Self::Pixel>
  }

  unimplemented_method! {
    fn get_pixel_mut(&mut self, _: u32, _: u32) -> Option<&mut Self::Pixel>
  }

  unimplemented_method! {
    fn get_pixel_unchecked(&self, _: u32, _: u32) -> &Self::Pixel
  }

  unimplemented_method! {
    fn get_pixel_index(&self, _: u32, _: u32) -> Option<usize>
  }

  unimplemented_method! {
    fn get_pixel_index_unchecked(&self, _: u32, _: u32) -> usize
  }

  unimplemented_method! {
    fn set_pixel(&mut self, _: u32, _: u32, _: Self::Pixel)
  }

  fn scale_up(&mut self, factor: usize) {
    match self {
      Self::Bgr(p) => p.scale_up(factor),
      Self::BgrAlpha(p) => p.scale_up(factor),
      Self::Indexed(p) => p.scale_up(factor),
      Self::Luma(p) => p.scale_up(factor),
      Self::LumaAlpha(p) => p.scale_up(factor),
      Self::Rgb(p) => p.scale_up(factor),
      Self::RgbAlpha(p) => p.scale_up(factor),
    }
  }

  fn flip_horizontal(&mut self) {
    match self {
      Self::Bgr(p) => p.flip_horizontal(),
      Self::BgrAlpha(p) => p.flip_horizontal(),
      Self::Indexed(p) => p.flip_horizontal(),
      Self::Luma(p) => p.flip_horizontal(),
      Self::LumaAlpha(p) => p.flip_horizontal(),
      Self::Rgb(p) => p.flip_horizontal(),
      Self::RgbAlpha(p) => p.flip_horizontal(),
    }
  }

  fn flip_vertical(&mut self) {
    match self {
      Self::Bgr(p) => p.flip_vertical(),
      Self::BgrAlpha(p) => p.flip_vertical(),
      Self::Indexed(p) => p.flip_vertical(),
      Self::Luma(p) => p.flip_vertical(),
      Self::LumaAlpha(p) => p.flip_vertical(),
      Self::Rgb(p) => p.flip_vertical(),
      Self::RgbAlpha(p) => p.flip_vertical(),
    }
  }

  fn rotate_90(&mut self) {
    match self {
      Self::Bgr(p) => p.rotate_90(),
      Self::BgrAlpha(p) => p.rotate_90(),
      Self::Indexed(p) => p.rotate_90(),
      Self::Luma(p) => p.rotate_90(),
      Self::LumaAlpha(p) => p.rotate_90(),
      Self::Rgb(p) => p.rotate_90(),
      Self::RgbAlpha(p) => p.rotate_90(),
    }
  }

  fn rotate_180(&mut self) {
    match self {
      Self::Bgr(p) => p.rotate_180(),
      Self::BgrAlpha(p) => p.rotate_180(),
      Self::Indexed(p) => p.rotate_180(),
      Self::Luma(p) => p.rotate_180(),
      Self::LumaAlpha(p) => p.rotate_180(),
      Self::Rgb(p) => p.rotate_180(),
      Self::RgbAlpha(p) => p.rotate_180(),
    }
  }

  fn rotate_270(&mut self) {
    match self {
      Self::Bgr(p) => p.rotate_270(),
      Self::BgrAlpha(p) => p.rotate_270(),
      Self::Indexed(p) => p.rotate_270(),
      Self::Luma(p) => p.rotate_270(),
      Self::LumaAlpha(p) => p.rotate_270(),
      Self::Rgb(p) => p.rotate_270(),
      Self::RgbAlpha(p) => p.rotate_270(),
    }
  }
}

impl IntoPixmap<BgrPixmap> for DynamicPixmap {
  /// Converts the value into a [`BgrPixmap`].
  fn into_pixmap(self) -> BgrPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}

impl IntoPixmap<BgrAlphaPixmap> for DynamicPixmap {
  /// Converts the value into a [`BgrAlphaPixmap`].
  fn into_pixmap(self) -> BgrAlphaPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}

impl IntoPixmap<IndexedPixmap> for DynamicPixmap {
  /// Converts the value into an [`IndexedPixmap`].
  fn into_pixmap(self) -> IndexedPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}

impl IntoPixmap<LumaPixmap> for DynamicPixmap {
  /// Converts the value into a [`LumaPixmap`].
  fn into_pixmap(self) -> LumaPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}

impl IntoPixmap<LumaAlphaPixmap> for DynamicPixmap {
  /// Converts the value into a [`LumaAlphaPixmap`].
  fn into_pixmap(self) -> LumaAlphaPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}

impl IntoPixmap<RgbPixmap> for DynamicPixmap {
  /// Converts the value into an [`RgbPixmap`].
  fn into_pixmap(self) -> RgbPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}

impl IntoPixmap<RgbAlphaPixmap> for DynamicPixmap {
  /// Converts the value into an [`RgbAlphaPixmap`].
  fn into_pixmap(self) -> RgbAlphaPixmap {
    match self {
      Self::Bgr(p) => p.into_pixmap(),
      Self::BgrAlpha(p) => p.into_pixmap(),
      Self::Indexed(p) => p.into_pixmap(),
      Self::Luma(p) => p.into_pixmap(),
      Self::LumaAlpha(p) => p.into_pixmap(),
      Self::Rgb(p) => p.into_pixmap(),
      Self::RgbAlpha(p) => p.into_pixmap(),
    }
  }
}
