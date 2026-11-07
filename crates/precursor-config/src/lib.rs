pub mod cursors;
pub mod theme;

use serde::Deserialize;

pub use cursors::*;
pub use theme::*;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub theme: Option<Theme>,
  pub cursors: Cursors,
}
