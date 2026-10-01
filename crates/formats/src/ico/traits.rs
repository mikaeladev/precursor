use std::io::{self, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::file::{ReadError, ReadResult};

pub trait IcoDirEntry {
  const RESOURCE_TYPE: u16;

  /// Constructs a new `IcoDirEntry`.
  fn new(
    width: u8,
    height: u8,
    color_count: u8,
    planes_or_hotx: u16,
    depth_or_hoty: u16,
  ) -> Self;

  /// Returns the width.
  fn width(&self) -> u8;

  /// Returns the height.
  fn height(&self) -> u8;

  /// Returns the colour count in ICO form, or zero (`0`) in CUR form.
  fn colors_or_zero(&self) -> u8;

  /// Returns the colour planes in ICO form, or the x-coordinate of the hotspot
  /// in CUR form.
  fn planes_or_hotx(&self) -> u16;

  /// Returns the bit depth in ICO form, or the y-coordinate of the hotspot in
  /// CUR form.
  fn depth_or_hoty(&self) -> u16;
}

pub trait IcoDir<D: IcoDirEntry>
where
  Self: Sized,
{
  /// Constructs a new `IcoDir`.
  fn new(entries: Vec<(D, Box<[u8]>)>) -> Self;

  /// Returns a reference to the inner value.
  fn as_inner(&self) -> &[(D, Box<[u8]>)];

  /// Returns the inner value.
  fn into_inner(self) -> Vec<(D, Box<[u8]>)>;

  /// Reads an ICO file from `reader`, returning the constructed `IcoDir`.
  ///
  /// # Errors
  ///
  /// TODO
  fn read<R: Read + Seek>(reader: &mut R) -> ReadResult<Self> {
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

      let end_pos = reader.stream_position()?;

      reader.seek(SeekFrom::Start(data_pos as u64))?;

      let mut buffer = Vec::with_capacity(data_size as usize);
      reader.take(data_size as u64).read_to_end(&mut buffer)?;

      reader.seek(SeekFrom::Start(end_pos))?;

      entries.push((entry, buffer.into_boxed_slice()));
    }

    Ok(Self::new(entries))
  }

  /// Writes an ICO file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    const RESERVED: u8 = 0;

    let entries = self.into_inner();
    let entries_count = entries.len() as u16;

    writer.write_u16::<LittleEndian>(RESERVED as u16)?;
    writer.write_u16::<LittleEndian>(D::RESOURCE_TYPE)?;
    writer.write_u16::<LittleEndian>(entries_count)?;

    let mut data_len = 6;
    let mut data_pos = 6 + 16 * entries_count as u32;

    for (entry, buffer) in &entries {
      writer.write_u8(entry.width())?;
      writer.write_u8(entry.height())?;

      writer.write_u8(entry.colors_or_zero())?;

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

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  fn exact_size(&self) -> usize {
    let entries = self.as_inner();

    entries
      .iter()
      .fold(6 + 16 * entries.len(), |acc, (_, buf)| acc + buf.len())
  }
}

#[cfg(test)]
mod tests {
  use std::io::{BufReader, Cursor};

  use super::*;

  #[derive(Debug, PartialEq, Eq)]
  struct TestDir(Vec<(TestDirEntry, Box<[u8]>)>);

  impl IcoDir<TestDirEntry> for TestDir {
    fn new(entries: Vec<(TestDirEntry, Box<[u8]>)>) -> Self {
      Self(entries)
    }
    fn as_inner(&self) -> &[(TestDirEntry, Box<[u8]>)] {
      &self.0
    }
    fn into_inner(self) -> Vec<(TestDirEntry, Box<[u8]>)> {
      self.0
    }
  }

  #[derive(Debug, PartialEq, Eq)]
  struct TestDirEntry {
    width: u8,
    height: u8,
    color_count: u8,
    planes_or_hotx: u16,
    depth_or_hoty: u16,
  }

  impl IcoDirEntry for TestDirEntry {
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
    fn colors_or_zero(&self) -> u8 {
      self.color_count
    }
    fn planes_or_hotx(&self) -> u16 {
      self.planes_or_hotx
    }
    fn depth_or_hoty(&self) -> u16 {
      self.depth_or_hoty
    }
  }

  const DEFAULT_DIR_ENTRY: TestDirEntry = TestDirEntry {
    width: 0,
    height: 0,
    color_count: 0,
    planes_or_hotx: 0,
    depth_or_hoty: 0,
  };

  const EMPTY_IMAGES_BUF: &[u8] = &[
    00, 00, // reserved
    00, 00, // res_type
    00, 00, // entries_count
  ];

  fn empty_images_dir() -> TestDir {
    TestDir::new(vec![])
  }

  #[test]
  fn read_empty_images() {
    let mut reader = BufReader::new(Cursor::new(EMPTY_IMAGES_BUF));

    let value = TestDir::read(&mut reader).unwrap();
    let expected = empty_images_dir();

    assert_eq!(value, expected);
  }

  #[test]
  fn write_empty_images() {
    let dir = empty_images_dir();

    let expected_size = EMPTY_IMAGES_BUF.len();
    assert_eq!(dir.exact_size(), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(dir.write(&mut writer).unwrap(), expected_size);
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

  fn empty_single_image_dir() -> TestDir {
    TestDir::new(vec![(DEFAULT_DIR_ENTRY, Box::new([]))])
  }

  #[test]
  fn read_empty_single_image() {
    let mut reader = BufReader::new(Cursor::new(EMPTY_SINGLE_IMAGE_BUF));

    let value = TestDir::read(&mut reader).unwrap();
    let expected = empty_single_image_dir();

    assert_eq!(value, expected);
  }

  #[test]
  fn write_empty_single_image() {
    let dir = empty_single_image_dir();

    let expected_size = EMPTY_SINGLE_IMAGE_BUF.len();
    assert_eq!(dir.exact_size(), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(dir.write(&mut writer).unwrap(), expected_size);
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

  fn single_image_dir() -> TestDir {
    TestDir::new(vec![(DEFAULT_DIR_ENTRY, Box::new(*b"hello world"))])
  }

  #[test]
  fn read_single_image() {
    let mut reader = BufReader::new(Cursor::new(SINGLE_IMAGE_BUFFER));

    let value = TestDir::read(&mut reader).unwrap();
    let expected = single_image_dir();

    assert_eq!(value, expected);
  }

  #[test]
  fn write_single_image() {
    let dir = single_image_dir();

    let expected_size = SINGLE_IMAGE_BUFFER.len();
    assert_eq!(dir.exact_size(), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(dir.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer, SINGLE_IMAGE_BUFFER)
  }

  #[rustfmt::skip]
  const MULTI_IMAGES_BUF: &[u8] = &[
    00, 00, // reserved
    00, 00, // res_type
    02, 00, // entries_count

    00, // width
    00, // height
    00, // color_count
    00, // reserved
    00, 00, // color_planes
    00, 00, // bit_depth
    05, 00, 00, 00, // data_size
    38, 00, 00, 00, // data_offset

    00, // width
    00, // height
    00, // color_count
    00, // reserved
    00, 00, // color_planes
    00, 00, // bit_depth
    05, 00, 00, 00, // data_size
    43, 00, 00, 00, // data_offset

    0x68, 0x65, 0x6C, 0x6C, 0x6F, // "hello"

    0x77, 0x6F, 0x72, 0x6C, 0x64, // "world"
  ];

  fn multi_images_dir() -> TestDir {
    TestDir::new(vec![
      (DEFAULT_DIR_ENTRY, Box::new(*b"hello")),
      (DEFAULT_DIR_ENTRY, Box::new(*b"world")),
    ])
  }

  #[test]
  fn read_multi_images() {
    let mut reader = BufReader::new(Cursor::new(MULTI_IMAGES_BUF));

    let value = TestDir::read(&mut reader).unwrap();
    let expected = multi_images_dir();

    assert_eq!(value, expected);
  }

  #[test]
  fn write_multi_images() {
    let dir = multi_images_dir();

    let expected_size = MULTI_IMAGES_BUF.len();
    assert_eq!(dir.exact_size(), expected_size);

    let mut writer = Vec::with_capacity(expected_size);
    assert_eq!(dir.write(&mut writer).unwrap(), expected_size);
    assert_eq!(&writer, MULTI_IMAGES_BUF)
  }
}
