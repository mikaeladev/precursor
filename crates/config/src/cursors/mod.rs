mod variants;

pub use variants::*;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CursorConfig {
  pub name: String,
  pub targets: Option<CursorTargets>,
  #[serde(flatten)]
  pub variant: CursorVariant,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct CursorTargets {
  pub linux: Option<LinuxTargetConfig>,
  pub windows: Option<WindowsTargetConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct LinuxTargetConfig {
  pub name: Option<String>,
  pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct WindowsTargetConfig {
  pub name: Option<String>,
}
