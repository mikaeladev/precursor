mod image;
mod windows;
mod xcursor;

use clap::ValueEnum;

pub use image::*;
pub use windows::*;
pub use xcursor::*;

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum CursorType {
  Scalable,
  Windows,
  Xcursor,
}

#[derive(Clone, Copy)]
pub enum CursorDuration {
  Milliseconds(u32),
  Jiffies(u32),
}

impl CursorDuration {
  pub const ZERO: Self = CursorDuration::Milliseconds(0);

  /// Converts the duration to a millisecond value.
  pub const fn milliseconds(self) -> u32 {
    match self {
      Self::Milliseconds(val) => val,
      Self::Jiffies(val) => val * (1000 / 60),
    }
  }

  /// Converts the duration to a jiffy value.
  pub const fn jiffies(self) -> u32 {
    match self {
      Self::Milliseconds(val) => val / (1000 / 60),
      Self::Jiffies(val) => val,
    }
  }
}
