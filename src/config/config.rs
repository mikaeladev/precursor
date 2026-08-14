use std::collections::HashMap;
use std::num::NonZero;
use std::path::PathBuf;

use serde::Deserialize;

use crate::config::{LocaleString, PlatformAlias};
use crate::cursors::{CursorDuration, CursorHotspot};

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
  #[serde(default)]
  pub hidden: Option<bool>,
  /// Name of a specific cursor to use as an example in selection UIs. Only
  /// applies when packaged for Linux.
  #[serde(default)]
  pub example: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CursorConfig {
  Static {
    /// Size of the cursor.
    size: NonZero<u16>,
    /// Co-ordinates of the cursor tip.
    hotspot: CursorHotspot,
    /// Path to the asset file.
    asset: PathBufOrAssetConfig,
    /// List of platform aliases.
    aliases: Option<Vec<PlatformAlias>>,
  },
  Animated {
    /// Size of the cursor.
    size: NonZero<u16>,
    /// List of asset files.
    assets: Vec<PathBufOrAssetConfig>,
    /// List of animation frames.
    frames: Vec<FrameConfig>,
    /// List of platform aliases.
    aliases: Option<Vec<PlatformAlias>>,
  },
}

#[derive(Default, Deserialize)]
pub struct AssetConfig {
  /// Path to the image file.
  pub path: PathBuf,
  /// Whether the image should be horizontally flipped.
  pub flip: Option<bool>,
  /// Whether the image should be vertically flipped.
  pub flop: Option<bool>,
  /// How much the image should be rotated.
  pub rotate: Option<i32>,
}

#[derive(Deserialize)]
pub struct FrameConfig {
  /// Which asset to use from the list (0-based).
  pub index: u16,
  /// Co-ordinates of the cursor tip.
  pub hotspot: CursorHotspot,
  /// How long this frame should last (in milliseconds).
  pub duration: CursorDuration,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum PathBufOrAssetConfig {
  PathBuf(PathBuf),
  AssetConfig(AssetConfig),
}

impl From<PathBuf> for AssetConfig {
  fn from(path: PathBuf) -> Self {
    AssetConfig {
      path,
      ..Default::default()
    }
  }
}

impl From<PathBufOrAssetConfig> for AssetConfig {
  fn from(value: PathBufOrAssetConfig) -> Self {
    match value {
      PathBufOrAssetConfig::AssetConfig(ac) => ac,
      PathBufOrAssetConfig::PathBuf(path) => path.into(),
    }
  }
}
