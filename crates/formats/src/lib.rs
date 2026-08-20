mod ani;
mod cur;
mod images;
mod xcursor;

pub use ani::*;
pub use cur::*;
pub use images::*;
pub use xcursor::*;

use std::io::{Result as IoResult, Write};

pub trait WriteTo {
  /// Writes the formatted data to `writer`.
  fn write_to<W: Write>(self, writer: W) -> IoResult<()>;
}
