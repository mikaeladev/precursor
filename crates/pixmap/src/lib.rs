mod convert;
mod dynamic;
mod error;

pub mod pixels;

use std::iter::FlatMap;
use std::vec::IntoIter;

pub use convert::*;
pub use dynamic::*;
pub use error::*;

use pixels::*;

pub trait Pixmap: Clone + PartialEq + Eq {
  type Pixel: Pixel;

  /// Returns the width of the pixmap.
  fn width(&self) -> u32;

  /// Returns the height of the pixmap.
  fn height(&self) -> u32;

  /// Returns a slice of the underlying pixel `Vec`.
  fn pixels(&self) -> &[Self::Pixel];

  /// Returns a mutable slice of the underlying pixel `Vec`.
  fn pixels_mut(&mut self) -> &mut [Self::Pixel];

  /// Returns the width and height of the pixmap as a tuple.
  fn dimensions(&self) -> (u32, u32) {
    (self.width(), self.height())
  }

  /// Returns `Some` reference to the pixel at `(x,y)`.
  ///
  /// # Notes
  ///
  /// Is `None` if the co-ordinates are out of bounds.
  fn get_pixel(&self, x: u32, y: u32) -> Option<&Self::Pixel> {
    self.pixels().get(self.get_pixel_index(x, y)?)
  }

  /// Returns a reference to the pixel at `(x,y)`.
  ///
  /// # Panics
  ///
  /// Panics if the co-ordinates are out of bounds.
  fn get_pixel_unchecked(&self, x: u32, y: u32) -> &Self::Pixel {
    let i = self.get_pixel_index(x, y).unwrap();
    self.pixels().get(i).unwrap()
  }

  /// Returns `Some` mutable reference to the pixel at `(x,y)`.
  ///
  /// # Notes
  ///
  /// Is `None` if the co-ordinates are out of bounds.
  fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Pixel> {
    let i = self.get_pixel_index(x, y)?;
    self.pixels_mut().get_mut(i)
  }

  /// Returns `Some` index of the pixel at `(x,y)`.
  ///
  /// # Notes
  ///
  /// Is `None` if the co-ordinates are out of bounds.
  fn get_pixel_index(&self, x: u32, y: u32) -> Option<usize> {
    let (width, height) = self.dimensions();

    if x >= width || y >= height {
      return None;
    }

    Some((y as usize * width as usize + x as usize) * Self::Pixel::CHANNELS)
  }

  /// Returns the index of the pixel at `(x,y)`.
  ///
  /// # Panics
  ///
  /// Panics if the co-ordinates are out of bounds.
  fn get_pixel_index_unchecked(&self, x: u32, y: u32) -> usize {
    self.get_pixel_index(x, y).unwrap()
  }

  /// Sets a pixel at `(x,y)`.
  ///
  /// # Panics
  ///
  /// Panics if the co-ordinates are out of bounds.
  fn set_pixel(&mut self, x: u32, y: u32, p: Self::Pixel) {
    *self.get_pixel_mut(x, y).unwrap() = p.into();
  }

  /// Flips the pixmap horizontally.
  fn flip_horizontal(&mut self) {
    let (width, height) = self.dimensions();

    for y in 0..height {
      for x1 in 0..width / 2 {
        let x2 = width - x1 - 1;

        let p1 = *self.get_pixel_unchecked(x1, y);
        let p2 = *self.get_pixel_unchecked(x2, y);

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

        let p1 = *self.get_pixel_unchecked(x, y1);
        let p2 = *self.get_pixel_unchecked(x, y2);

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
        let p = *self.get_pixel_unchecked(x, y);
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

        let p1 = *self.get_pixel_unchecked(x1, y1);
        let p2 = *self.get_pixel_unchecked(x2, y2);

        self.set_pixel(x1, y1, p2);
        self.set_pixel(x2, y2, p1);
      }
    }

    if height % 2 != 0 {
      let mid = height / 2;

      for x1 in 0..width / 2 {
        let x2 = width - x1 - 1;

        let p1 = *self.get_pixel_unchecked(x1, mid);
        let p2 = *self.get_pixel_unchecked(x2, mid);

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
        let p = *self.get_pixel_unchecked(x, y);
        out.set_pixel(y, width - x - 1, p);
      }
    }

    *self = out;
  }
}

type PixmapIter<T, const N: usize> = FlatMap<
  IntoIter<<T as Pixmap>::Pixel>,
  [u8; N],
  Box<dyn FnMut(<T as Pixmap>::Pixel) -> [u8; N]>,
