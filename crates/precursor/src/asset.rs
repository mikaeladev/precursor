use std::fs::File;
use std::io::{BufRead, BufReader};

use crate_config::{AssetValue, RotateValue};
use crate_formats::{DynamicPixmap, PixmapTransform, PngImage};

use crate::cursor::{CursorHotspot, CursorIcon};
use crate::error::{PrecursorError::InvalidAssetType, PrecursorResult};

pub fn icon(
  nominal: u32,
  hotspot: (u32, u32),
  asset_config: AssetValue,
) -> PrecursorResult<CursorIcon> {
  let mut reader = BufReader::new(File::open(asset_config.path())?);
  let mut pixmap;

  let buffer = reader.fill_buf()?;

  if buffer.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
    pixmap = DynamicPixmap::decode_png(reader)?;
  } else {
    return Err(InvalidAssetType);
  }

  if let AssetValue::Verbose {
    flip, flop, rotate, ..
  } = asset_config
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
