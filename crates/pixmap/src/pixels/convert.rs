use super::{LumaAlphaPixel, LumaPixel, Pixel, RgbAlphaPixel, RgbPixel};

pub trait FromPixel<P: Pixel> {
  /// Converts the pixel into this type.
  fn from_pixel(pixel: P) -> Self;
}

pub trait IntoPixel<P: Pixel> {
  /// Converts this value into a pixel.
  fn into_pixel(self) -> P;
}

impl<T: Pixel> FromPixel<T> for T {
  /// No-op.
  #[inline(always)]
  fn from_pixel(pixel: T) -> Self {
    pixel
  }
}

impl<T: Pixel, U: Pixel> IntoPixel<U> for T
where
  U: FromPixel<T>,
{
  /// Converts `T` into `U`.
  fn into_pixel(self) -> U {
    U::from_pixel(self)
  }
}

// -------------------------------------------------------------------------- //

impl FromPixel<LumaAlphaPixel> for LumaPixel {
  /// Converts a `LumaAlphaPixel` into a `LumaPixel`.
  fn from_pixel(pixel: LumaAlphaPixel) -> Self {
    Self { y: pixel.y }
  }
}

impl FromPixel<RgbPixel> for LumaPixel {
  /// Converts an `RgbPixel` into a `LumaPixel`.
  fn from_pixel(pixel: RgbPixel) -> Self {
    Self {
      y: (pixel.r as f32 * 0.2126
        + pixel.g as f32 * 0.7152
        + pixel.b as f32 * 0.0722) as u8,
    }
  }
}

impl FromPixel<RgbAlphaPixel> for LumaPixel {
  /// Converts an `RgbAlphaPixel` into a `LumaPixel`.
  fn from_pixel(pixel: RgbAlphaPixel) -> Self {
    RgbPixel::from_pixel(pixel).into_pixel()
  }
}

// -------------------------------------------------------------------------- //

impl FromPixel<LumaPixel> for LumaAlphaPixel {
  /// Converts a `LumaPixel` into a `LumaAlphaPixel`.
  fn from_pixel(pixel: LumaPixel) -> Self {
    Self {
      y: pixel.y,
      a: u8::MAX,
    }
  }
}

impl FromPixel<RgbPixel> for LumaAlphaPixel {
  /// Converts an `RgbPixel` into a `LumaAlphaPixel`.
  fn from_pixel(pixel: RgbPixel) -> Self {
    Self {
      y: LumaPixel::from_pixel(pixel).y,
      a: u8::MAX,
    }
  }
}

impl FromPixel<RgbAlphaPixel> for LumaAlphaPixel {
  /// Converts an `RgbAlphaPixel` into a  `LumaAlphaPixel`.
  fn from_pixel(pixel: RgbAlphaPixel) -> Self {
    Self {
      y: LumaPixel::from_pixel(pixel).y,
      a: pixel.a,
    }
  }
}

// -------------------------------------------------------------------------- //

impl FromPixel<LumaPixel> for RgbPixel {
  /// Converts a `LumaPixel` into an `RgbPixel`.
  fn from_pixel(pixel: LumaPixel) -> Self {
    Self {
      r: pixel.y,
      g: pixel.y,
      b: pixel.y,
    }
  }
}

impl FromPixel<LumaAlphaPixel> for RgbPixel {
  /// Converts a `LumaAlphaPixel` into an `RgbPixel`.
  fn from_pixel(pixel: LumaAlphaPixel) -> Self {
    LumaPixel::from_pixel(pixel).into_pixel()
  }
}

impl FromPixel<RgbAlphaPixel> for RgbPixel {
  /// Converts an `RgbAlphaPixel` into an `RgbPixel`.
  fn from_pixel(pixel: RgbAlphaPixel) -> Self {
    Self {
      r: pixel.r,
      g: pixel.g,
      b: pixel.b,
    }
  }
}

// -------------------------------------------------------------------------- //

impl FromPixel<LumaPixel> for RgbAlphaPixel {
  /// Converts a `LumaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(pixel: LumaPixel) -> Self {
    RgbPixel::from_pixel(pixel).into_pixel()
  }
}

impl FromPixel<LumaAlphaPixel> for RgbAlphaPixel {
  /// Converts a `LumaAlphaPixel` into an `RgbAlphaPixel`.
  fn from_pixel(pixel: LumaAlphaPixel) -> Self {
    Self {
      r: pixel.y,
      g: pixel.y,
      b: pixel.y,
      a: pixel.a,
    }
  }
}

