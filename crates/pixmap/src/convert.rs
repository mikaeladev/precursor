use indexmap::IndexSet;

use crate::{
  FromPixel, IndexedPixel, IndexedPixmap, IntoPixel, LumaAlphaPixmap,
  LumaPixmap, Pixmap, RgbAlphaPixmap, RgbPixel, RgbPixmap,
};

pub trait FromPixmap<P: Pixmap> {
  /// Converts the pixmap into this type.
  fn from_pixmap(pixmap: P) -> Self;
}

pub trait IntoPixmap<P: Pixmap> {
  /// Converts this value into a pixmap.
  fn into_pixmap(self) -> P;
}

impl<T: Pixmap> FromPixmap<T> for T {
  /// No-op.
  #[inline(always)]
  fn from_pixmap(pixmap: T) -> Self {
    pixmap
  }
}

impl<T: Pixmap, U: Pixmap> IntoPixmap<U> for T
where
  U: FromPixmap<T>,
{
  /// Converts `T` into `U`.
  fn into_pixmap(self) -> U {
    U::from_pixmap(self)
  }
}

// -------------------------------------------------------------------------- //

macro_rules! impl_from_basic_for_basic {
  ($pixmap1:ident, $pixmap2:ident, $doc:expr) => {
    impl FromPixmap<$pixmap1> for $pixmap2 {
      #[doc = $doc]
      fn from_pixmap(pixmap: $pixmap1) -> Self {
        Self {
          width: pixmap.width,
          height: pixmap.height,
          pixels: pixmap
            .pixels
            .into_iter()
            .map(|px| px.into_pixel())
            .collect(),
        }
      }
    }
  };
}

macro_rules! impl_from_basic_for_indexed_opaq {
  ($pixmap:ident, $doc:expr) => {
    impl FromPixmap<$pixmap> for IndexedPixmap {
      #[doc = $doc]
      fn from_pixmap(pixmap: $pixmap) -> Self {
        let mut palette = IndexSet::with_capacity(u8::MAX as usize);

        let pixels = pixmap.pixels.into_iter().map(|px| {
          let (index, _) = palette.insert_full(px.into_pixel());
          IndexedPixel { i: index as u8 }
        });

        Self {
          width: pixmap.width,
          height: pixmap.height,
          pixels: pixels.collect(),
          palette: palette.into_iter().collect(),
          trns: None,
        }
      }
    }
  };
}

macro_rules! impl_from_basic_for_indexed_alpha {
  ($pixmap:ident, $doc:expr) => {
    impl FromPixmap<$pixmap> for IndexedPixmap {
      #[doc = $doc]
      fn from_pixmap(pixmap: $pixmap) -> Self {
        let mut palette = IndexSet::with_capacity(u8::MAX as usize);
        let mut trns = Vec::with_capacity(u8::MAX as usize);

        let mut sorted = pixmap.pixels.to_vec();
        sorted.sort_by(|a, b| a.a.cmp(&b.a));

        for pixel in sorted {
          let alpha = pixel.a;
          let (_, exists) = palette.insert_full(pixel.into_pixel());

          if alpha != u8::MAX && !exists {
            trns.push(alpha);
          }
        }

        let pixels = pixmap.pixels.into_iter().map(|px| {
          let index = palette.get_index_of(&RgbPixel::from_pixel(px)).unwrap();
          IndexedPixel { i: index as u8 }
        });

        Self {
          width: pixmap.width,
          height: pixmap.height,
          pixels: pixels.collect(),
          palette: palette.into_iter().collect(),
          trns: Some(trns),
        }
      }
    }
  };
}

macro_rules! impl_from_indexed_for_basic_opaq {
  ($pixmap:ident, $doc:expr) => {
    impl FromPixmap<IndexedPixmap> for $pixmap {
      #[doc = $doc]
      fn from_pixmap(pixmap: IndexedPixmap) -> Self {
        Self {
          width: pixmap.width,
          height: pixmap.height,
          pixels: pixmap
            .pixels
            .into_iter()
            .map(|px| pixmap.palette[px.i as usize].into_pixel())
            .collect(),
        }
      }
    }
  };
}

