use std::io::{Result as IoResult, Write};

use crate_formats::{WriteTo, XcursorFile, XcursorImageChunk};

use crate::Cursor;

pub struct X11Cursor<'c>(pub &'c Cursor);

impl WriteTo for X11Cursor<'_> {
  fn write_to<W: Write>(self, writer: W) -> IoResult<()> {
    let Self(cursor) = self;

    let num_chunks =
      cursor.frames.iter().fold(0, |acc, f| acc + f.images.len());

    let mut chunks = Vec::with_capacity(num_chunks);

    for frame in &cursor.frames {
      let duration = frame.duration.and_then(|d| Some(d.milliseconds()));

      for image in &frame.images {
        let pixels = image.raster.to_bgra();

        chunks.push(XcursorImageChunk::new(
          image.nominal,
          image.raster.width(),
          image.raster.height(),
          image.hotspot.x,
          image.hotspot.y,
          duration,
          pixels,
        ));
      }
    }

    XcursorFile::new(chunks).write_to(writer)
  }
}
