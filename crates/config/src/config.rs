use std::collections::HashMap;

use crate_cursors::{CursorDuration, CursorHotspot};
use serde::Deserialize;

use crate::{AssetConfig, LocaleString, PlatformAlias};

#[derive(Deserialize)]
pub struct Config {
  pub package: PackageConfig,
  pub cursors: HashMap<String, CursorConfig>,
}

#[derive(Deserialize)]
pub struct PackageConfig {
  /// Short name for the cursor theme.
  pub name: LocaleString,
  /// Long description for the cursor theme.
  pub comment: LocaleString,
  /// Whether to hide the cursor theme in selection UIs, usually enabled for
  /// fallback themes. Only applies when packaged for Linux.
  pub hidden: Option<bool>,
  /// Name of a specific cursor to use as an example in selection UIs. Only
  /// applies when packaged for Linux.
  pub example: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CursorConfig {
  ScaledStatic {
    /// Nominal size of the cursor.
    nominal: u32,
    /// Co-ordinates of the cursor tip.
    hotspot: CursorHotspot,
    /// Path to an asset file (with optional transforms).
    asset: AssetConfig,
    /// Alternate names for the cursor.
    aliases: Option<Vec<PlatformAlias>>,
  },
  ScaledAnimated {
    /// Nominal size of the cursor.
    nominal: u32,
    /// Co-ordinates of the cursor tip.
    hotspot: CursorHotspot,
    /// Duration of the animation in full.
    duration: Option<CursorDuration>,
    /// Durations for each frame in the animation.
    durations: Option<Vec<CursorDuration>>,
    /// Sequence of frames (with optional overrides).
    sequence: Vec<ScaledCursorFrameConfig>,
    /// Paths to asset files (with optional transforms).
    assets: Vec<AssetConfig>,
    /// Alternate names for the cursor.
    aliases: Option<Vec<PlatformAlias>>,
  },
}

#[derive(Deserialize)]
pub struct ScaledCursorFrameConfig {
  /// Index of an asset in `super::assets`.
  pub asset: usize,
  /// Nominal size of the frame (overrides `super::nominal`).
  pub nominal: Option<u32>,
  /// Co-ordinates of the cursor tip (overrides `super::hotspot`).
  pub hotspot: Option<CursorHotspot>,
  /// Duration of this frame (conflicts with `super::durations`).
  pub duration: Option<CursorDuration>,
}
