use crate::Pixel;

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

  /// Scales the pixmap up by `factor`.
  fn scale_up(&mut self, factor: usize);

  /// Returns the width and height of the pixmap as a tuple.
  fn dimensions(&self) -> (u32, u32) {
    (self.width(), self.height())
  }

  /// Returns `Some` reference to the pixel at `(x,y)`.
  ///
  /// # Options
  ///
  /// Returns `None` if the co-ordinates are out of bounds.
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
  /// # Options
  ///
  /// Returns `None` if the co-ordinates are out of bounds.
  fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Self::Pixel> {
    let i = self.get_pixel_index(x, y)?;
    self.pixels_mut().get_mut(i)
  }

  /// Returns `Some` index of the pixel at `(x,y)`.
  ///
  /// # Options
  ///
  /// Returns `None` if the co-ordinates are out of bounds.
  fn get_pixel_index(&self, x: u32, y: u32) -> Option<usize> {
    let (width, height) = self.dimensions();

    if x >= width || y >= height {
      return None;
    }

    Some(
      (y as usize * width as usize + x as usize)
        * size_of::<<Self::Pixel as Pixel>::ByteArray>(),
    )
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

pub trait FromPixmap<P: Pixmap> {
  /// Converts the pixmap into this type.
  fn from_pixmap(pixmap: P) -> Self;
}

pub trait IntoPixmap<P: Pixmap> {
  /// Converts this value into a pixmap.
  fn into_pixmap(self) -> P;
}

impl<T: Pixmap> FromPixmap<T> for T {
  /// No-op.
  #[inline(always)]
  fn from_pixmap(pixmap: T) -> Self {
    pixmap
  }
}

impl<T: Pixmap, U: Pixmap> IntoPixmap<U> for T
where
  U: FromPixmap<T>,
{
  /// Converts `T` into `U`.
  fn into_pixmap(self) -> U {
    U::from_pixmap(self)
  }
}
