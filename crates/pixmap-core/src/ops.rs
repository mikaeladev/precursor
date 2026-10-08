use crate::Pixel;

/// Scales a pixmap in-place by a `factor`.
///
/// # Panics
///
/// Panics if `factor < 2`, or if the new length exceeds `isize::MAX`.
pub fn scale_up_in_place<P: Pixel>(
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
