macro_rules! impl_pixmap_new {
  // pixmap
  ($pixmap:ident) => {
    crate::macros::impl_pixmap_new!(@gen $pixmap, { }, impl_pixmap_new!(@doc $pixmap));
  };

  // pixmap with extra fields
  ($pixmap:ident, $fields:tt) => {
    crate::macros::impl_pixmap_new!(@gen $pixmap, $fields, crate::macros::impl_pixmap_new!(@doc $pixmap));
  };

  (@doc $pixmap:ident) => {
    concat!("Constructs a new [`", stringify!($pixmap), "`].")
  };

  (@gen $pixmap:ident, { $( $ident:ident; $type:ty ),* }, $doc:expr) => {
    impl $pixmap {
      #[doc = $doc]
      ///
      /// # Errors
      ///
      /// Fails with a [`PixmapError`] if the number of pixels
      /// `!= width * height`.
      ///
      /// [`PixmapError`]: crate::PixmapError
      pub fn new(
        width: u32,
        height: u32,
        pixels: Vec<<Self as crate::Pixmap>::Pixel>,
        $( $ident: $type, )*
      ) -> Self {
        assert_eq!(width as usize * height as usize, pixels.len(), "");

        Self {
          width,
          height,
          pixels,
          $( $ident, )*
        }
      }
    }
  };
}

pub(crate) use impl_pixmap_new;

macro_rules! impl_from_paletted_for_paletted {
  ($from:ident, $for:ident, $doc:expr) => {
    impl precursor_pixmap_core::FromPixmap<$from> for $for {
      #[doc = $doc]
      fn from_pixmap(pixmap: $from) -> Self {
        use precursor_pixmap_core::IntoPixel;

        Self {
          width: pixmap.width,
          height: pixmap.height,
          pixels: pixmap
            .pixels
            .into_iter()
            .map(|px| IntoPixel::into_pixel(px))
            .collect(),
        }
      }
    }
  };
}

pub(crate) use impl_from_paletted_for_paletted;

macro_rules! impl_from_paletted_for_indexed_opaque {
  ($from:ident, $for:ident<$for_pixel:ident>, $doc:expr) => {
    impl precursor_pixmap_core::FromPixmap<$from> for $for {
      #[doc = $doc]
      fn from_pixmap(pixmap: $from) -> Self {
        use precursor_pixmap_core::IntoPixel;

        let mut palette = indexmap::IndexSet::with_capacity(u8::MAX as usize);

        let pixels = pixmap.pixels.into_iter().map(|px| {
          let (index, _) = palette.insert_full(IntoPixel::into_pixel(px));
          $for_pixel { i: index as u8 }
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

pub(crate) use impl_from_paletted_for_indexed_opaque;

macro_rules! impl_from_paletted_for_indexed_alpha {
  ($from:ident, $for:ident<$for_pixel:ident>, $doc:expr) => {
    impl precursor_pixmap_core::FromPixmap<$from> for $for {
      #[doc = $doc]
      fn from_pixmap(pixmap: $from) -> Self {
        use precursor_pixmap_core::{FromPixel, IntoPixel};

        use crate::RgbPixel;

        let mut palette = indexmap::IndexSet::with_capacity(u8::MAX as usize);
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
          $for_pixel { i: index as u8 }
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

pub(crate) use impl_from_paletted_for_indexed_alpha;

macro_rules! impl_from_indexed_for_paletted_opaque {
  ($from:ident, $for:ident, $doc:expr) => {
    impl precursor_pixmap_core::FromPixmap<$from> for $for {
      #[doc = $doc]
      fn from_pixmap(pixmap: $from) -> Self {
        use precursor_pixmap_core::IntoPixel;

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

pub(crate) use impl_from_indexed_for_paletted_opaque;

macro_rules! impl_from_indexed_for_paletted_alpha {
  ($from:ident, $for:ident, $doc:expr) => {
    impl precursor_pixmap_core::FromPixmap<$from> for $for {
      #[doc = $doc]
      fn from_pixmap(pixmap: $from) -> Self {
        use precursor_pixmap_core::{IntoPixel, Pixmap};

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

pub(crate) use impl_from_indexed_for_paletted_alpha;
