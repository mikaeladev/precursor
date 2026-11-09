use std::num::NonZero;

#[derive(Debug, Clone)]
pub enum CursorFrames {
  Static(Vec<usize>),
  Animated(Vec<AnimationFrame>),
}

impl CursorFrames {
  /// Returns `true` if this is [`Static`][Self::Static].
  pub const fn is_static(&self) -> bool {
    matches!(self, Self::Static { .. })
  }

  /// Returns `true` if this is [`Animated`][Self::Animated].
  pub const fn is_animated(&self) -> bool {
    matches!(self, Self::Animated(_))
  }
}

#[derive(Debug, Clone)]
pub struct AnimationFrame {
  pub duration: NonZero<u32>,
  pub indexes: Vec<usize>,
}
