pub type RgbaPixel = [u8; 4];

#[derive(Debug, Clone)]
pub struct RasterImage {
  width: u32,
  height: u32,
  pixels: Vec<RgbaPixel>,
}

impl RasterImage {
  /// Creates a new RGBA image.
  pub fn new(width: u32, height: u32, pixels: Vec<RgbaPixel>) -> Self {
    let expected_len = width as usize * height as usize;
    let actual_len = pixels.len();

    if expected_len != actual_len {
      panic!(
        "wrong data size, expected {} pixels but got {}",
        expected_len, actual_len,
      )
    }

    Self {
      width,
      height,
      pixels,
    }
  }

  /// Returns the image width.
  pub const fn width(&self) -> u32 {
    self.width
  }

  /// Returns the image height.
  pub const fn height(&self) -> u32 {
    self.height
  }

  /// Extracts a slice containing the entire RGBA buffer.
  pub const fn pixels(&self) -> &[RgbaPixel] {
    self.pixels.as_slice()
  }

  /// Flattens a slice of the RGBA buffer into a new `Vec`.
  pub fn to_rgba(&self) -> Vec<u8> {
    self.pixels.concat()
  }

  /// Consumes the RGBA buffer and returns a flattened `Vec`.
  pub fn into_rgba(self) -> Vec<u8> {
    self.pixels.into_iter().flatten().collect()
  }

  /// Flattens a slice of the RGBA buffer into a new `Vec` formatted as BGRA.
  pub fn to_bgra(&self) -> Vec<u8> {
    self
      .pixels
      .iter()
      .flat_map(|c| [c[2], c[1], c[0], c[3]])
      .collect()
  }

  /// Consumes the RGBA buffer and returns a flattened `Vec` formatted as BGRA.
  pub fn into_bgra(self) -> Vec<u8> {
    self
      .pixels
      .into_iter()
      .flat_map(|c| [c[2], c[1], c[0], c[3]])
      .collect()
  }
}
