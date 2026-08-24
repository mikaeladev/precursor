use std::fs::File;
use std::io::{BufRead, BufReader};

use crate_config::{AssetConfig, RotateValue};
use crate_cursor::{CursorHotspot, CursorImage};
use crate_formats::raster::{DynamicPixmap, PixmapTransform, PngImage};

use crate::error::{PrecursorError::InvalidAssetType, PrecursorResult};

pub struct Asset {
  nominal: u32,
  hotspot: CursorHotspot,
  pixmap: DynamicPixmap,
}

impl Asset {
  /// Creates a new `Asset`.
  pub fn from_config(
    nominal: u32,
    hotspot: CursorHotspot,
    asset_config: &AssetConfig,
  ) -> PrecursorResult<Asset> {
    let mut reader = BufReader::new(File::open(&asset_config.path)?);
    let mut pixmap;

    let buffer = reader.fill_buf()?;

    if buffer.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
      pixmap = DynamicPixmap::decode_png(reader)?;
    } else {
      return Err(InvalidAssetType);
    }

    if asset_config.flip.unwrap_or_default() {
      match &mut pixmap {
        DynamicPixmap::Luma(p) => p.flip_horizontal(),
        DynamicPixmap::LumaAlpha(p) => p.flip_horizontal(),
        DynamicPixmap::Rgb(p) => p.flip_horizontal(),
        DynamicPixmap::RgbAlpha(p) => p.flip_horizontal(),
        DynamicPixmap::Indexed(p) => p.flip_horizontal(),
      }
    }

    if asset_config.flop.unwrap_or_default() {
      match &mut pixmap {
        DynamicPixmap::Luma(p) => p.flip_vertical(),
        DynamicPixmap::LumaAlpha(p) => p.flip_vertical(),
        DynamicPixmap::Rgb(p) => p.flip_vertical(),
        DynamicPixmap::RgbAlpha(p) => p.flip_vertical(),
        DynamicPixmap::Indexed(p) => p.flip_vertical(),
      }
    }

    if let Some(rotate) = asset_config.rotate {
      match rotate {
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

    Ok(Self {
      nominal,
      hotspot,
      pixmap,
    })
  }

  /// Converts the asset into a `CursorImage`.
  pub fn into_image(self) -> CursorImage {
    CursorImage {
      nominal: self.nominal,
      hotspot: self.hotspot,
      pixmap: self.pixmap,
    }
  }
}
