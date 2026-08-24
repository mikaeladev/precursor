mod convert;
mod dynamic;
mod pixel;

pub use convert::*;
pub use dynamic::*;
pub use pixel::*;

use crate::raster::RasterError;

pub trait Pixmap<P: Pixel> {
  /// Returns the width of the Pixmap.
  fn width(&self) -> u32;

  /// Returns the height of the Pixmap.
  fn height(&self) -> u32;

  /// Returns the width and height of the Pixmap as a tuple.
  fn dimensions(&self) -> (u32, u32) {
    (self.width(), self.height())
  }

  /// Returns a reference to the underlying `Vec<P>`.
  fn pixels(&self) -> &Vec<P>;

  /// Returns a mutable reference to the underlying `Vec<P>`.
  fn pixels_mut(&mut self) -> &mut Vec<P>;

  /// Copies and concatenates the pixels into a new `Vec<u8>`.
  fn pixels_concat(&self) -> Vec<u8>;

  /// Returns a reference to the pixel at `(x,y)`.
  fn get_pixel(&self, x: u32, y: u32) -> Option<&P> {
    self.pixels().get(self.get_pixel_index(x, y)?)
  }

  /// Returns a mutable reference to the pixel at `(x,y)`.
  fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut P> {
    let i = self.get_pixel_index(x, y)?;
    self.pixels_mut().get_mut(i)
  }

