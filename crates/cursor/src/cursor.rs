use crate_formats::RasterImage;

use crate::{CursorDuration, CursorHotspot};

pub struct Cursor {
  pub frames: Vec<CursorFrame>,
  pub metadata: Option<CursorMetadata>,
}

impl Cursor {
  pub const fn is_animated(&self) -> bool {
    self.frames.len() != 1
  }
}

#[derive(Clone)]
pub struct CursorFrame {
  pub images: Vec<CursorImage>,
  pub duration: Option<CursorDuration>,
}

#[derive(Clone)]
pub struct CursorImage {
  pub nominal: u32,
  pub hotspot: CursorHotspot,
  pub raster: RasterImage,
}

#[derive(Clone)]
pub struct CursorMetadata {
  // TODO
}
