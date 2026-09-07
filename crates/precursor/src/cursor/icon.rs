use std::fs::File;
use std::io::{BufRead, BufReader};

use crate_config::{AssetValue, CursorIconConfig, RotateValue};

use crate_formats::cursors::Hotspot;
use crate_formats::png::PngImage;

use crate_pixmap::DynamicPixmap;

use crate::error::{PrecursorError, PrecursorResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorIcon {
  pub nominal: u32,
  pub hotspot: Hotspot,
  pub pixmap: DynamicPixmap,
}

impl CursorIcon {
  /// Scales the icon up by `factor`.
  ///
  /// # Panics
  ///
  /// Panics if the new length exceeds `isize::MAX`.
  pub fn scale_up(&mut self, factor: usize) {
    self.nominal *= factor as u32;
    self.hotspot *= factor as u32;
    self.pixmap.scale_up(factor);
  }

  /// Attemts to construct a new `CursorIcon` from a [`CursorIconConfig`].
  ///
  /// # Errors
  ///
  /// Fails with a [`PrecursorResult`] if any of the following are true:
  ///
  /// * The hotspot is out of bounds (i.e. > `nominal`).
  /// * The asset is not a `PNG` file.
  /// * The `PNG` data is malformed.
  ///
  /// # Panics
  ///
  /// Panics if the image buffer exceeds `isize::MAX`.
  pub fn from_config(
    CursorIconConfig {
      asset,
      hotspot,
      nominal,
    }: CursorIconConfig,
  ) -> PrecursorResult<Self> {
    let mut reader = BufReader::new(File::open(asset.path())?);
    let mut pixmap;

    let buffer = reader.fill_buf()?;

    if buffer.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
      pixmap = DynamicPixmap::decode_png(reader)?;
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

    let hotspot = Hotspot {
      x: hotspot.0,
      y: hotspot.1,
    };

    Ok(CursorIcon {
      nominal,
      hotspot,
      pixmap,
    })
  }
}
