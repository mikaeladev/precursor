use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::error::{ReadError, ReadResult};

pub(super) trait DirEntry {
  const RESOURCE_TYPE: u16;

  fn new(
    width: u8,
    height: u8,
    color_count: u8,
    planes_or_hotx: u16,
    depth_or_hoty: u16,
  ) -> Self;

  fn width(&self) -> u8;
  fn height(&self) -> u8;
  fn color_count(&self) -> u8;
  fn planes_or_hotx(&self) -> u16;
  fn depth_or_hoty(&self) -> u16;
}

pub(super) fn read_impl<D: DirEntry, R: BufRead + Seek>(
  reader: &mut R,
) -> ReadResult<Vec<(D, Box<[u8]>)>> {
  if reader.read_u16::<LittleEndian>()? != 0 {
    return Err(ReadError::HeaderMissingReserved);
  }

  if let res_type = reader.read_u16::<LittleEndian>()?
    && res_type != D::RESOURCE_TYPE
  {
    return Err(ReadError::InvalidResourceType(D::RESOURCE_TYPE, res_type));
  }

  let entries_count = reader.read_u16::<LittleEndian>()?;

  let mut entries = Vec::with_capacity(entries_count as usize);

  let mut i = 0;

  while {
    i += 1;
    i <= entries_count
  } {
    let width = reader.read_u8()?;
    let height = reader.read_u8()?;
    let color_count = reader.read_u8()?;

    if reader.read_u8()? != 0 {
      return Err(ReadError::EntryMissingReserved(i));
    }

    let planes_or_hotx = reader.read_u16::<LittleEndian>()?;
    let depth_or_hoty = reader.read_u16::<LittleEndian>()?;

    let entry =
      D::new(width, height, color_count, planes_or_hotx, depth_or_hoty);

    let data_size = reader.read_u32::<LittleEndian>()?;
    let data_pos = reader.read_u32::<LittleEndian>()?;

    reader.seek(SeekFrom::Start(data_pos as u64))?;

    let mut buffer = Vec::with_capacity(data_size as usize);
    reader.take(data_size as u64).read_to_end(&mut buffer)?;

    reader.seek_relative(-(data_pos as i64))?;

    entries.push((entry, buffer.into_boxed_slice()));
  }

  Ok(entries)
}

pub(super) fn write_impl<D: DirEntry, W: Write>(
  writer: &mut W,
  entries: Vec<(D, Box<[u8]>)>,
) -> io::Result<usize> {
  const RESERVED: u8 = 0;

  let entries_count = entries.len() as u16;

  writer.write_u16::<LittleEndian>(RESERVED as u16)?;
  writer.write_u16::<LittleEndian>(D::RESOURCE_TYPE)?;
  writer.write_u16::<LittleEndian>(entries_count)?;

  let mut data_len = 6;
  let mut data_pos = 6 + 16 * entries_count as u32;

  for (entry, buffer) in &entries {
    writer.write_u8(entry.width())?;
    writer.write_u8(entry.height())?;

    writer.write_u8(entry.color_count())?;

    writer.write_u8(RESERVED)?;

    writer.write_u16::<LittleEndian>(entry.planes_or_hotx())?;
    writer.write_u16::<LittleEndian>(entry.depth_or_hoty())?;

    let data_size = buffer.len();
    data_len += 16 + data_size;
    let data_size = data_size as u32;

    writer.write_u32::<LittleEndian>(data_size)?;
    writer.write_u32::<LittleEndian>(data_pos)?;

    data_pos += data_size;
  }

  for (_, buffer) in entries {
    writer.write_all(&buffer)?;
  }

  Ok(data_len)
}

pub(super) fn exact_size_impl<T>(entries: &[(T, Box<[u8]>)]) -> usize {
  entries
    .iter()
    .fold(6 + 16 * entries.len(), |acc, (_, buf)| acc + buf.len())
}

#[cfg(test)]
mod tests {
  use std::io::{BufReader, Cursor};

  use super::*;

  #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
  struct TestDirEntry {
    width: u8,
    height: u8,
    color_count: u8,
    planes_or_hotx: u16,
    depth_or_hoty: u16,
  }

