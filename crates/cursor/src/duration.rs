use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct CursorDuration(u32);

impl CursorDuration {
  pub const ZERO: Self = Self(0);

  const JIFFY: f32 = 16.666666;

  /// Returns the duration in milliseconds.
  pub const fn milliseconds(self) -> u32 {
    self.0
  }

  /// Returns the duration in jiffies.
  pub const fn jiffies(self) -> u32 {
    (self.0 as f32 / Self::JIFFY) as u32
  }

  /// Creates a new `CursorDuration` from a millisecond value.
  pub const fn from_milliseconds(value: u32) -> Self {
    Self(value)
  }

  /// Creates a new `CursorDuration` from a jiffy value.
  pub const fn from_jiffies(value: u32) -> Self {
    Self((value as f32 * Self::JIFFY) as u32)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXPECTED_MS: u32 = 200;
  const EXPECTED_JIF: u32 = 12;

  #[test]
  fn test_ms_to_jiffy() {
    assert_eq!(
      CursorDuration::from_milliseconds(EXPECTED_MS).jiffies(),
      EXPECTED_JIF
    );
  }

  #[test]
  fn test_jiffy_to_ms() {
    assert_eq!(
      CursorDuration::from_jiffies(EXPECTED_JIF).milliseconds(),
      EXPECTED_MS
    );
  }

  #[test]
  fn test_inner_eq() {
    assert_eq!(
      CursorDuration::from_milliseconds(EXPECTED_MS),
      CursorDuration::from_jiffies(EXPECTED_JIF)
    );
  }
}
