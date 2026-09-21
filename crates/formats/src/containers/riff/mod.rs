use std::io::{self, ErrorKind, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId(pub [u8; 4]);

pub const RIFF_ID: ChunkId = ChunkId(*b"RIFF");
pub const LIST_ID: ChunkId = ChunkId(*b"LIST");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkValue {
  Children(ChunkId, ChunkId, Vec<Self>),
  Raw(Vec<u8>),
}

impl ChunkValue {
  /// Writes this chunk to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  ///
  /// Additionally, if the length of a chunk exceeds `u32::MAX`, the operation
  /// will fail with [`ErrorKind::InvalidData`].
  ///
  /// [`Write::write_all`]: Write::write_all
  /// [`ErrorKind::InvalidData`]: ErrorKind::InvalidData
  pub fn write<W: Seek + Write>(self, writer: &mut W) -> io::Result<usize> {
    match self {
      Self::Children(id, subid, subchunks) => {
        writer.write_all(&id.0)?;

        let data_start = writer.seek(SeekFrom::Current(0))?;
        writer.write_all(&[0, 0, 0, 0])?; // placeholder

        writer.write_all(&subid.0)?;
        let mut data_len = 4;

        for value in subchunks {
          data_len += value.write(writer)?
        }

        if data_len > u32::MAX as usize {
          return Err(io::Error::new(
            ErrorKind::InvalidData,
            "data length exceeds u32::MAX",
          ));
        }

        let data_end = writer.seek(SeekFrom::Current(0))?;

        writer.seek(SeekFrom::Start(data_start))?;
        writer.write_u32::<LittleEndian>(data_len as u32)?;

        writer.seek(SeekFrom::Start(data_end))?;

        Ok(8 + data_len + (data_len % 2))
      }

      Self::Raw(bytes) => {
        let data_len = bytes.len();

        writer.write_all(&bytes)?;

        if data_len % 2 != 0 {
          writer.write_all(&[0])?;
        }

        Ok(data_len + (data_len % 2))
      }
    }
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    match self {
      Self::Children(_, _, subchunks) => {
        let mut data_len = 4;
        for subchunk in subchunks {
          data_len += subchunk.exact_size();
        }

        8 + data_len + (data_len % 2)
      }

      Self::Raw(bytes) => {
        let data_len = bytes.len();
        data_len + (data_len % 2)
      }
    }
  }

  /// Constructs a new `RIFF` chunk.
  #[inline]
  pub const fn riff(subid: ChunkId, subchunks: Vec<Self>) -> Self {
    Self::Children(RIFF_ID, subid, subchunks)
  }

  /// Constructs a new `LIST` chunk.
  #[inline]
  pub const fn list(subid: ChunkId, subchunks: Vec<Self>) -> Self {
    Self::Children(LIST_ID, subid, subchunks)
  }
}

#[cfg(test)]
mod tests {
  use std::io::{self, Cursor};

  use super::*;

  #[test]
  fn empty_children() {
    let chunk = ChunkValue::riff(ChunkId(*b"MEOW"), vec![]);

    let expected_buffer: &[u8] = &[
      0x52, 0x49, 0x46, 0x46, // riff id: RIFF
      0x04, 0x00, 0x00, 0x00, // riff length: 4
      0x4D, 0x45, 0x4F, 0x57, // riff subchunk id: MEOW
    ];

    let expected_size = expected_buffer.len();
    assert_eq!(chunk.exact_size(), expected_size);

    let mut buffer = Cursor::new(Vec::with_capacity(expected_size));
    assert_eq!(chunk.write(&mut buffer).unwrap(), expected_size);
    assert_eq!(&buffer.into_inner(), expected_buffer);
  }

  #[test]
  fn empty_raw() {
    let chunk = ChunkValue::Raw(Vec::new());

    assert_eq!(chunk.exact_size(), 0);
    assert_eq!(chunk.write(&mut io::empty()).unwrap(), 0);
  }

  #[test]
  fn single_child() {
    let chunk = ChunkValue::riff(
      ChunkId(*b"ACON"),
      vec![ChunkValue::list(
        ChunkId(*b"fram"),
        vec![ChunkValue::Raw(Vec::from(b"icon"))],
      )],
    );

    let expected_buffer: &[u8] = &[
      0x52, 0x49, 0x46, 0x46, // riff id: RIFF
      0x14, 0x00, 0x00, 0x00, // riff length: 20
      0x41, 0x43, 0x4F, 0x4E, // riff subchunk id: ACON
      0x4C, 0x49, 0x53, 0x54, // list id: LIST
      0x08, 0x00, 0x00, 0x00, // list length: 8
      0x66, 0x72, 0x61, 0x6D, // list subchunk id: fram
      0x69, 0x63, 0x6F, 0x6E, // list subchunk data: icon
    ];

    let expected_size = expected_buffer.len();
    assert_eq!(chunk.exact_size(), expected_size);

    let mut buffer = Cursor::new(Vec::with_capacity(expected_size));
    assert_eq!(chunk.write(&mut buffer).unwrap(), expected_size);
    assert_eq!(&buffer.into_inner(), expected_buffer);
  }
}
