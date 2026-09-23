use std::io::{self, BufRead, ErrorKind, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::chunk_id::ChunkId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkValue {
  Parent(ChunkId, ChunkId, Vec<Self>),
  Child(ChunkId, Vec<u8>),
}

impl ChunkValue {
  // TODO
  pub fn read<R: BufRead>(reader: &mut R) -> io::Result<Self> {
    let chunk_id = ChunkId::read(reader)?;
    let data_len = reader.read_u32::<LittleEndian>()? as usize;

    let value;

    match chunk_id {
      ChunkId::LIST | ChunkId::RIFF => {
        let parent_kind = chunk_id;

        let parent_id = ChunkId::read(reader)?;
        let mut data_read = 4;

        let mut children = Vec::new();

        while data_read < data_len {
          let child = Self::read(reader)?;

          data_read += child.exact_size();
          children.push(child);
        }

        value = Self::Parent(parent_kind, parent_id, children);
      }
      _ => {
        let mut buffer = Vec::with_capacity(data_len);
        let mut handle = reader.take(data_len as u64);

        handle.read_to_end(&mut buffer)?;

        value = Self::Child(chunk_id, buffer);
      }
    }

    if data_len % 2 != 0 {
      reader.read_u8()?; // skip pad byte
    }

    Ok(value)
  }

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
  pub fn write<W: Write + Seek>(self, writer: &mut W) -> io::Result<usize> {
    let start_pos = writer.stream_position()?;

    let chunk_id;
    let mut data_len;

    writer.write_all(&[0; 8])?; // placeholders

    match self {
      Self::Parent(parent_kind, parent_id, children) => {
        writer.write_all(&parent_id.0)?;

        chunk_id = parent_kind;
        data_len = 4;

        for child in children {
          data_len += child.write(writer)?;
        }
      }
      Self::Child(child_id, data) => {
        chunk_id = child_id;
        data_len = data.len();

        for byte in data {
          writer.write_u8(byte)?;
        }
      }
    }

    if data_len > u32::MAX as usize {
      return Err(io::Error::new(
        ErrorKind::InvalidData,
        "data length exceeds u32::MAX",
      ));
    }

    let end_pos = writer.stream_position()?;

    writer.seek(SeekFrom::Start(start_pos))?;

    writer.write_all(&chunk_id.0)?;
    writer.write_u32::<LittleEndian>(data_len as u32)?;

    writer.seek(SeekFrom::Start(end_pos))?;

    if data_len % 2 != 0 {
      writer.write_all(&[0])?;
    }

    Ok(8 + data_len + (data_len % 2))
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    let data_len = match self {
      Self::Parent(_, _, subchunks) => subchunks
        .iter()
        .fold(4, |acc, subchunk| acc + subchunk.exact_size()),

      Self::Child(_, data) => data.len(),
    };

    8 + data_len + (data_len % 2)
  }
}

#[cfg(test)]
mod tests {
  use std::io::{BufReader, Cursor};

  use super::*;

  const EMPTY_PARENT_BUF: &[u8] = &[
    0x52, 0x49, 0x46, 0x46, // parent kind: RIFF
    0x04, 0x00, 0x00, 0x00, // parent length: 4
    0x4D, 0x45, 0x4F, 0x57, // parent id: MEOW
  ];

  #[test]
  fn read_empty_parent() {
    let chunk = ChunkValue::Parent(ChunkId::RIFF, ChunkId(*b"MEOW"), vec![]);

    let mut reader = BufReader::new(Cursor::new(EMPTY_PARENT_BUF));
    assert_eq!(ChunkValue::read(&mut reader).unwrap(), chunk);
  }

  #[test]
  fn write_empty_parent() {
    let chunk = ChunkValue::Parent(ChunkId::RIFF, ChunkId(*b"MEOW"), vec![]);

    let expected_size = EMPTY_PARENT_BUF.len();
    assert_eq!(chunk.exact_size(), expected_size);

    let mut writer = Cursor::new(Vec::with_capacity(expected_size));
    assert_eq!(chunk.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer.into_inner(), EMPTY_PARENT_BUF);
  }

  #[rustfmt::skip]
  const EMPTY_CHILD_BUF: &[u8] = &[
    0x52, 0x49, 0x46, 0x46, // parent kind: RIFF
    0x18, 0x00, 0x00, 0x00, // parent length: 24
    0x41, 0x43, 0x4F, 0x4E, // parent id: ACON

    0x4C, 0x49, 0x53, 0x54, // parent kind: LIST
    0x0C, 0x00, 0x00, 0x00, // parent length: 12
    0x66, 0x72, 0x61, 0x6D, // parent id: fram

    0x69, 0x63, 0x6F, 0x6E, // child id: icon
    0x00, 0x00, 0x00, 0x00, // child length: 0
  ];

  #[test]
  fn read_empty_child() {
    let chunk = ChunkValue::Parent(
      ChunkId::RIFF,
      ChunkId(*b"ACON"),
      vec![ChunkValue::Parent(
        ChunkId::LIST,
        ChunkId(*b"fram"),
        vec![ChunkValue::Child(ChunkId(*b"icon"), vec![])],
      )],
    );

    let mut reader = BufReader::new(Cursor::new(EMPTY_CHILD_BUF));
    assert_eq!(ChunkValue::read(&mut reader).unwrap(), chunk);
  }

  #[test]
  fn write_empty_child() {
    let chunk = ChunkValue::Parent(
      ChunkId::RIFF,
      ChunkId(*b"ACON"),
      vec![ChunkValue::Parent(
        ChunkId::LIST,
        ChunkId(*b"fram"),
        vec![ChunkValue::Child(ChunkId(*b"icon"), vec![])],
      )],
    );

    let expected_size = EMPTY_CHILD_BUF.len();
    assert_eq!(chunk.exact_size(), expected_size);

    let mut writer = Cursor::new(Vec::with_capacity(expected_size));
    assert_eq!(chunk.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer.into_inner(), EMPTY_CHILD_BUF);
  }
}
