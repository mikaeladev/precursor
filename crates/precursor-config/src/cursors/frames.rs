use std::fmt::{self, Formatter};
use std::num::NonZero;

use serde::de::{self, MapAccess, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

use crate::cursors::AssetIndexes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FramesConfig {
  Static(AssetIndexes),
  Animated(Vec<AnimatedFrameConfig>),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct AnimatedFrameConfig {
  pub duration: NonZero<u32>,
  pub assets: AssetIndexes,
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
          if matches!(value, Some(FramesConfig::Static(_))) {
            return Err(de::Error::custom(
              "static cursors only contain a single frame",
            ));
          }

          match frame {
            Frame::Static(_) if value.is_some() => {
              return Err(de::Error::invalid_value(
                Unexpected::Other("static frame"),
                &"animation frame",
              ));
            }
            Frame::Static(indexes) => {
              value = Some(FramesConfig::Static(indexes))
            }
            Frame::Animated { duration, assets } => {
              let frame = AnimatedFrameConfig { duration, assets };

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

enum Frame {
  Static(AssetIndexes),
  Animated {
    duration: NonZero<u32>,
    assets: AssetIndexes,
  },
}

impl<'de> Deserialize<'de> for Frame {
  fn deserialize<D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<Self, D::Error> {
    struct FrameVisitor;

    impl<'de> Visitor<'de> for FrameVisitor {
      type Value = Frame;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "a frame struct")
      }

      fn visit_map<A: MapAccess<'de>>(
        self,
        mut map: A,
      ) -> Result<Self::Value, A::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Duration {
          Infinity(Infinity),
          Integer(NonZero<u32>),
        }

        let mut duration: Option<Duration> = None;
        let mut assets: Option<AssetIndexes> = None;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
          Duration,
          Assets,
        }

        while let Some(key) = map.next_key()? {
          match key {
            Field::Duration => {
              if duration.is_some() {
                return Err(de::Error::duplicate_field("duration"));
              }
              duration = Some(map.next_value()?)
            }
            Field::Assets => {
              if assets.is_some() {
                return Err(de::Error::duplicate_field("assets"));
              }
              assets = Some(map.next_value()?)
            }
          }
        }

        let duration = duration.ok_or(de::Error::missing_field("duration"))?;
        let assets = assets.ok_or(de::Error::missing_field("assets"))?;

        Ok(match duration {
          Duration::Infinity(_) => Frame::Static(assets),
          Duration::Integer(duration) => Frame::Animated { duration, assets },
        })
      }
    }

    deserializer.deserialize_map(FrameVisitor)
  }
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
