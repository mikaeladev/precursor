mod alpha;
mod opaque;

pub use alpha::*;
pub use opaque::*;

pub(crate) fn rgb_to_y(r: u8, g: u8, b: u8) -> u8 {
  (r as f32 * 0.2126 + g as f32 * 0.7152 + b as f32 * 0.0722) as u8
}

#[cfg(test)]
mod constants {
  use super::*;

  pub const GREY_OPAQUE: LumaPixel = LumaPixel { y: 116 };

  pub const GREY_ALPHA_SOLID: LumaAlphaPixel = LumaAlphaPixel {
    y: GREY_OPAQUE.y,
    a: u8::MAX,
  };

  pub const GREY_ALPHA_TRANS: LumaAlphaPixel = LumaAlphaPixel {
    y: GREY_OPAQUE.y,
    a: 0,
  };
}

#[cfg(test)]
pub(crate) use constants::*;
