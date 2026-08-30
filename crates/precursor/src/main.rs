mod args;
mod asset;
mod config;
mod cursor;
mod error;

use std::env;
use std::fs::{self, File};
use std::io::ErrorKind as IoErrorKind;

use crate_config::{CursorConfig, CursorIconConfig, CursorSubconfig};
use crate_formats::write::WriteTo;
use crate_formats::{AniFile, CurFile, XcursorFile};

use crate::args::Command;
use crate::cursor::{Cursor, CursorDuration, CursorFrame, FromCursor};
use crate::error::PrecursorResult;

fn main() -> PrecursorResult {
  let args = args::parse();

  match args.command {
    Command::Build {
      input,
      target_dir,
      scalable,
      windows,
      xcursor,
      all,
    } => {
      let target_dir = if let Some(value) = target_dir {
        if !fs::metadata(&value)?.is_dir() {
          Err(IoErrorKind::NotADirectory.into())
        } else {
          Ok(value)
        }
      } else {
        env::current_dir()
      }?;

      let config = config::read(input.open()?)?;

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
    }

    Command::Check { input } => {
      let config = config::read(input.open()?);

      match config {
        Ok(_) => println!("Success!"),
        Err(err) => eprintln!("Error parsing config: {err}"),
      }
    }

    Command::Extract {
      input: _,
      frames: _,
    } => {
      todo!()
    }

    Command::Inspect { input: _ } => {
      todo!()
    }
  }

  Ok(())
}

fn cursor_from_config(cursor_config: CursorConfig) -> PrecursorResult<Cursor> {
  use CursorSubconfig::*;

  Ok(match cursor_config.subconfig {
    ScaledStatic {
      icon:
        CursorIconConfig {
          asset,
          nominal,
          hotspot,
        },
    } => {
      let icon = asset::icon(nominal, hotspot, asset)?;

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
        let icon = asset::icon(nominal, hotspot, asset)?;

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
