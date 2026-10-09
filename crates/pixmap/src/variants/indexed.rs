use precursor_pixmap_core::{IntoIter, Pixel, Pixmap};
use precursor_pixmap_derive::{Pixel, Pixmap};

use crate::macros::*;
use crate::variants::*;

#[derive(Debug, Pixmap, Clone, PartialEq, Eq)]
pub struct IndexedPixmap {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) pixels: Vec<IndexedPixel>,
  pub(crate) palette: Vec<RgbPixel>,
  pub(crate) trns: Option<Vec<u8>>,
}

impl IndexedPixmap {
  /// Returns a slice of the underlying palette `Vec`.
  pub const fn palette(&self) -> &[RgbPixel] {
    self.palette.as_slice()
  }

  /// Returns `Some` slice of the underlying trns `Vec`.
  pub const fn trns(&self) -> Option<&[u8]> {
    if let Some(trns) = &self.trns {
      Some(trns.as_slice())
    } else {
      None
    }
  }
}

impl_pixmap_new!(IndexedPixmap, {
  palette; Vec::<RgbPixel>,
  trns; Option::<Vec<u8>>
});

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