impl FromPixel<RgbPixel> for RgbAlphaPixel {
  /// Converts an `RgbPixel` into an `RgbAlphaPixel`.
  fn from_pixel(pixel: RgbPixel) -> Self {
    Self {
      r: pixel.r,
      g: pixel.g,
      b: pixel.b,
      a: u8::MAX,
    }
  }
}

// -------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
  use super::*;

  const WHITE: RgbPixel = RgbPixel {
    r: 255,
    g: 255,
    b: 255,
  };

  const INDIAN_RED: RgbPixel = RgbPixel {
    r: 205,
    g: 92,
    b: 92,
  };

  #[test]
  fn luma_from_luma_alpha() {
    // alpha gets droppped
    assert_eq!(
      LumaPixel::from_pixel(LumaAlphaPixel { y: 255, a: 255 }),
      LumaPixel { y: 255 }
    );
  }

  #[test]
  fn luma_from_rgb() {
    // rgb -> greyscale
    assert_eq!(LumaPixel::from_pixel(INDIAN_RED), LumaPixel { y: 116 });
    assert_eq!(LumaPixel::from_pixel(WHITE), LumaPixel { y: 255 });
  }

  #[test]
  fn luma_from_rgb_alpha() {
    // rgb -> greyscale, alpha gets dropped
    assert_eq!(
      LumaPixel::from_pixel(RgbAlphaPixel {
        r: 205,
        g: 92,
        b: 92,
        a: 255,
      }),
      LumaPixel { y: 116 }
    );
  }

  #[test]
  fn luma_alpha_from_luma() {
    // alpha defaults to 255
    assert_eq!(
      LumaAlphaPixel::from_pixel(LumaPixel { y: 255 }),
      LumaAlphaPixel { y: 255, a: 255 }
    );
  }

  #[test]
  fn luma_alpha_from_rgb() {
    // rgb -> greyscale, alpha defaults to 255
    assert_eq!(
      LumaAlphaPixel::from_pixel(INDIAN_RED),
      LumaAlphaPixel { y: 116, a: 255 }
    );
  }

  #[test]
  fn luma_alpha_from_rgb_alpha() {
    // rgb -> greyscale, alpha gets inherited
    assert_eq!(
      LumaAlphaPixel::from_pixel(RgbAlphaPixel {
        r: 205,
        g: 92,
        b: 92,
        a: 123,
      }),
      LumaAlphaPixel { y: 116, a: 123 }
    );
  }

  #[test]
  fn rgb_from_luma() {
    // y gets propagated
    assert_eq!(RgbPixel::from_pixel(LumaPixel { y: 255 }), WHITE);
  }

  #[test]
  fn rgb_from_luma_alpha() {
    // alpha gets dropped
    assert_eq!(
      RgbPixel::from_pixel(LumaAlphaPixel { y: 255, a: 255 }),
      WHITE
    );
  }

  #[test]
  fn rgb_from_rgb_alpha() {
    // alpha gets dropped
    assert_eq!(
      RgbPixel::from_pixel(RgbAlphaPixel {
        r: 205,
        g: 92,
        b: 92,
        a: 255,
      }),
      INDIAN_RED
    );
  }

  #[test]
  fn rgb_alpha_from_luma() {
    // y gets propagated, alpha defaults to 255
    assert_eq!(
      RgbAlphaPixel::from_pixel(LumaPixel { y: 123 }),
      RgbAlphaPixel {
        r: 123,
        g: 123,
        b: 123,
        a: 255,
      }
    );
  }

  #[test]
  fn rgb_alpha_from_luma_alpha() {
    // y gets propagated, alpha gets inherited
    assert_eq!(
      RgbAlphaPixel::from_pixel(LumaAlphaPixel { y: 123, a: 123 }),
      RgbAlphaPixel {
        r: 123,
        g: 123,
        b: 123,
        a: 123,
      }
    );
  }

  #[test]
  fn rgb_alpha_from_rgb() {
    // rgb gets inherited, alpha defaults to 255
    assert_eq!(
      RgbAlphaPixel::from_pixel(INDIAN_RED),
      RgbAlphaPixel {
        r: 205,
        g: 92,
        b: 92,
        a: 255,
      }
    );
  }
}
