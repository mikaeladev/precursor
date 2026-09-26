mod decode;
mod encode;
mod error;

pub use decode::*;
pub use encode::*;
pub use error::*;

pub const PNG_MAGIC: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