  impl DirEntry for TestDirEntry {
    const RESOURCE_TYPE: u16 = 0;

    fn new(
      width: u8,
      height: u8,
      color_count: u8,
      planes_or_hotx: u16,
      depth_or_hoty: u16,
    ) -> Self {
      Self {
        width,
        height,
        color_count,
        planes_or_hotx,
        depth_or_hoty,
      }
    }

    fn width(&self) -> u8 {
      self.width
    }
    fn height(&self) -> u8 {
      self.height
    }
    fn color_count(&self) -> u8 {
      self.color_count
    }
    fn planes_or_hotx(&self) -> u16 {
      self.planes_or_hotx
    }
    fn depth_or_hoty(&self) -> u16 {
      self.depth_or_hoty
    }
  }

  const EMPTY_IMAGES_BUF: &[u8] = &[
    00, 00, // reserved
    00, 00, // res_type
    00, 00, // entries_count
  ];

  #[test]
  fn read_empty_images() {
    let entries: Vec<(TestDirEntry, _)> = Vec::new();

    let mut reader = BufReader::new(Cursor::new(EMPTY_IMAGES_BUF));
    assert_eq!(read_impl(&mut reader).unwrap(), entries);
  }

  #[test]
  fn write_empty_images() {
    let entries: Vec<(TestDirEntry, _)> = Vec::new();

    let expected_size = EMPTY_IMAGES_BUF.len();
    assert_eq!(exact_size_impl(&entries), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(write_impl(&mut writer, entries).unwrap(), expected_size);
    assert_eq!(&writer, EMPTY_IMAGES_BUF)
  }

  #[rustfmt::skip]
  const EMPTY_SINGLE_IMAGE_BUF: &[u8] = &[
    00, 00, // reserved
    00, 00, // res_type
    01, 00, // entries_count

    00, // width
    00, // height
    00, // color_count
    00, // reserved
    00, 00, // color_planes
    00, 00, // bit_depth
    00, 00, 00, 00, // data_size
    22, 00, 00, 00, // data_offset
  ];

  #[test]
  fn read_empty_single_image() {
    let entries = vec![(TestDirEntry::default(), Box::new([]) as Box<[u8]>)];

    let mut reader = BufReader::new(Cursor::new(EMPTY_SINGLE_IMAGE_BUF));
    assert_eq!(read_impl(&mut reader).unwrap(), entries);
  }

  #[test]
  fn write_empty_single_image() {
    let entries = vec![(TestDirEntry::default(), Box::new([]) as Box<[u8]>)];

    let expected_size = EMPTY_SINGLE_IMAGE_BUF.len();
    assert_eq!(exact_size_impl(&entries), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(write_impl(&mut writer, entries).unwrap(), expected_size);
    assert_eq!(&writer, EMPTY_SINGLE_IMAGE_BUF);
  }

  #[rustfmt::skip]
  const SINGLE_IMAGE_BUFFER: &[u8] = &[
    00, 00, // reserved
    00, 00, // res_type
    01, 00, // entries_count

    00, // width
    00, // height
    00, // color_count
    00, // reserved
    00, 00, // color_planes
    00, 00, // bit_depth
    11, 00, 00, 00, // data_size
    22, 00, 00, 00, // data_offset

    0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20,
    0x77, 0x6F, 0x72, 0x6C, 0x64,
  ];

  #[test]
  fn read_single_image() {
    let entries = vec![(
      TestDirEntry::default(),
      Box::new(*b"hello world") as Box<[u8]>,
    )];

    let mut reader = BufReader::new(Cursor::new(SINGLE_IMAGE_BUFFER));
    assert_eq!(read_impl(&mut reader).unwrap(), entries);
  }

  #[test]
  fn write_single_image() {
    let entries = vec![(
      TestDirEntry::default(),
      Box::new(*b"hello world") as Box<[u8]>,
    )];

    let expected_size = SINGLE_IMAGE_BUFFER.len();
    assert_eq!(exact_size_impl(&entries), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(write_impl(&mut writer, entries).unwrap(), expected_size);
    assert_eq!(&writer, SINGLE_IMAGE_BUFFER)
  }
}
