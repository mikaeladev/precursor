use std::convert::Infallible;

use crate_formats::{
  AniFile, CurFile, IconColorCount, IconDirEntry, IntoPixmap, Pixmap, PngImage,
  RasterError, XcursorFile, XcursorImageChunk,
};

use super::{Cursor, CursorFrame};

pub trait FromCursor: Sized {
  /// The associated error that can be returned in the process.
  type Error;

  /// Creates a new value from a `Cursor` reference.
  fn from_cursor(cursor: &Cursor) -> Result<Self, Self::Error>;
}

impl FromCursor for XcursorFile<'_> {
  type Error = Infallible;

  fn from_cursor(cursor: &Cursor) -> Result<Self, Self::Error> {
    let num_chunks = cursor.frames.iter().fold(0, |acc, f| acc + f.icons.len());

    let mut chunks = Vec::with_capacity(num_chunks);

    for frame in &cursor.frames {
      let duration = frame.duration.and_then(|d| Some(d.milliseconds()));

      for icon in &frame.icons {
        let bgra = icon
          .pixmap
          .clone()
          .into_rgb_alpha()
          .pixels()
          .into_iter()
          .flat_map(|p| [p.b, p.g, p.r, p.a])
          .collect();

        chunks.push(XcursorImageChunk::new(
          icon.nominal,
          icon.pixmap.width(),
          icon.pixmap.height(),
          icon.hotspot.x,
          icon.hotspot.y,
          duration,
          bgra,
        ));
      }
    }

    Ok(XcursorFile::new(chunks))
  }
}

impl FromCursor for CurFile {
  type Error = RasterError;

  fn from_cursor(cursor: &Cursor) -> Result<Self, Self::Error> {
    let frame = cursor.frames.first().unwrap();

    Ok(frame_to_cur(frame)?)
  }
}

impl FromCursor for AniFile {
  type Error = RasterError;

  fn from_cursor(cursor: &Cursor) -> Result<Self, Self::Error> {
    let num_frames = cursor.frames.len();

    let mut frames = Vec::with_capacity(num_frames);
    let mut rates = Vec::with_capacity(num_frames);
    let mut sequence = Vec::with_capacity(num_frames);

    for index in 0..=cursor.frames.len() {
      let frame = cursor.frames.get(index).unwrap();

      frames.push(frame_to_cur(frame)?);
      rates.push(frame.duration.unwrap().jiffies());
      sequence.push(index as u32);
    }

    Ok(AniFile::new(frames, rates, sequence))
  }
}

fn frame_to_cur(frame: &CursorFrame) -> Result<CurFile, RasterError> {
  let num_icons = frame.icons.len();

  let mut entries = Vec::with_capacity(num_icons);
  let mut icons = Vec::with_capacity(num_icons);

  for icon in frame.icons.iter() {
    let png = icon.pixmap.encode_png()?;

    entries.push(IconDirEntry::new(
      icon.nominal as u16,
      icon.nominal as u16,
      IconColorCount::EightPlus,
      icon.hotspot.x as u16,
      icon.hotspot.y as u16,
      png.len() as u32,
    ));

    icons.push(png);
  }

  Ok(CurFile::new(entries, icons))
}
