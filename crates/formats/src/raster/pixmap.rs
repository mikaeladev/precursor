use indexmap::IndexSet;
use png::ColorType;

use super::{
  GrayscaleAlphaPixel, GrayscalePixel, IntoPixel, RgbAlphaPixel, RgbPixel,
};

/// Packed array of `GrayscalePixel`s.
pub type GrayscalePixmap = Vec<GrayscalePixel>;

/// Packed array of `GrayscaleAlphaPixel`s.
pub type GrayscaleAlphaPixmap = Vec<GrayscaleAlphaPixel>;

/// Packed array of `RgbPixel`s.
pub type RgbPixmap = Vec<RgbPixel>;

/// Packed array of `RgbAlphaPixel`s.
pub type RgbAlphaPixmap = Vec<RgbAlphaPixel>;

#[derive(Debug, Clone)]
pub struct IndexedPixmap {
  /// Array of `RgbPixel`s.
  pub palette: Vec<RgbPixel>,
  /// Optional array of alpha values corresponding to `RgbPixel`s in `palette`.
  ///
  /// Missing values are assumed to be opaque (`255`).
  pub trns: Option<Vec<u8>>,
  /// Packed array of palette indexes.
  pub pixels: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Pixmap {
  Grayscale(GrayscalePixmap),
  GrayscaleAlpha(GrayscaleAlphaPixmap),
  Rgb(RgbPixmap),
  RgbAlpha(RgbAlphaPixmap),
  Indexed(IndexedPixmap),
}

impl Pixmap {
  /// Returns the number of pixels in the pixmap.
  pub const fn pixels_len(&self) -> usize {
    match self {
      Self::Grayscale(v) => v.len(),
      Self::GrayscaleAlpha(v) => v.len(),
      Self::Rgb(v) => v.len(),
      Self::RgbAlpha(v) => v.len(),
      Self::Indexed(v) => v.pixels.len(),
    }
  }

  /// Returns the associated PNG [`ColorType`].
  pub const fn color_type(&self) -> ColorType {
    match self {
      Self::Grayscale(_) => ColorType::Grayscale,
      Self::GrayscaleAlpha(_) => ColorType::GrayscaleAlpha,
      Self::Rgb(_) => ColorType::Rgb,
      Self::RgbAlpha(_) => ColorType::Rgba,
      Self::Indexed(_) => ColorType::Indexed,
    }
  }
}

pub trait IntoPixmap {
  /// Converts the value to a `GrayscalePixmap`.
  fn into_grayscale(self) -> GrayscalePixmap;

  /// Converts the value to a `GrayscaleAlphaPixmap`.
  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap;

  /// Converts the value to an `RgbPixmap`.
  fn into_rgb(self) -> RgbPixmap;

  /// Converts the value to an `RgbAlphaPixmap`.
  fn into_rgb_alpha(self) -> RgbAlphaPixmap;

  /// Converts the value to an `IndexedPixmap`.
  fn into_indexed(self) -> IndexedPixmap;
}

impl IntoPixmap for GrayscalePixmap {
  fn into_grayscale(self) -> GrayscalePixmap {
    self
  }

  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap {
    self.into_iter().map(|p| p.into_grayscale_alpha()).collect()
  }

  fn into_rgb(self) -> RgbPixmap {
    self.into_iter().map(|p| p.into_rgb()).collect()
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    self.into_iter().map(|p| p.into_rgb_alpha()).collect()
  }

  fn into_indexed(self) -> IndexedPixmap {
    let mut palette = IndexSet::with_capacity(u8::MAX as usize);
    let mut pixels = Vec::with_capacity(self.len());

    for pixel in self {
      let (index, _) = palette.insert_full(pixel.into_rgb());
      pixels.push(index as u8);
    }

    let palette = palette.into_iter().collect();

    IndexedPixmap {
      palette,
      trns: None,
      pixels,
    }
  }
}

impl IntoPixmap for GrayscaleAlphaPixmap {
  fn into_grayscale(self) -> GrayscalePixmap {
    self.into_iter().map(|p| p.into_grayscale()).collect()
  }

  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap {
    self
  }

  fn into_rgb(self) -> RgbPixmap {
    self.into_iter().map(|p| p.into_rgb()).collect()
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    self.into_iter().map(|p| p.into_rgb_alpha()).collect()
  }

  fn into_indexed(self) -> IndexedPixmap {
    self.into_rgb_alpha().into_indexed()
  }
}

impl IntoPixmap for RgbPixmap {
  fn into_grayscale(self) -> GrayscalePixmap {
    self.into_iter().map(|p| p.into_grayscale()).collect()
  }

  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap {
    self.into_iter().map(|p| p.into_grayscale_alpha()).collect()
  }

  fn into_rgb(self) -> RgbPixmap {
    self
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    self.into_iter().map(|p| p.into_rgb_alpha()).collect()
  }

  fn into_indexed(self) -> IndexedPixmap {
    let mut palette = IndexSet::with_capacity(u8::MAX as usize);
    let mut pixels = Vec::with_capacity(self.len());

    for pixel in self {
      let (index, _) = palette.insert_full(pixel);
      pixels.push(index as u8);
    }

    let palette = palette.into_iter().collect();

    IndexedPixmap {
      palette,
      trns: None,
      pixels,
    }
  }
}

impl IntoPixmap for RgbAlphaPixmap {
  fn into_grayscale(self) -> GrayscalePixmap {
    self.into_iter().map(|p| p.into_grayscale()).collect()
  }

  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap {
    self.into_iter().map(|p| p.into_grayscale_alpha()).collect()
  }

  fn into_rgb(self) -> RgbPixmap {
    self.into_iter().map(|p| p.into_rgb()).collect()
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    self
  }

