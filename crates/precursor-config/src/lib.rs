pub mod cursors;
pub mod theme;

use serde::Deserialize;

pub use cursors::*;
pub use theme::*;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub theme: Theme,
  pub cursors: Cursors,
}
