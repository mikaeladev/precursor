use crate_config::VerboseFrame;

use crate::error::PrecursorResult;

use super::CursorIcon;

#[derive(Debug, Clone)]
pub struct CursorFrame {
  pub icons: Vec<CursorIcon>,
  pub duration: Option<CursorDuration>,
}

impl CursorFrame {
  // TODO: document
  pub fn from_config(
    VerboseFrame {
      icons: icon_configs,
      duration,
    }: VerboseFrame,
  ) -> PrecursorResult<Self> {
    let mut icons = Vec::with_capacity(icon_configs.len());

    for icon_config in icon_configs {
      icons.push(CursorIcon::from_config(icon_config)?);
    }

    Ok(CursorFrame {
      icons,
      duration: Some(CursorDuration::new(duration)),
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorDuration(u32);

impl CursorDuration {
  const JIFFY: f32 = 16.666666;

  /// Creates a new `CursorDuration`.
  pub const fn new(ms: u32) -> Self {
    Self(ms)
  }

  /// Returns the duration in milliseconds.
  pub const fn milliseconds(self) -> u32 {
    self.0
  }

  /// Returns the duration in jiffies.
  pub const fn jiffies(self) -> u32 {
    (self.0 as f32 / Self::JIFFY) as u32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn duration_to_jiffy() {
    assert_eq!(CursorDuration::new(200).jiffies(), 12);
  }
}
