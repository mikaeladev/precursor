use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId(pub [u8; 4]);

impl ChunkId {
  pub const RIFF: ChunkId = ChunkId(*b"RIFF");
  pub const LIST: ChunkId = ChunkId(*b"LIST");

  // TODO
  pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(Self(buf))
  }
}
