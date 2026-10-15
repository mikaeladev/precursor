mod cursors;
mod package;
mod values;

pub use cursors::*;
pub use package::*;
pub use values::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub package: PackageConfig,
  pub cursors: Vec<CursorConfig>,
}
