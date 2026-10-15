use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId(pub [u8; 4]);

impl ChunkId {
  pub const RIFF: ChunkId = ChunkId(*b"RIFF");
  pub const LIST: ChunkId = ChunkId(*b"LIST");

  // TODO
  pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    if buf.is_ascii() {
      Ok(Self(buf))
    } else {
      Err(io::Error::new(ErrorKind::InvalidData, "invalid ascii"))
    }
  }
}

impl Display for ChunkId {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(std::str::from_utf8(&self.0).unwrap())
  }
}
