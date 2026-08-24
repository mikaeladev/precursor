use super::{
  IndexedPixmap, LumaAlphaPixel, LumaAlphaPixmap, LumaPixel, LumaPixmap,
  PaletteIndex, Pixel, Pixmap, RgbAlphaPixel, RgbAlphaPixmap, RgbPixel,
  RgbPixmap,
};

pub trait PixmapTransform<P: Pixel>: Pixmap<P> {
  /// Sets a pixel at `(x,y)`.
  ///
  /// # Panics
  ///
  /// Panics if the co-ordinates are out of bounds.
  fn set_pixel(&mut self, x: u32, y: u32, p: P) {
    *self.get_pixel_mut(x, y).unwrap() = p;
  }

  /// Flips the pixmap horizontally.
  fn flip_horizontal(&mut self) {
    let (width, height) = self.dimensions();

    for y in 0..height {
      for x1 in 0..width / 2 {
        let x2 = width - x1 - 1;

        let p1 = *self.get_pixel(x1, y).unwrap();
        let p2 = *self.get_pixel(x2, y).unwrap();

        self.set_pixel(x2, y, p1);
        self.set_pixel(x1, y, p2);
      }
    }
  }

  /// Flips the pixmap vertically.
  fn flip_vertical(&mut self) {
    let (width, height) = self.dimensions();

    for y1 in 0..height / 2 {
      for x in 0..width {
        let y2 = height - y1 - 1;

        let p1 = *self.get_pixel(x, y1).unwrap();
        let p2 = *self.get_pixel(x, y2).unwrap();

        self.set_pixel(x, y2, p1);
        self.set_pixel(x, y1, p2);
      }
    }
  }

  /// Rotates the pixmap by 90°.
  fn rotate_90(&mut self) {
    let (width, height) = self.dimensions();

    let mut out = self.clone();

    for y in 0..height {
      for x in 0..width {
        let p = *self.get_pixel(x, y).unwrap();
        out.set_pixel(height - y - 1, x, p);
      }
    }

    *self = out;
  }

  /// Rotates the pixmap by 180°.
  fn rotate_180(&mut self) {
    let (width, height) = self.dimensions();

    for y1 in 0..height / 2 {
      for x1 in 0..width {
        let x2 = width - x1 - 1;
        let y2 = height - y1 - 1;

        let p1 = *self.get_pixel(x1, y1).unwrap();
        let p2 = *self.get_pixel(x2, y2).unwrap();

        self.set_pixel(x1, y1, p2);
        self.set_pixel(x2, y2, p1);
      }
    }

    if height % 2 != 0 {
      let mid = height / 2;

      for x1 in 0..width / 2 {
        let x2 = width - x1 - 1;

        let p1 = *self.get_pixel(x1, mid).unwrap();
        let p2 = *self.get_pixel(x2, mid).unwrap();

        self.set_pixel(x1, mid, p2);
        self.set_pixel(x2, mid, p1);
      }
    }
  }

  /// Rotates the pixmap by 270°.
  fn rotate_270(&mut self) {
    let (width, height) = self.dimensions();

    let mut out = self.clone();

    for y in 0..height {
      for x in 0..width {
        let p = *self.get_pixel(x, y).unwrap();
        out.set_pixel(y, width - x - 1, p);
      }
    }

    *self = out;
  }
}

impl PixmapTransform<LumaPixel> for LumaPixmap {}
impl PixmapTransform<LumaAlphaPixel> for LumaAlphaPixmap {}
impl PixmapTransform<RgbPixel> for RgbPixmap {}
impl PixmapTransform<RgbAlphaPixel> for RgbAlphaPixmap {}
impl PixmapTransform<PaletteIndex> for IndexedPixmap {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::raster::pixmap::tests::LUMA_PIXELS;

  #[test]
  fn set_pixel() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.set_pixel(0, 0, LumaPixel { y: 040 });
    pixmap.set_pixel(2, 2, LumaPixel { y: 255 });

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 040 }, LumaPixel { y: 200 }, LumaPixel { y: 145 },
      LumaPixel { y: 200 }, LumaPixel { y: 145 }, LumaPixel { y: 095 },
      LumaPixel { y: 145 }, LumaPixel { y: 095 }, LumaPixel { y: 255 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn flip_horizontal() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.flip_horizontal();

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 145 }, LumaPixel { y: 200 }, LumaPixel { y: 255 },
      LumaPixel { y: 095 }, LumaPixel { y: 145 }, LumaPixel { y: 200 },
      LumaPixel { y: 040 }, LumaPixel { y: 095 }, LumaPixel { y: 145 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn flip_horizontal_and_back() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.flip_horizontal();
    pixmap.flip_horizontal();

    assert_eq!(pixmap.pixels, Vec::from(LUMA_PIXELS));
  }

  #[test]
  fn flip_vertical() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.flip_vertical();

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 145 }, LumaPixel { y: 095 }, LumaPixel { y: 040 },
      LumaPixel { y: 200 }, LumaPixel { y: 145 }, LumaPixel { y: 095 },
      LumaPixel { y: 255 }, LumaPixel { y: 200 }, LumaPixel { y: 145 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn flip_vertical_and_back() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.flip_vertical();
    pixmap.flip_vertical();

    assert_eq!(pixmap.pixels, Vec::from(LUMA_PIXELS));
  }

  #[test]
  fn flip_horizontal_vertical() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.flip_horizontal();
    pixmap.flip_vertical();

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 040 }, LumaPixel { y: 095 }, LumaPixel { y: 145 },
      LumaPixel { y: 095 }, LumaPixel { y: 145 }, LumaPixel { y: 200 },
      LumaPixel { y: 145 }, LumaPixel { y: 200 }, LumaPixel { y: 255 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn rotate_90() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.rotate_90();

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 145 }, LumaPixel { y: 200 }, LumaPixel { y: 255 },
      LumaPixel { y: 095 }, LumaPixel { y: 145 }, LumaPixel { y: 200 },
      LumaPixel { y: 040 }, LumaPixel { y: 095 }, LumaPixel { y: 145 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn rotate_180() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.rotate_180();

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 040 }, LumaPixel { y: 095 }, LumaPixel { y: 145 },
      LumaPixel { y: 095 }, LumaPixel { y: 145 }, LumaPixel { y: 200 },
      LumaPixel { y: 145 }, LumaPixel { y: 200 }, LumaPixel { y: 255 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }

  #[test]
  fn rotate_270() {
    let mut pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    pixmap.rotate_270();

    #[rustfmt::skip]
    let expected_pixels = vec![
      LumaPixel { y: 145 }, LumaPixel { y: 095 }, LumaPixel { y: 040 },
      LumaPixel { y: 200 }, LumaPixel { y: 145 }, LumaPixel { y: 095 },
      LumaPixel { y: 255 }, LumaPixel { y: 200 }, LumaPixel { y: 145 },
    ];

    assert_eq!(pixmap.pixels, expected_pixels);
  }
}