>;

// -------------------------------------------------------------------------- //

macro_rules! pixmap {
  ($pixmap:ident<$pixel:ident<$n:literal>>) => {
    pixmap!(@gen $pixmap<$pixel<$n>>, { });
  };

  ($pixmap:ident<$pixel:ident<$n:literal>>, $fields:tt) => {
    pixmap!(@gen $pixmap<$pixel<$n>>, $fields);
  };

  (@gen $pixmap:ident<$pixel:ident<$n:literal>>, { $( $ident:ident; $type:ty ),* }) => {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct $pixmap {
      width: u32,
      height: u32,
      pixels: Vec<$pixel>,
      $( $ident: $type, )*
    }

    impl_new!($pixmap, { $( $ident; $type ),* });
    impl_iter!($pixmap, $n);
    impl_pixmap!($pixmap, $pixel);
  };
}

macro_rules! impl_new {
  // ident
  ($pixmap:ident) => {
    impl_new!(@gen $pixmap, { }, impl_new!(@doc $pixmap));
  };

  // ident + extra fields
  ($pixmap:ident, $fields:tt) => {
    impl_new!(@gen $pixmap, $fields, impl_new!(@doc $pixmap));
  };

  (@doc $pixmap:ident) => {
    concat!("Creates a new `", stringify!($pixmap), "`.")
  };

  (@gen $pixmap:ident, { $( $ident:ident; $type:ty ),* }, $doc:expr) => {
    impl $pixmap {
      #[doc = $doc]
      ///
      /// # Notes
      ///
      /// Returns an `Err` if the number of pixels `≠` the width `*` the
      /// height.
      pub fn new(
        width: u32,
        height: u32,
        pixels: Vec<<Self as Pixmap>::Pixel>,
        $( $ident: $type, )*
      ) -> Result<Self, PixmapError> {
        let expected_pixels = width as usize * height as usize;
        let actual_pixels = pixels.len();

        if expected_pixels != actual_pixels {
          return Err(PixmapError::WrongDimensions(
            expected_pixels,
            actual_pixels,
          ));
        }

        Ok(Self {
          width,
          height,
          pixels,
          $( $ident, )*
        })
      }
    }
  };
}

macro_rules! impl_iter {
  ($pixmap:ident, $n:literal) => {
    impl IntoIterator for $pixmap {
      type Item = u8;
      type IntoIter = PixmapIter<Self, $n>;

      fn into_iter(self) -> Self::IntoIter {
        self
          .pixels
          .into_iter()
          .flat_map(Box::new(|px| px.into_bytes()))
      }
    }
  };
}

macro_rules! impl_pixmap {
  ($pixmap:ident, $pixel:ident) => {
    impl Pixmap for $pixmap {
      type Pixel = $pixel;

      fn width(&self) -> u32 {
        self.width
      }

      fn height(&self) -> u32 {
        self.height
      }

      fn pixels(&self) -> &[Self::Pixel] {
        &self.pixels
      }

      fn pixels_mut(&mut self) -> &mut [Self::Pixel] {
        &mut self.pixels
      }
    }
  };
}

// -------------------------------------------------------------------------- //

pixmap!(LumaPixmap<LumaPixel<1>>);

pixmap!(LumaAlphaPixmap<LumaAlphaPixel<2>>);

pixmap!(RgbPixmap<RgbPixel<3>>);

pixmap!(RgbAlphaPixmap<RgbAlphaPixel<4>>);

pixmap!(IndexedPixmap<IndexedPixel<1>>, {
  palette; Vec::<RgbPixel>,
  trns; Option::<Vec<u8>>
});

impl IndexedPixmap {
  /// Returns a slice of the palette `Vec`.
  pub const fn palette(&self) -> &[RgbPixel] {
    self.palette.as_slice()
  }

  /// Returns `Some` slice of the trns `Vec`.
  pub const fn trns(&self) -> Option<&[u8]> {
    if let Some(trns) = &self.trns {
      Some(trns.as_slice())
    } else {
      None
    }
  }
}

// -------------------------------------------------------------------------- //

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

  #[test]
  fn into_iter() {
    let pixmap = LumaPixmap::new(3, 3, Vec::from(LUMA_PIXELS)).unwrap();

    #[rustfmt::skip]
    let expected_pixels = vec![
      255, 200, 145,
      200, 145, 095,
      145, 095, 040,
    ];

    assert_eq!(pixmap.into_iter().collect::<Vec<u8>>(), expected_pixels);
  }
}
