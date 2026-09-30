use crate_config::{CursorConfig, CursorTargets, CursorVariant};

use crate_formats::ani::AniFile;
use crate_formats::cur::CurFile;
use crate_formats::xcur::{XcursorChunk, XcursorFile};

use crate_pixmap::{IntoPixmap, RgbAlphaPixmap};
use crate_pixmap_png::{self, EncodeResult};

use crate::cursor::{CursorDuration, CursorFrame, CursorIcon};
use crate::error::PrecursorResult;

#[derive(Debug, Clone)]
pub struct Cursor {
  frames: Vec<CursorFrame>,
  metadata: CursorMetadata,
}

impl Cursor {
  /// Returns a reference to the underlying [`CursorMetadata`].
  pub const fn metadata(&self) -> &CursorMetadata {
    &self.metadata
  }

  /// Returns `true` if there are multiple frames in the cursor.
  pub const fn is_animated(&self) -> bool {
    self.frames.len() != 1
  }

  /// Constructs a new [`AniFile`].
  ///
  /// # Errors
  ///
  /// Returns the same errors as [`crate_pixmap_png::encode`].
  ///
  /// # Panics
  ///
  /// Panics if any icon [hotspot] is out of bounds.
  ///
  /// [hotspot]: crate_point::Point
  pub fn to_windows_ani(&self) -> EncodeResult<AniFile> {
    let num_frames = self.frames.len();

    let mut frames = Vec::with_capacity(num_frames);
    let mut rates = Vec::with_capacity(num_frames);
    let mut sequence = Vec::with_capacity(num_frames);

    for index in 0..=self.frames.len() {
      let frame = self.frames.get(index).unwrap();

      frames.push(frame.to_cur()?);
      rates.push(frame.duration.unwrap().jiffies());
      sequence.push(index as u32);
    }

    Ok(AniFile::new(frames, rates, sequence))
  }

  /// Constructs a new [`CurFile`].
  ///
  /// # Errors
  ///
  /// Returns the same errors as [`crate_pixmap_png::encode`].
  ///
  /// # Panics
  ///
  /// Panics if any icon [hotspot] is out of bounds.
  ///
  /// [hotspot]: crate_point::Point
  pub fn to_windows_cur(&self) -> EncodeResult<CurFile> {
    self.frames.first().unwrap().to_cur()
  }

  /// Constructs a new [`XcursorFile`].
  ///
  /// # Panics
  ///
  /// Panics if there are no chunks, or if the number of chunks exceeds
  /// `u32::MAX`.
  pub fn to_xcursor(&self) -> XcursorFile {
    let num_chunks = self.frames.iter().fold(0, |acc, f| acc + f.icons.len());

    let mut chunks = Vec::with_capacity(num_chunks);

    for frame in &self.frames {
      let duration = frame
        .duration
        .and_then(|d| Some(d.milliseconds()))
        .unwrap_or_default();

      for icon in &frame.icons {
        let rgba: RgbAlphaPixmap = icon.pixmap.clone().into_pixmap();
        let mut pixels: Box<[u8]> = rgba.into_iter().collect();

        for chunk in pixels.as_chunks_mut::<4>().0 {
          chunk.swap(0, 2); // rgba -> bgra
        }

        chunks.push(XcursorChunk::Image {
          nominal: icon.nominal,
          width: icon.pixmap.width(),
          height: icon.pixmap.height(),
          hotspot: icon.hotspot,
          duration,
          pixels,
        });
      }
    }

    XcursorFile::new(chunks)
  }

  /// Constructs a new `Cursor` from a [`CursorConfig`].
  ///
  /// # Errors
  ///
  /// Returns the same errors as [`CursorIcon::from_config`].
  pub fn from_config(
    CursorConfig {
      name,
      targets,
      variant,
    }: CursorConfig,
  ) -> PrecursorResult<Self> {
    use CursorVariant::*;

    let frames = match variant {
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
          let icon_config = crate_config::CursorIcon {
            asset,
            nominal,
            hotspot,
          };

          let icon = CursorIcon::from_config(icon_config)?;

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

    Ok(Self {
      frames,
      metadata: CursorMetadata { name, targets },
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorMetadata {
  pub name: String,
  pub targets: Option<CursorTargets>,
}

impl CursorMetadata {
  pub const fn linux_name(&self) -> &str {
    if let Some(targets) = &self.targets
      && let Some(linux) = &targets.linux
      && let Some(name) = &linux.name
    {
      name.as_str()
    } else {
      self.name.as_str()
    }
  }

  pub const fn linux_aliases(&self) -> &[String] {
    if let Some(targets) = &self.targets
      && let Some(linux) = &targets.linux
      && let Some(aliases) = &linux.aliases
      && !aliases.is_empty()
    {
      aliases.as_slice()
    } else {
      &[]
    }
  }

  pub const fn windows_name(&self) -> &str {
    if let Some(targets) = &self.targets
      && let Some(windows) = &targets.windows
      && let Some(name) = &windows.name
    {
      name.as_str()
    } else {
      &self.name.as_str()
    }
  }
}
