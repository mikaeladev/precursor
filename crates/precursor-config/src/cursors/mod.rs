mod assets;
mod frames;

use std::borrow::Cow;

use serde::Deserialize;

pub use assets::*;
pub use frames::*;

pub(crate) type Cursors = Vec<CursorConfig>;

#[derive(Debug, Deserialize)]
pub struct CursorConfig {
  pub name: String,
  pub aliases: Vec<String>,
  pub bitmaps: Vec<AssetConfig>,
  pub frames: Option<FramesConfig>,
}

impl CursorConfig {
  /// Returns the inner value of `frames` if [`Some`], and the indexes of
  /// `bitmaps` otherwise.
  pub fn get_frames(&self) -> Cow<'_, FramesConfig> {
    match self.frames.as_ref() {
      Some(frames) => Cow::Borrowed(frames),
      None => Cow::Owned(FramesConfig::Static(
        (0..self.bitmaps.len()).map(From::from).collect(),
      )),
    }
  }

  /// Returns a vector containing the indexes of `bitmaps`.
  fn get_bitmap_indexes(&self) -> Vec<usize> {
    (0..self.bitmaps.len()).collect()
  }

  /// Returns a vector containing the indexes of ophaned (unused) bitmaps.
  pub fn find_orphaned_bitmaps(&self) -> Vec<usize> {
    if let Some(frames) = &self.frames {
      let mut orphans = self.get_bitmap_indexes();

      let retain_fn =
        |i: usize, bitmaps: &AssetIndexes| !bitmaps.contains(&i.into());

      match frames {
        FramesConfig::Static(bitmaps) => {
          orphans.retain(|i| retain_fn(*i, bitmaps))
        }
        FramesConfig::Animated(frames) => {
          for frame in frames {
            orphans.retain(|i| retain_fn(*i, &frame.bitmaps));
          }
        }
      }

      orphans
    } else {
      // `get_frames` falls back to the indexes of `bitmaps`, leaving none unused
      Vec::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use std::num::NonZero;
  use std::path::PathBuf;

  use point::Point;

  use super::*;

  const DEFAULT_ASSET: AssetConfig = AssetConfig {
    path: PathBuf::new(),
    nominal: NonZero::new(1).unwrap(),
    hotspot: Point { x: 0, y: 0 },
    transforms: None,
  };

  #[test]
  fn get_frames() {
    let cursor = CursorConfig {
      name: String::new(),
      bitmaps: vec![DEFAULT_ASSET, DEFAULT_ASSET, DEFAULT_ASSET],
      frames: None,
      aliases: vec![],
    };

    assert_eq!(
      cursor.get_frames().as_ref(),
      &FramesConfig::Static(vec![0.into(), 1.into(), 2.into()])
    );
  }

  #[test]
  fn get_bitmap_indexes() {
    let cursor = CursorConfig {
      name: String::new(),
      bitmaps: vec![DEFAULT_ASSET, DEFAULT_ASSET, DEFAULT_ASSET],
      frames: None,
      aliases: vec![],
    };

    assert_eq!({ cursor }.get_bitmap_indexes(), vec![0, 1, 2]);
  }

  #[test]
  fn find_orphaned_bitmaps() {
    let empty_frames_cursor = CursorConfig {
      name: String::new(),
      bitmaps: vec![DEFAULT_ASSET, DEFAULT_ASSET, DEFAULT_ASSET],
      frames: None,
      aliases: vec![],
    };

    assert_eq!({ empty_frames_cursor }.find_orphaned_bitmaps(), vec![]);

    let static_frames_cursor = CursorConfig {
      name: String::new(),
      bitmaps: vec![DEFAULT_ASSET, DEFAULT_ASSET, DEFAULT_ASSET],
      frames: Some(FramesConfig::Static(vec![0.into(), 2.into()])),
      aliases: vec![],
    };

    assert_eq!({ static_frames_cursor }.find_orphaned_bitmaps(), vec![1]);

    let animation_frames_cursor = CursorConfig {
      name: String::new(),
      bitmaps: vec![
        DEFAULT_ASSET,
        DEFAULT_ASSET,
        DEFAULT_ASSET,
        DEFAULT_ASSET,
        DEFAULT_ASSET,
      ],
      frames: Some(FramesConfig::Animated(vec![
        AnimatedFrameConfig {
          duration: NonZero::new(200).unwrap(),
          bitmaps: vec![0.into(), 1.into()],
        },
        AnimatedFrameConfig {
          duration: NonZero::new(200).unwrap(),
          bitmaps: vec![2.into(), 3.into()],
        },
      ])),
      aliases: vec![],
    };

    assert_eq!({ animation_frames_cursor }.find_orphaned_bitmaps(), vec![4]);
  }
}
