use crate_config::{CursorConfig, CursorIconConfig, CursorSubconfig};

use crate::cursor::{CursorDuration, CursorFrame, CursorIcon};
use crate::error::PrecursorResult;

#[derive(Debug, Clone)]
pub struct Cursor {
  pub frames: Vec<CursorFrame>,
  pub metadata: Option<CursorMetadata>,
}

impl Cursor {
  // TODO: document
  pub const fn is_animated(&self) -> bool {
    self.frames.len() != 1
  }

  // Attempts to
  pub fn from_config(
    CursorConfig { subconfig, .. }: CursorConfig,
  ) -> PrecursorResult<Self> {
    use CursorSubconfig::*;

    let frames = match subconfig {
      ScaledStatic { icon: icon_config } => {
        let icon = CursorIcon::from_config(icon_config)?;

        vec![CursorFrame {
          icons: vec![icon],
          duration: None,
        }]
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

          frames.push(CursorFrame {
            icons: vec![icon],
            duration: Some(CursorDuration::new(duration)),
          });
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

        vec![CursorFrame {
          icons,
          duration: None,
        }]
      }
      VerboseAnimated { sequence } => {
        let mut frames = Vec::with_capacity(sequence.len());

        for frame_config in sequence {
          let mut icons = Vec::with_capacity(frame_config.icons.len());

          for icon_config in frame_config.icons {
            icons.push(CursorIcon::from_config(icon_config)?);
          }

          frames.push(CursorFrame {
            icons,
            duration: Some(CursorDuration::new(frame_config.duration)),
          });
        }

        frames
      }
    };

    Ok(Cursor {
      frames,
      metadata: None,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorMetadata {
  // TODO
}
