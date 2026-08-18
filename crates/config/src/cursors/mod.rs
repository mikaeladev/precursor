mod alias;
mod asset;
mod frame;

pub use alias::*;
pub use asset::*;
pub use frame::*;

use crate_cursors::{CursorDuration, CursorHotspot};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CursorConfig {
  ScaledStatic(ScaledStaticCursorConfig),
  ScaledAnimated(ScaledAnimatedCursorConfig),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ScaledStaticCursorConfig {
  /// Nominal size of the cursor.
  pub nominal: u32,
  /// Co-ordinates of the cursor tip.
  pub hotspot: CursorHotspot,
  /// Path to an asset file (with optional transforms).
  pub asset: AssetConfig,
  /// Alternate names for the cursor.
  pub aliases: Option<Vec<PlatformAlias>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ScaledAnimatedCursorConfig {
  /// Nominal size of the cursor.
  pub nominal: u32,
  /// Co-ordinates of the cursor tip.
  pub hotspot: CursorHotspot,
  /// Duration of the animation in full.
  pub duration: Option<CursorDuration>,
  /// Durations for each frame in the animation.
  pub durations: Option<Vec<CursorDuration>>,
  /// Sequence of frames (with optional overrides).
  pub sequence: Vec<ScaledCursorFrameConfig>,
  /// Paths to asset files (with optional transforms).
  pub assets: Vec<AssetConfig>,
  /// Alternate names for the cursor.
  pub aliases: Option<Vec<PlatformAlias>>,
}

impl From<ScaledStaticCursorConfig> for CursorConfig {
  fn from(value: ScaledStaticCursorConfig) -> Self {
    Self::ScaledStatic(value)
  }
}

impl From<ScaledAnimatedCursorConfig> for CursorConfig {
  fn from(value: ScaledAnimatedCursorConfig) -> Self {
    Self::ScaledAnimated(value)
  }
}

// TODO: tests
