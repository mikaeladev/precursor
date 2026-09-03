use std::convert::Infallible;

use crate_formats::ani::AniFile;
use crate_formats::cur::{CurFile, CurIcon};
use crate_formats::rasters::{RasterError, RasterResult};
use crate_formats::xcursor::{XcursorFile, XcursorImageChunk};

use crate_pixmap::IntoPixmap;

use super::{Cursor, CursorFrame};

pub trait FromCursor: Sized {
  /// The associated error that can be returned in the process.
  type Error;

  /// Attempts to create a new value from a `Cursor` reference.
  fn from_cursor(cursor: &Cursor) -> Result<Self, Self::Error>;
}

impl FromCursor for XcursorFile<'_> {
  type Error = Infallible;

  /// Creates a new `XcursorFile` from a `Cursor` reference.
  ///
  /// # Panics
  ///
  /// Panics if any icon hotspot is out of bounds.
  fn from_cursor(cursor: &Cursor) -> Result<Self, Infallible> {
    let num_chunks = cursor.frames.iter().fold(0, |acc, f| acc + f.icons.len());

    let mut chunks = Vec::with_capacity(num_chunks);

    for frame in &cursor.frames {
      let duration = frame.duration.and_then(|d| Some(d.milliseconds()));

      for icon in &frame.icons {
        chunks.push(XcursorImageChunk::new(
          icon.nominal,
          icon.hotspot,
          icon.pixmap.clone().into_pixmap(),
          duration,
        ));
      }
    }

    Ok(XcursorFile::new(chunks))
  }
}

/// Attempts to create a new `CurFile` from a `Cursor` reference.
///
/// Fails with a `RasterError` if the pixmap is malformed.
///
/// # Panics
///
/// Panics if any icon hotspot is out of bounds.
impl FromCursor for CurFile {
  type Error = RasterError;

  fn from_cursor(cursor: &Cursor) -> Result<Self, RasterError> {
    let frame = cursor.frames.first().unwrap();

    Ok(frame_to_cur(frame)?)
  }
}

impl FromCursor for AniFile {
  type Error = RasterError;

  /// Attempts to create a new `AniFile` from a `Cursor` reference.
  ///
  /// Fails with a `RasterError` if the pixmap is malformed.
  ///
  /// # Panics
  ///
  /// Panics if any icon hotspot is out of bounds.
  fn from_cursor(cursor: &Cursor) -> Result<Self, RasterError> {
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

/// Creates a new `CurFile` from a `CursorFrame` reference.
///
/// # Panics
///
/// Panics if any icon hotspot is out of bounds.
fn frame_to_cur(frame: &CursorFrame) -> RasterResult<CurFile> {
  let icons = frame
    .icons
    .iter()
    .map(|icon| CurIcon::new(icon.hotspot, icon.pixmap.clone()))
    .collect();

  CurFile::new(icons)
}
