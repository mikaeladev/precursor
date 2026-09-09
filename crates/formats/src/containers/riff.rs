use std::io::{Error, ErrorKind, Result, Seek, SeekFrom, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId(pub [u8; 4]);

pub const RIFF_ID: ChunkId = ChunkId(*b"RIFF");
pub const LIST_ID: ChunkId = ChunkId(*b"LIST");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkValue {
  Children(ChunkId, ChunkId, Vec<ChunkValue>),
  Raw(Vec<u8>),
}

impl ChunkValue {
  /// Attempts to write the chunk to `writer`.
  pub fn write<W: Seek + Write>(self, writer: &mut W) -> Result<usize> {
    match self {
      ChunkValue::Children(id, subid, subchunks) => {
        writer.write_all(&id.0)?;

        let data_start = writer.seek(SeekFrom::Current(0))?;
        writer.write_all(&[0, 0, 0, 0])?; // placeholder

        writer.write_all(&subid.0)?;
        let mut data_len = 4;

        for value in subchunks {
          data_len += value.write(writer)?
        }

        if data_len > u32::MAX as usize {
          return Err(Error::new(
            ErrorKind::InvalidData,
            "data length exceeds u32::MAX",
          ));
        }

        let data_end = writer.seek(SeekFrom::Current(0))?;

        writer.seek(SeekFrom::Start(data_start))?;
        writer.write_all(&(data_len as u32).to_le_bytes())?;

        writer.seek(SeekFrom::Start(data_end))?;

        Ok(8 + data_len + (data_len % 2))
      }

      ChunkValue::Raw(bytes) => {
        let data_len = bytes.len();

        writer.write_all(&bytes)?;

        if data_len % 2 != 0 {
          writer.write_all(&[0])?;
        }

        Ok(data_len + (data_len % 2))
      }
    }
  }

  /// Returns the size of the chunk in bytes.
  pub fn size(&self) -> usize {
    match self {
      ChunkValue::Children(_, _, subchunks) => {
        let mut data_len = 4;
        for subchunk in subchunks {
          data_len += subchunk.size();
        }

        8 + data_len + (data_len % 2)
      }

      ChunkValue::Raw(bytes) => {
        let data_len = bytes.len();
        data_len + (data_len % 2)
      }
    }
  }

  /// Constructs a new `RIFF` chunk.
  #[inline]
  pub const fn riff(subid: ChunkId, subchunks: Vec<ChunkValue>) -> Self {
    Self::Children(RIFF_ID, subid, subchunks)
  }

  /// Constructs a new `LIST` chunk.
  #[inline]
  pub const fn list(subid: ChunkId, subchunks: Vec<ChunkValue>) -> Self {
    Self::Children(LIST_ID, subid, subchunks)
  }
}

#[cfg(test)]
mod tests {
  use std::io::Cursor;
  use std::sync::LazyLock;

  use super::*;

  static CHUNKS: LazyLock<ChunkValue> = LazyLock::new(|| {
    ChunkValue::riff(
      ChunkId(*b"ACON"),
      vec![ChunkValue::list(
        ChunkId(*b"fram"),
        vec![ChunkValue::Raw(Vec::from(b"icon"))],
      )],
    )
  });

  #[test]
  fn write() {
    let capacity = CHUNKS.size();

    let mut writer = Cursor::new(Vec::with_capacity(capacity));
    CHUNKS.clone().write(&mut writer).unwrap();

    let mut expected = Vec::with_capacity(capacity);

    expected.write_all(b"RIFF").unwrap();
    expected.write_all(&20_u32.to_le_bytes()).unwrap();

    expected.write_all(b"ACON").unwrap();
    expected.write_all(b"LIST").unwrap();
    expected.write_all(&8_u32.to_le_bytes()).unwrap();

    expected.write_all(b"fram").unwrap();
    expected.write_all(b"icon").unwrap();

    assert_eq!(writer.into_inner(), expected);
  }

  #[test]
  fn size() {
    let capacity = CHUNKS.size();

    let mut writer = Cursor::new(Vec::with_capacity(capacity));
    CHUNKS.clone().write(&mut writer).unwrap();

    assert_eq!(writer.into_inner().len(), capacity);
  }
}
