use crate::pixels::Pixel;

/// Resizes the pixmap in-place by a `factor`.
///
/// # Panics
///
/// Panics if `factor < 2`, or if the new length exceeds `isize::MAX`.
pub fn scale_pixmap<P: Pixel>(
  width: &mut u32,
  height: &mut u32,
  pixels: &mut Vec<P>,
  factor: usize,
) {
  assert!(factor > 1, "factor should be > 1");

  *width *= factor as u32;
  *height *= factor as u32;

  let row_len = *width as usize;
  let col_len = *height as usize;

  let additional = row_len * col_len - pixels.len();
  pixels.reserve_exact(additional);

  // horizontal
  let mut i = 0;
  loop {
    let pixel = pixels[i];
    for _ in 1..=(factor - 1) {
      i += 1;
      pixels.insert(i, pixel);
    }

    i += 1;
    if i >= (row_len * col_len) / factor {
      break;
    }
  }

  // vertical
  let mut i = 0;
  loop {
    for _ in 1..=(factor - 1) {
      for _ in 1..=row_len {
        let pixel = pixels[i];
        pixels.insert(row_len + i, pixel);
        i += 1;
      }
    }

    i += row_len;
    if i >= row_len * col_len {
      break;
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::LumaPixel;
  use crate::tests::LUMA_PIXELS;

  use super::*;

  macro_rules! px {
    ($y:literal) => {
      LumaPixel { y: $y }
    };
  }

  #[test]
  fn scale_double() {
    let mut width = 3;
    let mut height = 3;
    let mut pixels = Vec::from(LUMA_PIXELS);

    scale_pixmap(&mut width, &mut height, &mut pixels, 2);

    assert_eq!(width, 6);
    assert_eq!(height, 6);

    assert_eq!(pixels.len(), pixels.capacity());

    #[rustfmt::skip]
    let expected_pixels = vec![
      px!(255), px!(255), px!(200), px!(200), px!(145), px!(145),
      px!(255), px!(255), px!(200), px!(200), px!(145), px!(145),
      px!(200), px!(200), px!(145), px!(145), px!(095), px!(095),
      px!(200), px!(200), px!(145), px!(145), px!(095), px!(095),
      px!(145), px!(145), px!(095), px!(095), px!(040), px!(040),
      px!(145), px!(145), px!(095), px!(095), px!(040), px!(040),
    ];

    assert_eq!(pixels, expected_pixels);
  }

  #[test]
  fn scale_triple() {
    let mut width = 3;
    let mut height = 3;
    let mut pixels = Vec::from(LUMA_PIXELS);

    scale_pixmap(&mut width, &mut height, &mut pixels, 3);

    assert_eq!(width, 9);
    assert_eq!(height, 9);

    assert_eq!(pixels.len(), pixels.capacity());

    #[rustfmt::skip]
    let expected_pixels = vec![
      px!(255), px!(255), px!(255), px!(200), px!(200), px!(200), px!(145), px!(145), px!(145),
      px!(255), px!(255), px!(255), px!(200), px!(200), px!(200), px!(145), px!(145), px!(145),
      px!(255), px!(255), px!(255), px!(200), px!(200), px!(200), px!(145), px!(145), px!(145),
      px!(200), px!(200), px!(200), px!(145), px!(145), px!(145), px!(095), px!(095), px!(095),
      px!(200), px!(200), px!(200), px!(145), px!(145), px!(145), px!(095), px!(095), px!(095),
      px!(200), px!(200), px!(200), px!(145), px!(145), px!(145), px!(095), px!(095), px!(095),
      px!(145), px!(145), px!(145), px!(095), px!(095), px!(095), px!(040), px!(040), px!(040),
      px!(145), px!(145), px!(145), px!(095), px!(095), px!(095), px!(040), px!(040), px!(040),
      px!(145), px!(145), px!(145), px!(095), px!(095), px!(095), px!(040), px!(040), px!(040),
    ];

    assert_eq!(pixels, expected_pixels);
  }
}
