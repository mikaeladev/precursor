mod error;
mod pixmap;
mod png;

pub use error::*;
pub use pixmap::*;
pub use png::*;

#[derive(Debug, Clone)]
pub struct RasterImage {
  width: u32,
  height: u32,
  pixmap: DynamicPixmap,
}

impl RasterImage {
  /// Creates a new `RasterImage`.
  pub fn new(
    width: u32,
    height: u32,
    pixmap: DynamicPixmap,
  ) -> Result<Self, RasterError> {
    let expected_pixels = width as usize * height as usize;
    let actual_pixels = pixmap.pixels_len();

    if expected_pixels != actual_pixels {
      return Err(RasterError::WrongDimensions(expected_pixels, actual_pixels));
    }

    Ok(Self {
      width,
      height,
      pixmap,
    })
  }

  /// Returns the image width.
  pub const fn width(&self) -> u32 {
    self.width
  }

  /// Returns the image height.
  pub const fn height(&self) -> u32 {
    self.height
  }

  /// Returns a reference to the pixmap.
  pub const fn pixmap(&self) -> &DynamicPixmap {
    &self.pixmap
  }

  /// Consumes the struct and returns the pixmap.
  pub fn into_pixmap(self) -> DynamicPixmap {
    self.pixmap
  }
}
