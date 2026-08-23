mod dynamic;
mod pixel;

use indexmap::IndexSet;

pub use dynamic::*;
pub use pixel::*;

/// Packed array of greyscale pixels.
pub type LumaPixmap = Vec<LumaPixel>;

/// Packed array of greyscale alpha pixels.
pub type LumaAlphaPixmap = Vec<LumaAlphaPixel>;

/// Packed array of RGB pixels.
pub type RgbPixmap = Vec<RgbPixel>;

/// Packed array of RGB alpha pixels.
pub type RgbAlphaPixmap = Vec<RgbAlphaPixel>;

#[derive(Debug, Clone)]
pub struct IndexedPixmap {
  /// Array of `RgbPixel`s.
  pub palette: Vec<RgbPixel>,
  /// Optional array of alpha values corresponding to pixels in `palette`.
  ///
  /// Missing values are assumed to be opaque (`255`).
  pub trns: Option<Vec<u8>>,
  /// Packed array of palette indexes.
  pub pixels: Vec<u8>,
}

pub trait Pixmap {
  /// Concatenates the pixels into a new `Vec`.
  fn concat(&self) -> Vec<u8>;
}

impl Pixmap for LumaPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().map(|p| p.y).collect()
  }
}

impl Pixmap for LumaAlphaPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().flat_map(|p| p.into_iter()).collect()
  }
}

impl Pixmap for RgbPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().flat_map(|p| p.into_iter()).collect()
  }
}

impl Pixmap for RgbAlphaPixmap {
  fn concat(&self) -> Vec<u8> {
    self.iter().flat_map(|p| p.into_iter()).collect()
  }
}

impl Pixmap for IndexedPixmap {
  fn concat(&self) -> Vec<u8> {
    self.pixels.to_vec()
  }
}

pub trait IntoPixmap {
  /// Converts the value to a `LumaPixmap`.
  fn into_luma(self) -> LumaPixmap;

  /// Converts the value to a `LumaAlphaPixmap`.
  fn into_luma_alpha(self) -> LumaAlphaPixmap;

  /// Converts the value to an `RgbPixmap`.
  fn into_rgb(self) -> RgbPixmap;

  /// Converts the value to an `RgbAlphaPixmap`.
  fn into_rgb_alpha(self) -> RgbAlphaPixmap;

  /// Converts the value to an `IndexedPixmap`.
  fn into_indexed(self) -> IndexedPixmap;
}

impl IntoPixmap for LumaPixmap {
  fn into_luma(self) -> LumaPixmap {
    self
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    self.into_iter().map(|p| p.into_luma_alpha()).collect()
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

impl IntoPixmap for LumaAlphaPixmap {
  fn into_luma(self) -> LumaPixmap {
    self.into_iter().map(|p| p.into_luma()).collect()
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
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
  fn into_luma(self) -> LumaPixmap {
    self.into_iter().map(|p| p.into_luma()).collect()
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    self.into_iter().map(|p| p.into_luma_alpha()).collect()
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
  fn into_luma(self) -> LumaPixmap {
    self.into_iter().map(|p| p.into_luma()).collect()
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    self.into_iter().map(|p| p.into_luma_alpha()).collect()
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
  fn into_luma(self) -> LumaPixmap {
    drop(self.trns);

    let palette: Vec<_> =
      self.palette.into_iter().map(|p| p.into_luma()).collect();

    self
      .pixels
      .into_iter()
      .map(|i| palette[i as usize])
      .collect()
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    let palette: Vec<_> =
      self.palette.into_iter().map(|p| p.into_luma()).collect();

    let pixels_iter = self.pixels.into_iter().map(|i| {
      let index = i as usize;

      let alpha = if let Some(trns) = &self.trns
        && let Some(alpha) = trns.get(index)
      {
        *alpha
      } else {
        u8::MAX
      };

      LumaAlphaPixel {
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
