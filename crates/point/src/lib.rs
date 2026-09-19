use std::ops::{
  Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> From<(T, T)> for Point<T> {
  fn from((x, y): (T, T)) -> Self {
    Point { x, y }
  }
}

impl<T: Add<Output = T>> Add for Point<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl<T: Copy + Add<Output = T>> Add<T> for Point<T> {
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self {
      x: self.x + rhs,
      y: self.y + rhs,
    }
  }
}

impl<T: AddAssign> AddAssign for Point<T> {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl<T: Copy + AddAssign> AddAssign<T> for Point<T> {
  fn add_assign(&mut self, rhs: T) {
    self.x += rhs;
    self.y += rhs;
  }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Point<T> {
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self {
      x: self.x - rhs,
      y: self.y - rhs,
    }
  }
}

impl<T: SubAssign> SubAssign for Point<T> {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

impl<T: Copy + SubAssign> SubAssign<T> for Point<T> {
  fn sub_assign(&mut self, rhs: T) {
    self.x -= rhs;
    self.y -= rhs;
  }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    }
  }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Point<T> {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl<T: MulAssign> MulAssign for Point<T> {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
  }
}

impl<T: Copy + MulAssign> MulAssign<T> for Point<T> {
  fn mul_assign(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
  }
}

impl<T: Div<Output = T>> Div for Point<T> {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
    }
  }
}

impl<T: Copy + Div<Output = T>> Div<T> for Point<T> {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
    }
  }
}

impl<T: DivAssign> DivAssign for Point<T> {
  fn div_assign(&mut self, rhs: Self) {
    self.x /= rhs.x;
    self.y /= rhs.y;
  }
}

impl<T: Copy + DivAssign> DivAssign<T> for Point<T> {
  fn div_assign(&mut self, rhs: T) {
    self.x /= rhs;
    self.y /= rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const INITIAL: Point<u8> = Point { x: 2, y: 4 };

  #[test]
  fn from_tuple() {
    assert_eq!(Point::from((2, 4)), INITIAL)
  }

  const EXPECTED_ADD: Point<u8> = Point { x: 3, y: 5 };

  #[test]
  fn add_self() {
    assert_eq!(INITIAL + Point { x: 1, y: 1 }, EXPECTED_ADD)
  }

  #[test]
  fn add_generic() {
    assert_eq!(INITIAL + 1, EXPECTED_ADD)
  }

  #[test]
  fn add_assign_self() {
    let mut point = INITIAL;
    point += Point { x: 1, y: 1 };
    assert_eq!(point, EXPECTED_ADD)
  }

  #[test]
  fn add_assign_generic() {
    let mut point = INITIAL;
    point += 1;
    assert_eq!(point, EXPECTED_ADD)
  }

  const EXPECTED_SUB: Point<u8> = Point { x: 1, y: 3 };

  #[test]
  fn sub_self() {
    assert_eq!(INITIAL - Point { x: 1, y: 1 }, EXPECTED_SUB)
  }

  #[test]
  fn sub_generic() {
    assert_eq!(INITIAL - 1, EXPECTED_SUB)
  }

  #[test]
  fn sub_assign_self() {
    let mut point = INITIAL;
    point -= Point { x: 1, y: 1 };
    assert_eq!(point, EXPECTED_SUB)
  }

  #[test]
  fn sub_assign_generic() {
    let mut point = INITIAL;
    point -= 1;
    assert_eq!(point, EXPECTED_SUB)
  }

  const EXPECTED_MUL: Point<u8> = Point { x: 4, y: 8 };

  #[test]
  fn mul_self() {
    assert_eq!(INITIAL * Point { x: 2, y: 2 }, EXPECTED_MUL)
  }

  #[test]
  fn mul_generic() {
    assert_eq!(INITIAL * 2, EXPECTED_MUL)
  }

  #[test]
  fn mul_assign_self() {
    let mut point = INITIAL;
    point *= Point { x: 2, y: 2 };
    assert_eq!(point, EXPECTED_MUL)
  }

  #[test]
  fn mul_assign_generic() {
    let mut point = INITIAL;
    point *= 2;
    assert_eq!(point, EXPECTED_MUL)
  }

  const EXPECTED_DIV: Point<u8> = Point { x: 1, y: 2 };

  #[test]
  fn div_self() {
    assert_eq!(INITIAL / Point { x: 2, y: 2 }, EXPECTED_DIV)
  }

  #[test]
  fn div_generic() {
    assert_eq!(INITIAL / 2, EXPECTED_DIV)
  }

  #[test]
  fn div_assign_self() {
    let mut point = INITIAL;
    point /= Point { x: 2, y: 2 };
    assert_eq!(point, EXPECTED_DIV)
  }

  #[test]
  fn div_assign_generic() {
    let mut point = INITIAL;
    point /= 2;
    assert_eq!(point, EXPECTED_DIV)
  }
}
