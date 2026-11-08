/// Attempts to convert the value to `u8`, returning `0` if an error occurs.
///
/// # Examples
///
/// ```
/// use precursor_ico::into_u8;
///
/// assert_eq!(into_u8!(256_u16), 0_u8);
/// assert_eq!(into_u8!(128_u32), 128_u8);
/// ```
#[macro_export]
macro_rules! into_u8 {
  ($value:expr) => {
    match TryInto::<u8>::try_into($value) {
      Ok(value) => value,
      Err(_) => 0,
    }
  };
}
