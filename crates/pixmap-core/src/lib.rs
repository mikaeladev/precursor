pub mod iter;
pub mod ops;
pub mod pixel;
pub mod pixmap;

pub use iter::*;
pub use pixel::*;
pub use pixmap::*;

#[cfg(test)]
mod tests {
  use std::sync::LazyLock;

  use super::*;

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  struct TestPixel {
    pub y: u8,
  }

  impl Pixel for TestPixel {
    type ByteArray = [u8; 1];

    fn from_bytes([y]: Self::ByteArray) -> Self {
      Self { y }
    }

    fn into_bytes(self) -> Self::ByteArray {
      [self.y]
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq)]
  struct TestPixmap {
    width: u32,
    height: u32,
    pixels: Vec<TestPixel>,
  }

  impl Pixmap for TestPixmap {
    type Pixel = TestPixel;

    fn width(&self) -> u32 {
      self.width
    }

    fn height(&self) -> u32 {
      self.height
    }

    fn pixels(&self) -> &[TestPixel] {
      &self.pixels
    }

    fn pixels_mut(&mut self) -> &mut [TestPixel] {
      &mut self.pixels
    }

    fn scale_up(&mut self, factor: usize) {
      ops::scale_up_in_place(
        &mut self.width,
        &mut self.height,
        &mut self.pixels,
        factor,
      )
    }
  }

  impl IntoIterator for TestPixmap {
    type Item = u8;
    type IntoIter =
      IntoIter<TestPixel, { size_of::<<TestPixel as Pixel>::ByteArray>() }>;

    fn into_iter(self) -> Self::IntoIter {
      IntoIter::new(self.pixels)
    }
  }

  const fn px(y: u8) -> TestPixel {
    TestPixel { y }
  }

  #[rustfmt::skip]
  const PIXELS: [TestPixel; 9] = [
    px(255), px(200), px(145),
    px(200), px(145), px(095),
    px(145), px(095), px(040),
  ];

  static PIXMAP: LazyLock<TestPixmap> = LazyLock::new(|| TestPixmap {
    width: 3,
    height: 3,
    pixels: Vec::from(PIXELS),
  });

  #[test]
  fn width() {
    assert_eq!(PIXMAP.width(), PIXMAP.width);
  }

  #[test]
  fn height() {
    assert_eq!(PIXMAP.height(), PIXMAP.height);
  }

  #[test]
  fn dimensions() {
    assert_eq!(PIXMAP.dimensions(), (PIXMAP.width, PIXMAP.height));
  }

  #[test]
  fn pixels() {
    assert_eq!(PIXMAP.pixels(), &PIXMAP.pixels);
  }

  #[test]
  fn get_pixel() {
    assert_eq!(PIXMAP.get_pixel(0, 0), Some(&PIXELS[0]));
    assert_eq!(PIXMAP.get_pixel(1, 1), Some(&PIXELS[4]));
    assert_eq!(PIXMAP.get_pixel(2, 2), Some(&PIXELS[8]));
    assert_eq!(PIXMAP.get_pixel(3, 3), None);
  }

  #[test]
  fn get_pixel_index() {
    assert_eq!(PIXMAP.get_pixel_index(0, 0), Some(0));
    assert_eq!(PIXMAP.get_pixel_index(1, 1), Some(4));
    assert_eq!(PIXMAP.get_pixel_index(2, 2), Some(8));
    assert_eq!(PIXMAP.get_pixel_index(3, 3), None);
  }

