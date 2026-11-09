use std::fmt::{self, Formatter};
use std::num::NonZero;

use serde::de::{self, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

use crate::cursors::AssetIndexes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FramesConfig {
  Static { bitmaps: AssetIndexes },
  Animated(Vec<AnimatedFrameConfig>),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct AnimatedFrameConfig {
  pub duration: NonZero<u32>,
  pub bitmaps: AssetIndexes,
}

impl<'de> Deserialize<'de> for FramesConfig {
  fn deserialize<D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<Self, D::Error> {
    struct FramesConfigVisitor;

    impl<'de> Visitor<'de> for FramesConfigVisitor {
      type Value = FramesConfig;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "a sequence of frames")
      }

      fn visit_seq<A: SeqAccess<'de>>(
        self,
        mut seq: A,
      ) -> Result<Self::Value, A::Error> {
        let mut value = None;

        while let Some(frame) = seq.next_element::<Frame>()? {
          if matches!(value, Some(FramesConfig::Static { .. })) {
            return Err(de::Error::custom(
              "static cursors only contain a single frame",
            ));
          }

          match frame {
            Frame::Static { .. } if value.is_some() => {
              return Err(de::Error::invalid_value(
                Unexpected::Other("static frame"),
                &"animation frame",
              ));
            }
            Frame::Static { bitmaps, .. } => {
              value = Some(FramesConfig::Static { bitmaps })
            }
            Frame::Animated { duration, bitmaps } => {
              let frame = AnimatedFrameConfig { duration, bitmaps };

              if let Some(FramesConfig::Animated(frames)) = &mut value {
                frames.push(frame);
              } else {
                let capacity = seq.size_hint().unwrap_or(2);
                let mut frames = Vec::with_capacity(capacity);

                frames.push(frame);
                value = Some(FramesConfig::Animated(frames))
              }
            }
          }
        }

        value.ok_or(de::Error::invalid_length(0, &"at least 1"))
      }
    }

    deserializer.deserialize_seq(FramesConfigVisitor)
  }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Frame {
  Static {
    #[allow(dead_code)]
    duration: Infinity,
    bitmaps: AssetIndexes,
  },
  Animated {
    duration: NonZero<u32>,
    bitmaps: AssetIndexes,
  },
}

#[derive(Debug, Clone, Copy)]
struct Infinity;

impl<'de> Deserialize<'de> for Infinity {
  fn deserialize<D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<Self, D::Error> {
    struct InfinityVisitor;

    impl<'de> Visitor<'de> for InfinityVisitor {
      type Value = Infinity;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "inf")
      }

      fn visit_f64<E: de::Error>(self, float: f64) -> Result<Self::Value, E> {
        match float {
          f64::INFINITY => Ok(Infinity),
          _ => Err(E::invalid_value(Unexpected::Float(float), &"inf")),
        }
      }
    }

    deserializer.deserialize_f64(InfinityVisitor)
  }
}
