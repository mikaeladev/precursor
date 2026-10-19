use pixmap_core::{IntoIter, Pixel, Pixmap};
use pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct IndexedPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<IndexedPixel>,
  pub(crate) palette: Vec<RgbPixel>,
  pub(crate) alpha: Option<Vec<u8>>,
}

impl IndexedPixmap {
  /// Returns a slice of the underlying palette vector.
  pub const fn palette(&self) -> &[RgbPixel] {
    self.palette.as_slice()
  }

  /// Returns `Some` slice of the underlying alpha vector`.
  pub const fn alpha(&self) -> Option<&[u8]> {
    if let Some(alpha) = &self.alpha {
      Some(alpha.as_slice())
    } else {
      None
    }
  }
}

impl_pixmap_new!(IndexedPixmap, {
  palette; Vec::<RgbPixel>,
  alpha; Option::<Vec<u8>>
});

impl_from_paletted_for_indexed_opaque!(
  BgrPixmap,
  IndexedPixmap<IndexedPixel>,
  "Converts a [`BgrPixmap`] into an [`IndexedPixmap`]."
);

impl_from_paletted_for_indexed_opaque!(
  BgrAlphaPixmap,
  IndexedPixmap<IndexedPixel>,
  "Converts a [`BgrAlphaPixmap`] into an [`IndexedPixmap`]."
);

impl_from_paletted_for_indexed_opaque!(
  LumaPixmap,
  IndexedPixmap<IndexedPixel>,
  "Converts a [`LumaPixmap`] into an [`IndexedPixmap`]."
);

impl_from_paletted_for_indexed_alpha!(
  LumaAlphaPixmap,
  IndexedPixmap<IndexedPixel>,
  "Converts a [`LumaAlphaPixmap`] into an [`IndexedPixmap`]."
);

impl_from_paletted_for_indexed_opaque!(
  RgbPixmap,
  IndexedPixmap<IndexedPixel>,
  "Converts an [`RgbPixmap`] into an [`IndexedPixmap`]."
);

impl_from_paletted_for_indexed_alpha!(
  RgbAlphaPixmap,
  IndexedPixmap<IndexedPixel>,
  "Converts an [`RgbAlphaPixmap`] into an [`IndexedPixmap`]."
);

#[derive(Debug, Pixel, Clone, Copy, PartialEq, Eq)]
pub struct IndexedPixel {
  pub i: u8,
}
