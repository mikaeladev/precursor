use std::fs::File;
use std::io::{BufRead, BufReader};

use precursor_config::{AssetValue, RotateValue};
use precursor_formats::png::PngFile;
use precursor_pixmap::DynamicPixmap;
use precursor_point::Point;

use crate::error::{PrecursorError, PrecursorResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorIcon {
  pub pixmap: DynamicPixmap,
  pub nominal: u32,
  pub hotspot: Point<u32>,
}

impl CursorIcon {
  /// Scales the icon up by `factor`.
  ///
  /// # Panics
  ///
  /// Panics if the new length of the pixmap exceeds `isize::MAX`.
  pub fn scale_up(&mut self, factor: usize) {
    self.nominal *= factor as u32;
    self.hotspot *= factor as u32;
    self.pixmap.scale_up(factor);
  }

  /// Constructs a new `CursorIcon` from a [`CursorIconConfig`].
  ///
  /// # Errors
  ///
  /// Fails with a [`PrecursorError`] if any of the following are true:
  ///
  /// * The hotspot is out of bounds (i.e. > `nominal`).
  /// * The asset is not a supported file format.
  /// * The image data is malformed.
  ///
  /// # Panics
  ///
  /// Panics if the image buffer exceeds `isize::MAX`.
  pub fn from_config(
    precursor_config::CursorIcon {
      asset,
      hotspot,
      nominal,
    }: precursor_config::CursorIcon,
  ) -> PrecursorResult<Self> {
    let mut reader = BufReader::new(File::open(asset.path())?);
    let mut pixmap;

    let buffer = reader.fill_buf()?;

    if buffer.starts_with(PngFile::MAGIC) {
      pixmap = PngFile::read(&mut reader)?.into_pixmap();
      drop(reader);
    } else {
      return Err(PrecursorError::InvalidAssetType);
    }

    if let AssetValue::Verbose {
      flip, flop, rotate, ..
    } = asset
    {
      if flip.unwrap_or_default() {
        pixmap.flip_horizontal();
      }

      if flop.unwrap_or_default() {
        pixmap.flip_vertical();
      }

      if let Some(value) = rotate {
        match value {
          RotateValue::Ninety => pixmap.rotate_90(),
          RotateValue::OneEighty => pixmap.rotate_180(),
          RotateValue::TwoSeventy => pixmap.rotate_270(),
        }
      }
    }

    let hotspot = Point::from(hotspot);

    Ok(CursorIcon {
      nominal,
      hotspot,
      pixmap,
    })
  }
}
