use super::traits::{IcoDir, IcoDirEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconDir(Vec<(IconDirEntry, Box<[u8]>)>);

impl IcoDir<IconDirEntry> for IconDir {
  fn new(images: Vec<(IconDirEntry, Box<[u8]>)>) -> Self {
    Self(images)
  }
  fn as_inner(&self) -> &[(IconDirEntry, Box<[u8]>)] {
    &self.0
  }
  fn into_inner(self) -> Vec<(IconDirEntry, Box<[u8]>)> {
    self.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconDirEntry {
  pub width: u8,
  pub height: u8,
  pub color_count: u8,
  pub color_planes: u16,
  pub bit_depth: u16,
}

impl IcoDirEntry for IconDirEntry {
  const RESOURCE_TYPE: u16 = 1;

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
      color_planes: planes_or_hotx,
      bit_depth: depth_or_hoty,
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
    self.color_planes
  }
  fn depth_or_hoty(&self) -> u16 {
    self.bit_depth
  }
}
