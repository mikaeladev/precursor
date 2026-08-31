use std::env;
use std::fs::{self, File};
use std::io::{ErrorKind as IoErrorKind, Result as IoResult};
use std::path::PathBuf;

use crate_config::{CursorConfig, CursorIconConfig, CursorSubconfig};
use crate_formats::write::WriteTo;
use crate_formats::{AniFile, CurFile, XcursorFile};

use crate::args::BuildArgs;
use crate::asset;
use crate::config;
use crate::cursor::{Cursor, CursorDuration, CursorFrame, FromCursor};
use crate::error::PrecursorResult;

pub fn build(
  BuildArgs {
    input,
    target_dir,
    scalable,
    windows,
    xcursor,
    all,
  }: BuildArgs,
) -> PrecursorResult {
  let config = config::read(input.open()?)?;
  let target_dir = get_target_dir(target_dir)?;

  for cursor_config in config.cursors {
    let cursor_path = target_dir.join(&cursor_config.name);
    let cursor = cursor_from_config(cursor_config)?;

    if all || scalable {
      // TODO
    }

    if all || windows {
      if cursor.is_animated() {
        AniFile::from_cursor(&cursor)?
          .write_to(File::create(cursor_path.with_extension("ani"))?)?;
      } else {
        CurFile::from_cursor(&cursor)?
          .write_to(File::create(cursor_path.with_extension("cur"))?)?;
      }
    }

    if all || xcursor {
      XcursorFile::from_cursor(&cursor)
        .unwrap() // infallible
        .write_to(File::create(cursor_path)?)?;
    }

    // TODO: name aliasing
  }

  Ok(())
}

/// Returns a path to the target directory.
///
/// Falls back to the current working directory if the value is `None`.
fn get_target_dir(target_dir: Option<PathBuf>) -> IoResult<PathBuf> {
  if let Some(value) = target_dir {
    if !fs::metadata(&value)?.is_dir() {
      Err(IoErrorKind::NotADirectory.into())
    } else {
      Ok(value)
    }
  } else {
    env::current_dir()
  }
}

fn cursor_from_config(
  CursorConfig { subconfig, .. }: CursorConfig,
) -> PrecursorResult<Cursor> {
  use CursorSubconfig::*;

  Ok(match subconfig {
    ScaledStatic {
      icon:
        CursorIconConfig {
          asset,
          nominal,
          hotspot,
        },
    } => {
      let icon = asset::icon_from_asset(nominal, hotspot, asset)?;

      // TODO: scale icon for various DPIs
      let icons = vec![icon];

      let frame = CursorFrame {
        icons,
        duration: None,
      };

      Cursor {
        frames: vec![frame],
        metadata: None,
      }
    }
    ScaledAnimated {
      nominal,
      hotspot,
      sequence,
    } => {
      let num_frames = sequence.len();

      let mut frames = Vec::with_capacity(num_frames);

      for (asset, duration) in sequence {
        let icon = asset::icon_from_asset(nominal, hotspot, asset)?;

        // TODO: scale icon for various DPIs
        let icons = vec![icon];
        let duration = Some(CursorDuration::new(duration));

        frames.push(CursorFrame { icons, duration });
      }

      Cursor {
        frames,
        metadata: None,
      }
    }
    _ => todo!(),
  })
}
