mod asset;
mod cursor;
mod package;

pub use asset::*;
pub use cursor::*;
pub use package::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub package: PackageConfig,
  pub cursors: Vec<CursorConfig>,
}
