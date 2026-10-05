use std::io::{self, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::chunk::XcursorChunk;
use super::entry::XcursorTocEntry;
use super::error::{ReadError, ReadResult};

#[derive(Debug)]
pub struct XcursorFile {
  chunks: Vec<XcursorChunk>,
}

impl XcursorFile {
  pub const MAGIC: &[u8] = b"Xcur";

  pub(super) const HEADER_SIZE: usize = 16;
  pub(super) const VERSION: u32 = 0x10000;

  /// Constructs a new `XcursorFile`.
  ///
  /// # Panics
  ///
  /// Panics in the following situations:
  ///
  /// - `chunks` is empty;
  /// - `chunks` exceeds `u32::MAX` in length;
  /// - Any individal chunk exceeds `u32::MAX` in size.
  pub fn new(chunks: Vec<XcursorChunk>) -> Self {
    const MAX: usize = u32::MAX as usize;

    assert!(!chunks.is_empty(), "chunks should not be empty");

    assert!(
      chunks.len() <= MAX,
      "chunks length should not exceed u32::MAX"
    );

    for chunk in &chunks {
      assert!(
        chunk.exact_size() <= MAX,
        "chunk size should not exceed u32::MAX"
      )
    }

    Self { chunks }
  }

  pub fn read<R: Read + Seek>(reader: &mut R) -> ReadResult<Self> {
    let mut magic_buf = [0; 4];
    reader.read_exact(&mut magic_buf)?;

    if magic_buf != Self::MAGIC {
      return Err(ReadError::MissingMagicNumber(magic_buf));
    }

    if let file_header_size = reader.read_u32::<LittleEndian>()? as usize
      && file_header_size != Self::HEADER_SIZE
    {
      return Err(ReadError::InvalidFileHeaderSize(file_header_size));
    }

    if let file_version = reader.read_u32::<LittleEndian>()?
      && file_version != Self::VERSION
    {
      return Err(ReadError::UnsupportedFileVersion(file_version));
    }

    let mut num_entries = reader.read_u32::<LittleEndian>()? as usize;
    let mut entries = Vec::with_capacity(num_entries);

    while num_entries > 0 {
      entries.push(XcursorTocEntry::read(reader)?);
      num_entries -= 1;
    }

    let mut chunks = Vec::with_capacity(entries.len());

    for entry in entries {
      let chunk_pos = entry.offset() as u64;
      reader.seek(SeekFrom::Start(chunk_pos))?;
      chunks.push(XcursorChunk::read(reader)?);
    }

    Ok(Self { chunks })
  }

  /// Writes an Xcursor file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    let chunks_len = self.chunks.len();

    writer.write_all(Self::MAGIC)?;
    writer.write_u32::<LittleEndian>(Self::HEADER_SIZE as u32)?;
    writer.write_u32::<LittleEndian>(Self::VERSION)?;
    writer.write_u32::<LittleEndian>(chunks_len as u32)?;

    let mut data_pos =
      (Self::HEADER_SIZE + XcursorTocEntry::ENTRY_SIZE * chunks_len) as u32;

    for chunk in &self.chunks {
      let entry = match chunk {
        XcursorChunk::Comment { kind, .. } => XcursorTocEntry::Comment {
          kind: *kind,
          offset: data_pos,
        },
        XcursorChunk::Image { nominal, .. } => XcursorTocEntry::Image {
          nominal: *nominal,
          offset: data_pos,
        },
      };

      entry.write(writer)?;
      data_pos += chunk.exact_size() as u32;
    }

    for chunk in self.chunks {
      chunk.write(writer)?;
    }

    Ok(data_pos as usize)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    let f = |acc: usize, chunk: &XcursorChunk| {
      acc + XcursorTocEntry::ENTRY_SIZE + chunk.exact_size()
    };

    self.chunks.iter().fold(Self::HEADER_SIZE, f)
  }
}

// TODO: tests
