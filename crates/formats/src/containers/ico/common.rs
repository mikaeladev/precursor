use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};

pub(super) trait IcoDirEntry {
  fn width(&self) -> u8;
  fn height(&self) -> u8;
  fn color_count(&self) -> u8;
  fn planes_or_hotx(&self) -> u16;
  fn depth_or_hoty(&self) -> u16;
}

pub(super) fn write_impl<T: IcoDirEntry, W: Write>(
  writer: &mut W,
  res_type: u16,
  entries: Vec<(T, Box<[u8]>)>,
) -> io::Result<usize> {
  const RESERVED: u8 = 0;

  let entries_count = entries.len() as u16;

  writer.write_u16::<LittleEndian>(RESERVED as u16)?;
  writer.write_u16::<LittleEndian>(res_type)?;
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
  use super::*;

  struct TestDirEntry;

  #[rustfmt::skip]
  impl IcoDirEntry for TestDirEntry {
    fn width(&self) -> u8 { 0 }
    fn height(&self) -> u8 { 0 }
    fn color_count(&self) -> u8 { 0 }
    fn planes_or_hotx(&self) -> u16 { 0 }
    fn depth_or_hoty(&self) -> u16 { 0 }
  }

  #[test]
  fn empty_images() {
    let entries: Vec<(TestDirEntry, _)> = Vec::new();

    let expected_buffer = &[
      00, 00, // reserved
      00, 00, // res_type
      00, 00, // entries_count
    ];

    let expected_size = expected_buffer.len();
    assert_eq!(exact_size_impl(&entries), expected_size);

    let mut buffer = Vec::with_capacity(expected_size);
    assert_eq!(write_impl(&mut buffer, 0, entries).unwrap(), expected_size);

    assert_eq!(&buffer, expected_buffer)
  }

  #[test]
  fn empty_image_buffer() {
    // NOTE: may error in future

    let entries = vec![(TestDirEntry, Box::new([]) as Box<[u8]>)];

    #[rustfmt::skip]
    let expected_buffer = &[
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

    let expected_size = expected_buffer.len();
    assert_eq!(exact_size_impl(&entries), expected_size);

    let mut buffer = Vec::with_capacity(expected_size);
    assert_eq!(write_impl(&mut buffer, 0, entries).unwrap(), expected_size);

    assert_eq!(&buffer, expected_buffer)
  }

  #[test]
  fn single_image() {
    let entries = vec![(TestDirEntry, Box::new(*b"hello world") as Box<[u8]>)];

    #[rustfmt::skip]
    let expected_buffer = &[
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

    let expected_size = expected_buffer.len();
    assert_eq!(exact_size_impl(&entries), expected_size);

    let mut buffer = Vec::with_capacity(expected_size);
    assert_eq!(write_impl(&mut buffer, 0, entries).unwrap(), expected_size);

    assert_eq!(&buffer, expected_buffer)
  }
}
