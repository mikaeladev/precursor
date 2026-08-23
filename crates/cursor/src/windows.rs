use std::io::Write;

use crate_formats::raster::{PngImage, RasterError};
use crate_formats::write::{WriteResult, WriteTo};
use crate_formats::{AniFile, CurFile, IconColorCount, IconDirEntry};

use crate::{Cursor, CursorFrame};

pub struct WindowsCursor<'c>(pub &'c Cursor);

impl WindowsCursor<'_> {
  /// Writes the formatted data to `writer`.
  pub fn write_to<W: Write>(self, writer: W) -> WriteResult {
    let Self(cursor) = self;

    if cursor.frames.len() == 1 {
      let frame = cursor.frames.first().unwrap();

      CurFile::try_from(frame)?.write_to(writer)?;
    } else {
      let num_frames = cursor.frames.len();

      let mut frames = Vec::with_capacity(num_frames);
      let mut rates = Vec::with_capacity(num_frames);
      let mut sequence = Vec::with_capacity(num_frames);

      for index in 0..=cursor.frames.len() {
        let frame = cursor.frames.get(index).unwrap();

        frames.push(frame.try_into()?);
        rates.push(frame.duration.unwrap().jiffies());
        sequence.push(index as u32);
      }

      AniFile::new(frames, rates, sequence).write_to(writer)?;
    }

    Ok(())
  }
}

impl<'f> TryFrom<&'f CursorFrame> for CurFile {
  type Error = RasterError;

  fn try_from(value: &'f CursorFrame) -> Result<Self, Self::Error> {
    let num_images = value.images.len();

    let mut entries = Vec::with_capacity(num_images);
    let mut images = Vec::with_capacity(num_images);

    for image in value.images.iter() {
      let png = image.pixmap.encode_png()?;

      entries.push(IconDirEntry::new(
        image.nominal as u16,
        image.nominal as u16,
        IconColorCount::EightPlus,
        image.hotspot.x as u16,
        image.hotspot.y as u16,
        png.len() as u32,
      ));

      images.push(png);
    }

    Ok(CurFile::new(entries, images))
  }
}
