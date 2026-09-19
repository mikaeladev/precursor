use std::convert::Infallible;

use crate_formats::cursors::ani::AniFile;
use crate_formats::cursors::cur::{CurFile, CurIcon};
use crate_formats::cursors::xcursor::{XcursorChunk, XcursorFile};
use crate_formats::rasters::png::PngImage;
use crate_formats::rasters::{RasterError, RasterResult};
use crate_pixmap::{IntoPixmap, RgbAlphaPixmap};
use crate_point::Point;

use super::{Cursor, CursorFrame};

pub trait FromCursor: Sized {
  /// The associated error that can be returned in the process.
  type Error;

  /// Attempts to create a new value from a [`Cursor`] reference.
  fn from_cursor(cursor: &Cursor) -> Result<Self, Self::Error>;
}

impl FromCursor for XcursorFile {
  type Error = Infallible;

  /// Constructs a new [`XcursorFile`] from a [`Cursor`] reference.
  ///
  /// # Errors
  ///
  /// This function is infallible. Yay!
  ///
  /// # Panics
  ///
  /// Panics if there are no chunks, or if the number of chunks exceeds
  /// `u32::MAX`.
  fn from_cursor(cursor: &Cursor) -> Result<Self, Infallible> {
    let num_chunks = cursor.frames.iter().fold(0, |acc, f| acc + f.icons.len());

    let mut chunks = Vec::with_capacity(num_chunks);

    for frame in &cursor.frames {
      let duration = frame
        .duration
        .and_then(|d| Some(d.milliseconds()))
        .unwrap_or_default();

      for icon in &frame.icons {
        let rgba: RgbAlphaPixmap = icon.pixmap.clone().into_pixmap();
        let mut pixels: Box<[u8]> = rgba.into_iter().collect();

        for chunk in pixels.as_chunks_mut::<4>().0 {
          chunk.swap(0, 2); // rgba -> bgra
        }

        chunks.push(XcursorChunk::Image {
          nominal: icon.nominal,
          width: icon.pixmap.width(),
          height: icon.pixmap.height(),
          hotspot: icon.hotspot,
          duration,
          pixels,
        });
      }
    }

    Ok(XcursorFile::new(chunks))
  }
}

impl FromCursor for CurFile {
  type Error = RasterError;

  /// Attempts to construct a new [`CurFile`] from a [`Cursor`] reference.
  ///
  /// # Errors
  ///
  /// Fails with a [`RasterError`] if any pixmap is malformed.
  ///
  /// # Panics
  ///
  /// Panics if any icon [hotspot] is out of bounds.
  ///
  /// [hotspot]: Point
  fn from_cursor(cursor: &Cursor) -> Result<Self, RasterError> {
    frame_to_cur(cursor.frames.first().unwrap())
  }
}

impl FromCursor for AniFile {
  type Error = RasterError;

  /// Attempts to construct a new [`AniFile`] from a [`Cursor`] reference.
  ///
  /// # Errors
  ///
  /// Fails with a [`RasterError`] if any pixmap is malformed.
  ///
  /// # Panics
  ///
  /// Panics if any icon [hotspot] is out of bounds.
  ///
  /// [hotspot]: Point
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

    Ok(AniFile::new(frames, rates, sequence)?)
  }
}

/// Attempts to construct a new [`CurFile`] from a [`CursorFrame`] reference.
///
/// # Errors
///
/// Fails with a [`RasterError`] if any pixmap is malformed.
///
/// # Panics
///
/// Panics if any icon [hotspot] is out of bounds.
///
/// [hotspot]: Point
fn frame_to_cur(frame: &CursorFrame) -> RasterResult<CurFile> {
  let mut icons = Vec::with_capacity(frame.icons.len());

  for icon in &frame.icons {
    icons.push(CurIcon::new(
      icon.pixmap.width() as u16,
      icon.pixmap.height() as u16,
      Point::from((icon.hotspot.x as u16, icon.hotspot.y as u16)),
      icon.pixmap.clone().encode_png()?.into_boxed_slice(),
    ));
  }

  Ok(CurFile::new(icons))
}
