mod alpha;
mod opaque;

pub use alpha::*;
pub use opaque::*;

#[cfg(test)]
mod constants {
  use super::*;

  pub const RED_OPAQUE: RgbPixel = RgbPixel {
    r: 205,
    g: 92,
    b: 92,
  };

  pub const RED_ALPHA_SOLID: RgbAlphaPixel = RgbAlphaPixel {
    r: RED_OPAQUE.r,
    g: RED_OPAQUE.g,
    b: RED_OPAQUE.b,
    a: u8::MAX,
  };

  pub const RED_ALPHA_TRANS: RgbAlphaPixel = RgbAlphaPixel {
    r: RED_OPAQUE.r,
    g: RED_OPAQUE.g,
    b: RED_OPAQUE.b,
    a: u8::MIN,
  };

  pub const GREY_OPAQUE: RgbPixel = RgbPixel {
    r: 116,
    g: 116,
    b: 116,
  };

  pub const GREY_ALPHA_SOLID: RgbAlphaPixel = RgbAlphaPixel {
    r: GREY_OPAQUE.r,
    g: GREY_OPAQUE.g,
    b: GREY_OPAQUE.b,
    a: u8::MAX,
  };

  pub const GREY_ALPHA_TRANS: RgbAlphaPixel = RgbAlphaPixel {
    r: GREY_OPAQUE.r,
    g: GREY_OPAQUE.g,
    b: GREY_OPAQUE.b,
    a: u8::MIN,
  };
}

#[cfg(test)]
pub(crate) use constants::*;
