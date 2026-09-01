use std::fs::File;
use std::io::{BufRead, BufReader};

use crate_config::{AssetValue, CursorIconConfig, RotateValue};
use crate_formats::{DynamicPixmap, Pixmap, PngImage};

use crate::error::{PrecursorError, PrecursorResult};

#[derive(Debug, Clone)]
pub struct CursorIcon {
  pub nominal: u32,
  pub hotspot: CursorHotspot,
  pub pixmap: DynamicPixmap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorHotspot {
  pub x: u32,
  pub y: u32,
}

impl CursorIcon {
  // TODO: document
  pub fn to_scaled_vec(self) -> Vec<Self> {
    // TODO: scale
    vec![self]
  }

  // TODO: document
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
        match &mut pixmap {
          DynamicPixmap::Luma(p) => p.flip_horizontal(),
          DynamicPixmap::LumaAlpha(p) => p.flip_horizontal(),
          DynamicPixmap::Rgb(p) => p.flip_horizontal(),
          DynamicPixmap::RgbAlpha(p) => p.flip_horizontal(),
          DynamicPixmap::Indexed(p) => p.flip_horizontal(),
        }
      }

      if flop.unwrap_or_default() {
        match &mut pixmap {
          DynamicPixmap::Luma(p) => p.flip_vertical(),
          DynamicPixmap::LumaAlpha(p) => p.flip_vertical(),
          DynamicPixmap::Rgb(p) => p.flip_vertical(),
          DynamicPixmap::RgbAlpha(p) => p.flip_vertical(),
          DynamicPixmap::Indexed(p) => p.flip_vertical(),
        }
      }

      if let Some(value) = rotate {
        match value {
          RotateValue::Ninety => match &mut pixmap {
            DynamicPixmap::Luma(p) => p.rotate_90(),
            DynamicPixmap::LumaAlpha(p) => p.rotate_90(),
            DynamicPixmap::Rgb(p) => p.rotate_90(),
            DynamicPixmap::RgbAlpha(p) => p.rotate_90(),
            DynamicPixmap::Indexed(p) => p.rotate_90(),
          },
          RotateValue::OneEighty => match &mut pixmap {
            DynamicPixmap::Luma(p) => p.rotate_180(),
            DynamicPixmap::LumaAlpha(p) => p.rotate_180(),
            DynamicPixmap::Rgb(p) => p.rotate_180(),
            DynamicPixmap::RgbAlpha(p) => p.rotate_180(),
            DynamicPixmap::Indexed(p) => p.rotate_180(),
          },
          RotateValue::TwoSeventy => match &mut pixmap {
            DynamicPixmap::Luma(p) => p.rotate_270(),
            DynamicPixmap::LumaAlpha(p) => p.rotate_270(),
            DynamicPixmap::Rgb(p) => p.rotate_270(),
            DynamicPixmap::RgbAlpha(p) => p.rotate_270(),
            DynamicPixmap::Indexed(p) => p.rotate_270(),
          },
        }
      }
    }

    let hotspot = CursorHotspot {
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
