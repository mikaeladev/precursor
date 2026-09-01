use crate_config::{CursorConfig, CursorIconConfig, CursorSubconfig};

use crate::error::PrecursorResult;

use super::{CursorDuration, CursorFrame, CursorIcon};

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

  // TODO: document
  pub fn from_config(
    CursorConfig { subconfig, .. }: CursorConfig,
  ) -> PrecursorResult<Self> {
    use CursorSubconfig::*;

    let frames = match subconfig {
      ScaledStatic { icon: icon_config } => {
        let icon = CursorIcon::from_config(icon_config)?;

        vec![CursorFrame {
          icons: icon.to_scaled_vec(),
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
            icons: icon.to_scaled_vec(),
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
          frames.push(CursorFrame::from_config(frame_config)?);
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