macro_rules! impl_from_indexed_for_basic_alpha {
  ($pixmap:ident, $doc:expr) => {
    impl FromPixmap<IndexedPixmap> for $pixmap {
      #[doc = $doc]
      fn from_pixmap(pixmap: IndexedPixmap) -> Self {
        let pixels_iter = pixmap.pixels.into_iter().map(|px| {
          let index = px.i as usize;

          let mut pixel: <Self as Pixmap>::Pixel =
            pixmap.palette[index].into_pixel();

          if let Some(trns) = &pixmap.trns
            && let Some(alpha) = trns.get(index)
          {
            pixel.a = *alpha;
          };

          pixel
        });

        Self {
          width: pixmap.width,
          height: pixmap.height,
          pixels: pixels_iter.collect(),
        }
      }
    }
  };
}

// -------------------------------------------------------------------------- //

impl_from_basic_for_basic!(
  LumaAlphaPixmap,
  LumaPixmap,
  "Converts a [`LumaAlphaPixmap`] into a [`LumaPixmap`]."
);

impl_from_basic_for_basic!(
  RgbPixmap,
  LumaPixmap,
  "Converts an [`RgbPixmap`] into a [`LumaPixmap`]."
);

impl_from_basic_for_basic!(
  RgbAlphaPixmap,
  LumaPixmap,
  "Converts an [`RgbAlphaPixmap`] into a [`LumaPixmap`]."
);

impl_from_indexed_for_basic_opaq!(
  LumaPixmap,
  "Converts an [`IndexedPixmap`] into a [`LumaPixmap`]."
);

// -------------------------------------------------------------------------- //

impl_from_basic_for_basic!(
  LumaPixmap,
  LumaAlphaPixmap,
  "Converts a [`LumaPixmap`] into a [`LumaAlphaPixmap`]."
);

impl_from_basic_for_basic!(
  RgbPixmap,
  LumaAlphaPixmap,
  "Converts an [`RgbPixmap`] into a [`LumaAlphaPixmap`]."
);

impl_from_basic_for_basic!(
  RgbAlphaPixmap,
  LumaAlphaPixmap,
  "Converts an [`RgbAlphaPixmap`] into a [`LumaAlphaPixmap`]."
);

impl_from_indexed_for_basic_alpha!(
  LumaAlphaPixmap,
  "Converts an [`IndexedPixmap`] into a [`LumaAlphaPixmap`]."
);

// -------------------------------------------------------------------------- //

impl_from_basic_for_basic!(
  LumaPixmap,
  RgbPixmap,
  "Converts a [`LumaPixmap`] into an [`RgbPixmap`]."
);

impl_from_basic_for_basic!(
  LumaAlphaPixmap,
  RgbPixmap,
  "Converts a [`LumaAlphaPixmap`] into an [`RgbPixmap`]."
);

impl_from_basic_for_basic!(
  RgbAlphaPixmap,
  RgbPixmap,
  "Converts an [`RgbAlphaPixmap`] into an [`RgbPixmap`]."
);

impl_from_indexed_for_basic_opaq!(
  RgbPixmap,
  "Converts an [`IndexedPixmap`] into an [`RgbAlphaPixmap`]."
);

// -------------------------------------------------------------------------- //

impl_from_basic_for_basic!(
  LumaPixmap,
  RgbAlphaPixmap,
  "Converts a [`LumaPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_basic_for_basic!(
  LumaAlphaPixmap,
  RgbAlphaPixmap,
  "Converts a [`LumaAlphaPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_basic_for_basic!(
  RgbPixmap,
  RgbAlphaPixmap,
  "Converts an [`RgbPixmap`] into an [`RgbAlphaPixmap`]."
);

impl_from_indexed_for_basic_alpha!(
  RgbAlphaPixmap,
  "Converts an [`IndexedPixmap`] into an [`RgbAlphaPixmap`]."
);

// -------------------------------------------------------------------------- //

impl_from_basic_for_indexed_opaq!(
  LumaPixmap,
  "Converts a [`LumaPixmap`] into an [`IndexedPixmap`]."
);

impl_from_basic_for_indexed_alpha!(
  LumaAlphaPixmap,
  "Converts a [`LumaAlphaPixmap`] into an [`IndexedPixmap`]."
);

impl_from_basic_for_indexed_opaq!(
  RgbPixmap,
  "Converts an [`RgbPixmap`] into an [`IndexedPixmap`]."
);

impl_from_basic_for_indexed_alpha!(
  RgbAlphaPixmap,
  "Converts an [`RgbAlphaPixmap`] into an [`IndexedPixmap`]."
);
