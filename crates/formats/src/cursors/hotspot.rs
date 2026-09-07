use std::ops::{Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hotspot {
  pub x: u32,
  pub y: u32,
}

impl From<(u32, u32)> for Hotspot {
  fn from((x, y): (u32, u32)) -> Self {
    Hotspot { x, y }
  }
}

impl Mul<u32> for Hotspot {
  type Output = Self;

  fn mul(self, rhs: u32) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl MulAssign<u32> for Hotspot {
  fn mul_assign(&mut self, rhs: u32) {
    self.x *= rhs;
    self.y *= rhs;
  }
}
