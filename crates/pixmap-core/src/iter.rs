use std::vec;

use crate::Pixel;

pub struct IntoIter<T: Pixel, const N: usize>
where
  T::ByteArray: Into<[u8; N]>,
{
  pixels: vec::IntoIter<T>,
  bytes: [u8; N],
  index: usize,
}

impl<T: Pixel, const N: usize> IntoIter<T, N>
where
  T::ByteArray: Into<[u8; N]>,
{
  /// Constructs a new iterator from a `Vec<T>`.
  pub fn new(pixels: Vec<T>) -> Self {
    Self {
      pixels: pixels.into_iter(),
      bytes: [0; N],
      index: N,
    }
  }
}

impl<T: Pixel, const N: usize> Iterator for IntoIter<T, N>
where
  T::ByteArray: Into<[u8; N]>,
{
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == N {
      self.pixels.next().and_then(|pixel| {
        self.bytes = pixel.into_bytes().into();
        self.index = 0;
        self.next()
      })
    } else {
      let byte = Some(self.bytes[self.index]);
      self.index += 1;
      byte
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let (pixels_lower, _) = self.pixels.size_hint();
    let exact_size = pixels_lower * N - (N - self.index);
    (exact_size, Some(exact_size))
  }
}
