/// Converts the given value to `u8`, returning zero (`0`) if it would exceed
/// `u8::MAX`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(wrap_u8!(256 as u16), 0);
/// assert_eq!(wrap_u8!(128 as u16), 128);
/// ```
macro_rules! wrap_u8 {
  ($value:expr) => {
    if $value as usize > u8::MAX as usize {
      0
    } else {
      $value as u8
    }
  };
}

pub(crate) use wrap_u8;

#[cfg(test)]
mod tests {
  #[test]
  fn do_wrap() {
    assert_eq!(wrap_u8!(256 as u16), 0)
  }

  #[test]
  fn dont_wrap() {
    assert_eq!(wrap_u8!(128 as u16), 128)
  }
}
