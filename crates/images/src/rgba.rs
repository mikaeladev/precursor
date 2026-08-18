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

  /// Extracts a slice containing the entire RGBA buffer.
  pub const fn as_rgba(&self) -> &[u8] {
    self.rgba.as_slice()
  }

  /// Copies the contents of the RGBA buffer into a new buffer.
  #[allow(dead_code)]
  pub fn to_rgba(&self) -> Vec<u8> {
    self.rgba.to_vec()
  }

  /// Consumes the struct and returns the RGBA buffer.
  #[allow(dead_code)]
  pub fn into_rgba(self) -> Vec<u8> {
    self.rgba
  }

  /// Creates a new BGRA buffer from the RGBA buffer.
  pub fn to_bgra(&self) -> Vec<u8> {
    let chunks = self.rgba.as_chunks::<4>().0;
    let map = chunks.iter().flat_map(|c| [c[2], c[1], c[0], c[3]]);
    map.collect()
  }

  /// Consumes the struct and returns a BGRA buffer.
  #[allow(dead_code)]
  pub fn into_bgra(self) -> Vec<u8> {
    let mut bgra = self.into_rgba();
    for chunk in bgra.as_chunks_mut::<4>().0 {
      chunk.swap(0, 2);
    }
    bgra
  }
}
