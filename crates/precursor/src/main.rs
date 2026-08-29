mod args;
mod asset;
mod error;

use std::env::current_dir;
use std::fs::{File, metadata};
use std::io::{BufReader, ErrorKind as IoErrorKind, read_to_string};
use std::path::PathBuf;

use crate_config::{Config, CursorConfig, CursorIconConfig, CursorSubconfig};
use crate_cursor::{Cursor, CursorDuration, CursorFrame, FromCursor};
use crate_formats::write::WriteTo;
use crate_formats::{AniFile, CurFile, XcursorFile};

use clap::Parser;

use crate::args::{Cli, Command};
use crate::asset::asset_to_icon;
use crate::error::{IoError, PrecursorResult};

fn main() -> PrecursorResult {
  let args = Cli::parse();

  match args.command {
    Command::Build {
      input,
      target_directory,
      scalable,
      windows,
      xcursor,
      all,
    } => {
      let target_directory = get_target_directory(target_directory)?;

      let config = read_config(input.open()?)?;

      for cursor_config in config.cursors {
        let cursor_path = target_directory.join(&cursor_config.name);
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
      let config = read_config(input.open()?);

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

fn get_target_directory(
  target_directory: Option<PathBuf>,
) -> Result<PathBuf, IoError> {
  if let Some(value) = target_directory {
    if !metadata(&value)?.is_dir() {
      Err(IoErrorKind::NotADirectory.into())
    } else {
      Ok(value)
    }
  } else {
    current_dir()
  }
}

fn read_config(reader: BufReader<File>) -> PrecursorResult<Config> {
  let config_str = read_to_string(reader)?;
  let config = toml::from_str::<Config>(&config_str)?;

  Ok(config)
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
      let icon = asset_to_icon(nominal, hotspot, asset)?;

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
        let icon = asset_to_icon(nominal, hotspot, asset)?;

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