  #[test]
  fn set_pixel() {
    let mut pixmap = PIXMAP.clone();

    pixmap.set_pixel(0, 0, px(040));
    pixmap.set_pixel(2, 2, px(255));

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(040), px(200), px(145),
      px(200), px(145), px(095),
      px(145), px(095), px(255),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn flip_horizontal() {
    let mut pixmap = PIXMAP.clone();

    pixmap.flip_horizontal();

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(145), px(200), px(255),
      px(095), px(145), px(200),
      px(040), px(095), px(145),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn flip_horizontal_and_back() {
    let mut pixmap = PIXMAP.clone();

    pixmap.flip_horizontal();
    pixmap.flip_horizontal();

    assert_eq!(pixmap.pixels, PIXMAP.pixels);
  }

  #[test]
  fn flip_vertical() {
    let mut pixmap = PIXMAP.clone();

    pixmap.flip_vertical();

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(145), px(095), px(040),
      px(200), px(145), px(095),
      px(255), px(200), px(145),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn flip_vertical_and_back() {
    let mut pixmap = PIXMAP.clone();

    pixmap.flip_vertical();
    pixmap.flip_vertical();

    assert_eq!(pixmap.pixels, PIXMAP.pixels);
  }

  #[test]
  fn flip_horizontal_and_vertical() {
    let mut pixmap = PIXMAP.clone();

    pixmap.flip_horizontal();
    pixmap.flip_vertical();

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(040), px(095), px(145),
      px(095), px(145), px(200),
      px(145), px(200), px(255),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn rotate_90() {
    let mut pixmap = PIXMAP.clone();

    pixmap.rotate_90();

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(145), px(200), px(255),
      px(095), px(145), px(200),
      px(040), px(095), px(145),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn rotate_180() {
    let mut pixmap = PIXMAP.clone();

    pixmap.rotate_180();

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(040), px(095), px(145),
      px(095), px(145), px(200),
      px(145), px(200), px(255),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn rotate_270() {
    let mut pixmap = PIXMAP.clone();

    pixmap.rotate_270();

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(145), px(095), px(040),
      px(200), px(145), px(095),
      px(255), px(200), px(145),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn scale_up_double() {
    let mut pixmap = PIXMAP.clone();

    pixmap.scale_up(2);

    assert_eq!(pixmap.width, 6);
    assert_eq!(pixmap.height, 6);

    assert_eq!(pixmap.pixels.len(), pixmap.pixels.capacity());

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(255), px(255), px(200), px(200), px(145), px(145),
      px(255), px(255), px(200), px(200), px(145), px(145),
      px(200), px(200), px(145), px(145), px(095), px(095),
      px(200), px(200), px(145), px(145), px(095), px(095),
      px(145), px(145), px(095), px(095), px(040), px(040),
      px(145), px(145), px(095), px(095), px(040), px(040),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn scale_up_triple() {
    let mut pixmap = PIXMAP.clone();

    pixmap.scale_up(3);

    assert_eq!(pixmap.width, 9);
    assert_eq!(pixmap.height, 9);

    assert_eq!(pixmap.pixels.len(), pixmap.pixels.capacity());

    #[rustfmt::skip]
    let expected_pixels = vec![
      px(255), px(255), px(255), px(200), px(200), px(200), px(145), px(145), px(145),
      px(255), px(255), px(255), px(200), px(200), px(200), px(145), px(145), px(145),
      px(255), px(255), px(255), px(200), px(200), px(200), px(145), px(145), px(145),
      px(200), px(200), px(200), px(145), px(145), px(145), px(095), px(095), px(095),
      px(200), px(200), px(200), px(145), px(145), px(145), px(095), px(095), px(095),
      px(200), px(200), px(200), px(145), px(145), px(145), px(095), px(095), px(095),
      px(145), px(145), px(145), px(095), px(095), px(095), px(040), px(040), px(040),
      px(145), px(145), px(145), px(095), px(095), px(095), px(040), px(040), px(040),
      px(145), px(145), px(145), px(095), px(095), px(095), px(040), px(040), px(040),
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn into_iter() {
    let pixmap = TestPixmap {
      width: 2,
      height: 2,
      pixels: vec![px(0), px(1), px(2), px(3)],
    };

    let mut iter = pixmap.into_iter();
    assert_eq!(iter.size_hint(), (4, Some(4)));

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.size_hint(), (3, Some(3)));

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.size_hint(), (2, Some(2)));

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.size_hint(), (1, Some(1)));

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.size_hint(), (0, Some(0)));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
  }
}
