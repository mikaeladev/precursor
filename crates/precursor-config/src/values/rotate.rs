use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RotateValue {
  Ninety = 90,
  OneEighty = 180,
  TwoSeventy = 270,
}
