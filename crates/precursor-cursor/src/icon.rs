use std::fs::File;
use std::io::{BufRead, BufReader};

use precursor_config::{AssetValue, RotateValue};
use precursor_error::{Error, ErrorMessageWithAttrs, Result};
use precursor_filesys::ErrorMessageWithPath;
use precursor_formats::png::PngFile;

use pixmap::{DynamicPixmap, Pixmap};
use point::Point;

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

  /// Constructs a new `CursorIcon` from a [`precursor_config::CursorIcon`].
  ///
  /// # Errors
  ///
  /// Fails if any of the following are true:
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
  ) -> Result<Self> {
    let asset_path = asset.path();

    let asset_file = File::open(asset_path).map_err(|src| {
      Error::new(
        ErrorMessageWithPath::new(
          format!("failed to open asset; {}", src.kind()),
          asset_path,
        ),
        src,
      )
    })?;

    let mut reader = BufReader::new(asset_file);

    let buffer = reader.fill_buf().map_err(|src| {
      Error::new(
        ErrorMessageWithPath::new(
          format!("failed to fill icon buffer; {}", src.kind()),
          asset_path,
        ),
        src,
      )
    })?;

    let mut pixmap = if buffer.starts_with(PngFile::MAGIC) {
      let png = PngFile::read(&mut reader).map_err(|src| {
        // TODO: make this use some kind of builder
        Error::new(
          ErrorMessageWithAttrs {
            message: Box::new("failed to read asset as PNG"),
            attrs: vec![
              ("reason".to_string(), Box::new(src.to_string())),
              (
                "asset".to_string(),
                Box::new(asset_path.display().to_string()),
              ),
            ],
          },
          src,
        )
      })?;

      png.into_pixmap()
    } else {
      todo!()
    };

    drop(reader);

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