  fn into_indexed(self) -> IndexedPixmap {
    let mut palette = IndexSet::with_capacity(u8::MAX as usize);
    let mut trns = Vec::with_capacity(u8::MAX as usize);
    let mut pixels = Vec::with_capacity(self.len());

    let mut sorted = self.to_vec();
    sorted.sort_by(|a, b| a.a.cmp(&b.a));

    for pixel in sorted {
      let alpha = pixel.a;

      let (_, exists) = palette.insert_full(pixel.into_rgb());

      if alpha != u8::MAX && !exists {
        trns.push(alpha);
      }
    }

    for pixel in self {
      let index = palette.get_index_of(&pixel.into_rgb()).unwrap();
      pixels.push(index as u8);
    }

    let palette = palette.into_iter().collect();
    let trns = Some(trns);

    IndexedPixmap {
      palette,
      trns,
      pixels,
    }
  }
}

impl IntoPixmap for IndexedPixmap {
  fn into_grayscale(self) -> GrayscalePixmap {
    drop(self.trns);

    let palette: Vec<_> = self
      .palette
      .into_iter()
      .map(|p| p.into_grayscale())
      .collect();

    self
      .pixels
      .into_iter()
      .map(|i| palette[i as usize])
      .collect()
  }

  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap {
    let palette: Vec<_> = self
      .palette
      .into_iter()
      .map(|p| p.into_grayscale())
      .collect();

    let pixels_iter = self.pixels.into_iter().map(|i| {
      let index = i as usize;

      let alpha = if let Some(trns) = &self.trns
        && let Some(alpha) = trns.get(index)
      {
        *alpha
      } else {
        u8::MAX
      };

      GrayscaleAlphaPixel {
        y: palette[index].y,
        a: alpha,
      }
    });

    pixels_iter.collect()
  }

  fn into_rgb(self) -> RgbPixmap {
    drop(self.trns);

    self
      .pixels
      .into_iter()
      .map(|i| self.palette[i as usize])
      .collect()
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    let pixels_iter = self.pixels.into_iter().map(|i| {
      let index = i as usize;

      let rgb = self.palette[index];

      let alpha = if let Some(trns) = &self.trns
        && let Some(alpha) = trns.get(index)
      {
        *alpha
      } else {
        u8::MAX
      };

      RgbAlphaPixel {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
        a: alpha,
      }
    });

    pixels_iter.collect()
  }

  fn into_indexed(self) -> IndexedPixmap {
    self
  }
}

impl IntoPixmap for Pixmap {
  fn into_grayscale(self) -> GrayscalePixmap {
    match self {
      Self::Grayscale(v) => v.into_grayscale(),
      Self::GrayscaleAlpha(v) => v.into_grayscale(),
      Self::Rgb(v) => v.into_grayscale(),
      Self::RgbAlpha(v) => v.into_grayscale(),
      Self::Indexed(v) => v.into_grayscale(),
    }
  }

  fn into_grayscale_alpha(self) -> GrayscaleAlphaPixmap {
    match self {
      Self::Grayscale(v) => v.into_grayscale_alpha(),
      Self::GrayscaleAlpha(v) => v.into_grayscale_alpha(),
      Self::Rgb(v) => v.into_grayscale_alpha(),
      Self::RgbAlpha(v) => v.into_grayscale_alpha(),
      Self::Indexed(v) => v.into_grayscale_alpha(),
    }
  }

  fn into_rgb(self) -> RgbPixmap {
    match self {
      Self::Grayscale(v) => v.into_rgb(),
      Self::GrayscaleAlpha(v) => v.into_rgb(),
      Self::Rgb(v) => v.into_rgb(),
      Self::RgbAlpha(v) => v.into_rgb(),
      Self::Indexed(v) => v.into_rgb(),
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    match self {
      Self::Grayscale(v) => v.into_rgb_alpha(),
      Self::GrayscaleAlpha(v) => v.into_rgb_alpha(),
      Self::Rgb(v) => v.into_rgb_alpha(),
      Self::RgbAlpha(v) => v.into_rgb_alpha(),
      Self::Indexed(v) => v.into_rgb_alpha(),
    }
  }

  fn into_indexed(self) -> IndexedPixmap {
    match self {
      Self::Grayscale(v) => v.into_indexed(),
      Self::GrayscaleAlpha(v) => v.into_indexed(),
      Self::Rgb(v) => v.into_indexed(),
      Self::RgbAlpha(v) => v.into_indexed(),
      Self::Indexed(v) => v.into_indexed(),
    }
  }
}

pub trait ConcatPixmap {
  /// Concatenates the pixels into a new `Vec`.
  fn concat(&self) -> Vec<u8>;
}

impl ConcatPixmap for GrayscalePixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().map(|p| p.y).collect()
  }
}

impl ConcatPixmap for GrayscaleAlphaPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().flat_map(|p| p.into_iter()).collect()
  }
}

impl ConcatPixmap for RgbPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().flat_map(|p| p.into_iter()).collect()
  }
}

impl ConcatPixmap for RgbAlphaPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().flat_map(|p| p.into_iter()).collect()
  }
}

impl ConcatPixmap for IndexedPixmap {
  fn concat(&self) -> Vec<u8> {
    self.pixels.to_vec()
  }
}

impl ConcatPixmap for Pixmap {
  fn concat(&self) -> Vec<u8> {
    match self {
      Self::Grayscale(v) => v.concat(),
      Self::GrayscaleAlpha(v) => v.concat(),
      Self::Rgb(v) => v.concat(),
      Self::RgbAlpha(v) => v.concat(),
      Self::Indexed(v) => v.concat(),
    }
  }
}