  /// Returns the index of the pixel at `(x,y)`.
  fn get_pixel_index(&self, x: u32, y: u32) -> Option<usize> {
    let (width, height) = self.dimensions();
    if x >= width || y >= height {
      return None;
    }
    Some((y as usize * width as usize + x as usize) * P::NUM_CHANNELS)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LumaPixmap {
  /// Width of the pixmap.
  width: u32,
  /// Height of the pixmap.
  height: u32,
  /// Packed array of greyscale pixels.
  pixels: Vec<LumaPixel>,
}

impl LumaPixmap {
  /// Creates a new `LumaPixmap`.
  ///
  /// Returns an `Err` if the number of pixels `!=` the width `*` the height.
  pub fn new(
    width: u32,
    height: u32,
    pixels: Vec<LumaPixel>,
  ) -> Result<Self, RasterError> {
    let expected_pixels = width as usize * height as usize;
    let actual_pixels = pixels.len();

    if expected_pixels != actual_pixels {
      return Err(RasterError::WrongDimensions(expected_pixels, actual_pixels));
    }

    Ok(Self {
      width,
      height,
      pixels,
    })
  }
}

impl Pixmap<LumaPixel> for LumaPixmap {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn pixels(&self) -> &Vec<LumaPixel> {
    &self.pixels
  }

  fn pixels_mut(&mut self) -> &mut Vec<LumaPixel> {
    &mut self.pixels
  }

  fn pixels_concat(&self) -> Vec<u8> {
    self.pixels.iter().map(|p| p.y).collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LumaAlphaPixmap {
  /// Width of the pixmap.
  width: u32,
  /// Height of the pixmap.
  height: u32,
  /// Packed array of greyscale alpha pixels.
  pixels: Vec<LumaAlphaPixel>,
}

impl LumaAlphaPixmap {
  /// Creates a new `LumaAlphaPixmap`.
  ///
  /// Returns an `Err` if the number of pixels `!=` the width `*` the height.
  pub fn new(
    width: u32,
    height: u32,
    pixels: Vec<LumaAlphaPixel>,
  ) -> Result<Self, RasterError> {
    let expected_pixels = width as usize * height as usize;
    let actual_pixels = pixels.len();

    if expected_pixels != actual_pixels {
      return Err(RasterError::WrongDimensions(expected_pixels, actual_pixels));
    }

    Ok(Self {
      width,
      height,
      pixels,
    })
  }
}

impl Pixmap<LumaAlphaPixel> for LumaAlphaPixmap {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn pixels(&self) -> &Vec<LumaAlphaPixel> {
    &self.pixels
  }

  fn pixels_mut(&mut self) -> &mut Vec<LumaAlphaPixel> {
    &mut self.pixels
  }

  fn pixels_concat(&self) -> Vec<u8> {
    self.pixels.iter().flat_map(|p| p.into_iter()).collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbPixmap {
  /// Width of the pixmap.
  width: u32,
  /// Height of the pixmap.
  height: u32,
  /// Packed array of RGB pixels.
  pixels: Vec<RgbPixel>,
}

impl RgbPixmap {
  /// Creates a new `RgbPixmap`.
  ///
  /// Returns an `Err` if the number of pixels `!=` the width `*` the height.
  pub fn new(
    width: u32,
    height: u32,
    pixels: Vec<RgbPixel>,
  ) -> Result<Self, RasterError> {
    let expected_pixels = width as usize * height as usize;
    let actual_pixels = pixels.len();

    if expected_pixels != actual_pixels {
      return Err(RasterError::WrongDimensions(expected_pixels, actual_pixels));
    }

    Ok(Self {
      width,
      height,
      pixels,
    })
  }
}

impl Pixmap<RgbPixel> for RgbPixmap {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn pixels(&self) -> &Vec<RgbPixel> {
    &self.pixels
  }

  fn pixels_mut(&mut self) -> &mut Vec<RgbPixel> {
    &mut self.pixels
  }

  fn pixels_concat(&self) -> Vec<u8> {
    self.pixels.iter().flat_map(|p| p.into_iter()).collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbAlphaPixmap {
  /// Width of the pixmap.
  width: u32,
  /// Height of the pixmap.
  height: u32,
  /// Packed array of RGB alpha pixels.
  pixels: Vec<RgbAlphaPixel>,
}

impl RgbAlphaPixmap {
  /// Creates a new `RgbAlphaPixmap`.
  ///
  /// Returns an `Err` if the number of pixels `!=` the width `*` the height.
  pub fn new(
    width: u32,
    height: u32,
    pixels: Vec<RgbAlphaPixel>,
  ) -> Result<Self, RasterError> {
    let expected_pixels = width as usize * height as usize;
    let actual_pixels = pixels.len();

    if expected_pixels != actual_pixels {
      return Err(RasterError::WrongDimensions(expected_pixels, actual_pixels));
    }

    Ok(Self {
      width,
      height,
      pixels,
    })
  }
}

impl Pixmap<RgbAlphaPixel> for RgbAlphaPixmap {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn pixels(&self) -> &Vec<RgbAlphaPixel> {
    &self.pixels
  }

  fn pixels_mut(&mut self) -> &mut Vec<RgbAlphaPixel> {
    &mut self.pixels
  }

  fn pixels_concat(&self) -> Vec<u8> {
    self.pixels.iter().flat_map(|p| p.into_iter()).collect()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedPixmap {
  /// Width of the pixmap.
  width: u32,
  /// Height of the pixmap.
  height: u32,
  /// Packed array of palette indexes.
  pixels: Vec<PaletteIndex>,
  /// Array of `RgbPixel`s.
  palette: Vec<RgbPixel>,
  /// Optional array of alpha values corresponding to pixels in `palette`.
  ///
  /// Missing values are assumed to be opaque (`255`).
  trns: Option<Vec<u8>>,
}

impl IndexedPixmap {
  /// Creates a new `IndexedPixmap`.
  ///
  /// Returns an `Err` if the number of pixels `!=` the width `*` the height.
  pub fn new(
    width: u32,
    height: u32,
    pixels: Vec<PaletteIndex>,
    palette: Vec<RgbPixel>,
    trns: Option<Vec<u8>>,
  ) -> Result<Self, RasterError> {
    let expected_pixels = width as usize * height as usize;
    let actual_pixels = pixels.len();

    if expected_pixels != actual_pixels {
      return Err(RasterError::WrongDimensions(expected_pixels, actual_pixels));
    }

    Ok(Self {
      width,
      height,
      pixels,
      palette,
      trns,
    })
  }

  /// Returns a reference to the palette.
  pub const fn palette(&self) -> &Vec<RgbPixel> {
    &self.palette
  }

  /// Returns a reference to the trns.
  pub const fn trns(&self) -> &Option<Vec<u8>> {
    &self.trns
  }
}

impl Pixmap<PaletteIndex> for IndexedPixmap {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn pixels(&self) -> &Vec<PaletteIndex> {
    &self.pixels
  }

  fn pixels_mut(&mut self) -> &mut Vec<PaletteIndex> {
    &mut self.pixels
  }

  fn pixels_concat(&self) -> Vec<u8> {
    self.pixels.iter().map(|p| p.i).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rustfmt::skip]
  pub const LUMA_PIXELS: [LumaPixel; 9] = [
    LumaPixel { y: 255 }, LumaPixel { y: 200 }, LumaPixel { y: 145 },
    LumaPixel { y: 200 }, LumaPixel { y: 145 }, LumaPixel { y: 095 },
    LumaPixel { y: 145 }, LumaPixel { y: 095 }, LumaPixel { y: 040 },
  ];

  #[test]
  fn width() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    assert_eq!(pixmap.width(), pixmap.width);
  }

  #[test]
  fn height() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    assert_eq!(pixmap.height(), pixmap.height);
  }

  #[test]
  fn dimensions() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    assert_eq!(pixmap.dimensions(), (pixmap.width, pixmap.height));
  }

  #[test]
  fn pixels() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    assert_eq!(pixmap.pixels(), &pixmap.pixels);
  }

  #[test]
  fn pixels_concat() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    #[rustfmt::skip]
    let expected_pixels = vec![
      255, 200, 145,
      200, 145, 095,
      145, 095, 040,
    ];

    assert_eq!(pixmap.pixels_concat(), expected_pixels);
  }

  #[test]
  fn get_pixel() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    assert_eq!(pixmap.get_pixel(0, 0), Some(&LUMA_PIXELS[0]));
    assert_eq!(pixmap.get_pixel(1, 1), Some(&LUMA_PIXELS[4]));
    assert_eq!(pixmap.get_pixel(2, 2), Some(&LUMA_PIXELS[8]));
    assert_eq!(pixmap.get_pixel(3, 3), None);
  }

  #[test]
  fn get_pixel_index() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    assert_eq!(pixmap.get_pixel_index(0, 0), Some(0));
    assert_eq!(pixmap.get_pixel_index(1, 1), Some(4));
    assert_eq!(pixmap.get_pixel_index(2, 2), Some(8));
    assert_eq!(pixmap.get_pixel_index(3, 3), None);
  }
}
