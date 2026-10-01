use crate_point::Point;

use crate::ico::traits::{IcoDir, IcoDirEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorDir(pub Vec<(CursorDirEntry, Box<[u8]>)>);

impl IcoDir<CursorDirEntry> for CursorDir {
  fn new(entries: Vec<(CursorDirEntry, Box<[u8]>)>) -> Self {
    Self(entries)
  }
  fn as_inner(&self) -> &[(CursorDirEntry, Box<[u8]>)] {
    &self.0
  }
  fn into_inner(self) -> Vec<(CursorDirEntry, Box<[u8]>)> {
    self.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorDirEntry {
  pub width: u8,
  pub height: u8,
  pub hotspot: Point<u16>,
}

impl IcoDirEntry for CursorDirEntry {
  const RESOURCE_TYPE: u16 = 2;

  fn new(
    width: u8,
    height: u8,
    _color_count: u8,
    planes_or_hotx: u16,
    depth_or_hoty: u16,
  ) -> Self {
    Self {
      width,
      height,
      hotspot: Point {
        x: planes_or_hotx,
        y: depth_or_hoty,
      },
    }
  }

  fn width(&self) -> u8 {
    self.width
  }
  fn height(&self) -> u8 {
    self.height
  }
  fn colors_or_zero(&self) -> u8 {
    0
  }
  fn planes_or_hotx(&self) -> u16 {
    self.hotspot.x
  }
  fn depth_or_hoty(&self) -> u16 {
    self.hotspot.y
  }
}
