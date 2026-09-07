use crate_config::{CursorConfig, CursorIconConfig, CursorSubconfig};

use crate::cursor::{CursorDuration, CursorFrame, CursorIcon};
use crate::error::PrecursorResult;

#[derive(Debug, Clone)]
pub struct Cursor {
  pub frames: Vec<CursorFrame>,
  pub metadata: Option<CursorMetadata>,
}

impl Cursor {
  /// Constructs a new [`Cursor`].
  ///
  /// # Panics
  ///
  /// Panics if `frames` is empty.
  pub const fn new(
    frames: Vec<CursorFrame>,
    metadata: Option<CursorMetadata>,
  ) -> Self {
    assert!(!frames.is_empty(), "frames should not be empty");

    Self { frames, metadata }
  }

  /// Returns `true` if there are multiple frames in the cursor.
  pub const fn is_animated(&self) -> bool {
    self.frames.len() != 1
  }

  /// Attempts to construct a new [`Cursor`] from a [`CursorConfig`].
  ///
  /// # Errors
  ///
  /// Fails with a [`PrecursorError`] if any asset fails to decode.
  pub fn from_config(
    CursorConfig { subconfig, .. }: CursorConfig,
  ) -> PrecursorResult<Self> {
    use CursorSubconfig::*;

    let frames = match subconfig {
      ScaledStatic { icon: icon_config } => {
        let icon = CursorIcon::from_config(icon_config)?;

        let mut icon_x2 = icon.clone();
        icon_x2.scale_up(2);

        let mut icon_x3 = icon.clone();
        icon_x3.scale_up(3);

        let mut icon_x4 = icon.clone();
        icon_x4.scale_up(4);

        vec![CursorFrame::new(
          vec![icon, icon_x2, icon_x3, icon_x4],
          None,
        )]
      }
      ScaledAnimated {
        nominal,
        hotspot,
        sequence,
      } => {
        let mut frames = Vec::with_capacity(sequence.len());

        for (asset, duration) in sequence {
          let icon = CursorIcon::from_config(CursorIconConfig {
            asset,
            nominal,
            hotspot,
          })?;

          let mut icon_x2 = icon.clone();
          icon_x2.scale_up(2);

          let mut icon_x3 = icon.clone();
          icon_x3.scale_up(3);

          let mut icon_x4 = icon.clone();
          icon_x4.scale_up(4);

          frames.push(CursorFrame::new(
            vec![icon, icon_x2, icon_x3, icon_x4],
            Some(CursorDuration::new(duration)),
          ));
        }

        frames
      }
      VerboseStatic {
        icons: icon_configs,
      } => {
        let mut icons = Vec::with_capacity(icon_configs.len());

        for icon_config in icon_configs {
          icons.push(CursorIcon::from_config(icon_config)?);
        }

        vec![CursorFrame::new(icons, None)]
      }
      VerboseAnimated { sequence } => {
        let mut frames = Vec::with_capacity(sequence.len());

        for frame_config in sequence {
          let mut icons = Vec::with_capacity(frame_config.icons.len());

          for icon_config in frame_config.icons {
            icons.push(CursorIcon::from_config(icon_config)?);
          }

          frames.push(CursorFrame::new(
            icons,
            Some(CursorDuration::new(frame_config.duration)),
          ));
        }

        frames
      }
    };

    Ok(Cursor::new(frames, None))
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorMetadata {
  // TODO
}
