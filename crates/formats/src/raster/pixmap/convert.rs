use indexmap::IndexSet;

use super::{
  IndexedPixmap, IntoNonPalettePixel, LumaAlphaPixel, LumaAlphaPixmap,
  LumaPixmap, PaletteIndex, RgbAlphaPixel, RgbAlphaPixmap, RgbPixmap,
};

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
    LumaAlphaPixmap {
      width: self.width,
      height: self.width,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| p.into_luma_alpha())
        .collect(),
    }
  }

  fn into_rgb(self) -> RgbPixmap {
    RgbPixmap {
      width: self.width,
      height: self.width,
      pixels: self.pixels.into_iter().map(|p| p.into_rgb()).collect(),
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    RgbAlphaPixmap {
      width: self.width,
      height: self.width,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| p.into_rgb_alpha())
        .collect(),
    }
  }

  fn into_indexed(self) -> IndexedPixmap {
    let mut palette = IndexSet::with_capacity(u8::MAX as usize);
    let mut pixels = Vec::with_capacity(self.pixels.len());

    for pixel in self.pixels {
      let (index, _) = palette.insert_full(pixel.into_rgb());
      pixels.push(PaletteIndex { i: index as u8 });
    }

    let palette = palette.into_iter().collect();

    IndexedPixmap {
      width: self.width,
      height: self.height,
      pixels,
      palette,
      trns: None,
    }
  }
}

impl IntoPixmap for LumaAlphaPixmap {
  fn into_luma(self) -> LumaPixmap {
    LumaPixmap {
      width: self.width,
      height: self.width,
      pixels: self.pixels.into_iter().map(|p| p.into_luma()).collect(),
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    self
  }

  fn into_rgb(self) -> RgbPixmap {
    RgbPixmap {
      width: self.width,
      height: self.width,
      pixels: self.pixels.into_iter().map(|p| p.into_rgb()).collect(),
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    RgbAlphaPixmap {
      width: self.width,
      height: self.width,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| p.into_rgb_alpha())
        .collect(),
    }
  }

  fn into_indexed(self) -> IndexedPixmap {
    self.into_rgb_alpha().into_indexed()
  }
}

impl IntoPixmap for RgbPixmap {
  fn into_luma(self) -> LumaPixmap {
    LumaPixmap {
      width: self.width,
      height: self.width,
      pixels: self.pixels.into_iter().map(|p| p.into_luma()).collect(),
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    LumaAlphaPixmap {
      width: self.width,
      height: self.width,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| p.into_luma_alpha())
        .collect(),
    }
  }

  fn into_rgb(self) -> RgbPixmap {
    self
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    RgbAlphaPixmap {
      width: self.width,
      height: self.width,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| p.into_rgb_alpha())
        .collect(),
    }
  }

  fn into_indexed(self) -> IndexedPixmap {
    let mut palette = IndexSet::with_capacity(u8::MAX as usize);
    let mut pixels = Vec::with_capacity(self.pixels.len());

    for pixel in self.pixels {
      let (index, _) = palette.insert_full(pixel);
      pixels.push(PaletteIndex { i: index as u8 });
    }

    let palette = palette.into_iter().collect();

    IndexedPixmap {
      width: self.width,
      height: self.height,
      pixels,
      palette,
      trns: None,
    }
  }
}

impl IntoPixmap for RgbAlphaPixmap {
  fn into_luma(self) -> LumaPixmap {
    LumaPixmap {
      width: self.width,
      height: self.width,
      pixels: self.pixels.into_iter().map(|p| p.into_luma()).collect(),
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    LumaAlphaPixmap {
      width: self.width,
      height: self.width,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| p.into_luma_alpha())
        .collect(),
    }
  }

  fn into_rgb(self) -> RgbPixmap {
    RgbPixmap {
      width: self.width,
      height: self.width,
      pixels: self.pixels.into_iter().map(|p| p.into_rgb()).collect(),
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    self
  }

  fn into_indexed(self) -> IndexedPixmap {
    let mut palette = IndexSet::with_capacity(u8::MAX as usize);
    let mut trns = Vec::with_capacity(u8::MAX as usize);
    let mut pixels = Vec::with_capacity(self.pixels.len());

    let mut sorted = self.pixels.to_vec();
    sorted.sort_by(|a, b| a.a.cmp(&b.a));

    for pixel in sorted {
      let alpha = pixel.a;

      let (_, exists) = palette.insert_full(pixel.into_rgb());

      if alpha != u8::MAX && !exists {
        trns.push(alpha);
      }
    }

    for pixel in self.pixels {
      let index = palette.get_index_of(&pixel.into_rgb()).unwrap();
      pixels.push(PaletteIndex { i: index as u8 });
    }

    let palette = palette.into_iter().collect();
    let trns = Some(trns);

    IndexedPixmap {
      width: self.width,
      height: self.height,
      pixels,
      palette,
      trns,
    }
  }
}

impl IntoPixmap for IndexedPixmap {
  fn into_luma(self) -> LumaPixmap {
    drop(self.trns);

    let palette: Vec<_> =
      self.palette.into_iter().map(|p| p.into_luma()).collect();

    LumaPixmap {
      width: self.width,
      height: self.height,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| palette[p.i as usize])
        .collect(),
    }
  }

  fn into_luma_alpha(self) -> LumaAlphaPixmap {
    let palette: Vec<_> =
      self.palette.into_iter().map(|p| p.into_luma()).collect();

    let pixels_iter = self.pixels.into_iter().map(|p| {
      let index = p.i as usize;

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

    LumaAlphaPixmap {
      width: self.width,
      height: self.height,
      pixels: pixels_iter.collect(),
    }
  }

  fn into_rgb(self) -> RgbPixmap {
    drop(self.trns);

    RgbPixmap {
      width: self.width,
      height: self.height,
      pixels: self
        .pixels
        .into_iter()
        .map(|p| self.palette[p.i as usize])
        .collect(),
    }
  }

  fn into_rgb_alpha(self) -> RgbAlphaPixmap {
    let pixels_iter = self.pixels.into_iter().map(|p| {
      let index = p.i as usize;

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

    RgbAlphaPixmap {
      width: self.width,
      height: self.height,
      pixels: pixels_iter.collect(),
    }
  }

  fn into_indexed(self) -> IndexedPixmap {
    self
  }
}
