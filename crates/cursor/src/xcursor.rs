use std::io::Write;

use crate_formats::raster::{IntoPixmap, Pixmap};
use crate_formats::write::{WriteResult, WriteTo};
use crate_formats::{XcursorFile, XcursorImageChunk};

use crate::Cursor;

pub struct X11Cursor<'c>(pub &'c Cursor);

impl WriteTo for X11Cursor<'_> {
  fn write_to<W: Write>(self, writer: W) -> WriteResult {
    let Self(cursor) = self;

    let num_chunks =
      cursor.frames.iter().fold(0, |acc, f| acc + f.images.len());

    let mut chunks = Vec::with_capacity(num_chunks);

    for frame in &cursor.frames {
      let duration = frame.duration.and_then(|d| Some(d.milliseconds()));

      for image in &frame.images {
        let bgra = image
          .pixmap
          .clone()
          .into_rgb_alpha()
          .pixels()
          .into_iter()
          .flat_map(|p| [p.b, p.g, p.r, p.a])
          .collect();

        chunks.push(XcursorImageChunk::new(
          image.nominal,
          image.pixmap.width(),
          image.pixmap.height(),
          image.hotspot.x,
          image.hotspot.y,
          duration,
          bgra,
        ));
      }
    }

    XcursorFile::new(chunks).write_to(writer)
  }
}
