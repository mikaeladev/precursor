#[derive(Clone)]
pub struct RgbaImage {
  width: u32,
  height: u32,
  rgba: Vec<u8>,
}

impl RgbaImage {
  /// Creates a new RGBA image.
  pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Self {
    let expected_len = width as usize * height as usize * 4;
    let actual_len = rgba.len();

    if expected_len != actual_len {
      panic!("wrong data size, expected {expected_len} and got {actual_len}")
    }

    Self {
      width,
      height,
      rgba,
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

  /// Returns the length of the RGBA buffer.
  pub const fn len(&self) -> usize {
    self.rgba.len()
  }

  /// Consumes the image and returns an RGBA buffer.
  pub fn into_rgba(self) -> Vec<u8> {
    self.rgba
  }

  /// Consumes the image and returns a BGRA buffer.
  pub fn into_bgra(self) -> Vec<u8> {
    let mut bgra = self.into_rgba();

    for chunk in bgra.as_chunks_mut::<4>().0 {
      chunk.swap(0, 2);
    }

    bgra
  }
}
