pub trait ByteArray: Sized + Eq + Clone + Copy {}

impl<const N: usize> ByteArray for [u8; N] {}

pub trait Pixel: Sized + Clone + Copy {
  /// Sized array of bytes (`u8`).
  type ByteArray: ByteArray;

  /// Constructs this pixel from a byte array.
  fn from_bytes(bytes: Self::ByteArray) -> Self;

  /// Converts this pixel into a byte array.
  fn into_bytes(self) -> Self::ByteArray;
}

pub trait FromPixel<P: Pixel> {
  /// Converts the pixel into this type.
  fn from_pixel(pixel: P) -> Self;
}

impl<T: Pixel> FromPixel<T> for T {
  /// No-op.
  #[inline(always)]
  fn from_pixel(pixel: T) -> Self {
    pixel
  }
}

pub trait IntoPixel<P: Pixel> {
  /// Converts this value into a pixel.
  fn into_pixel(self) -> P;
}

impl<T: Pixel, U: Pixel> IntoPixel<U> for T
where
  U: FromPixel<T>,
{
  /// Converts `T` into `U`.
  fn into_pixel(self) -> U {
    U::from_pixel(self)
  }
}
