mod args;
mod asset;
mod error;

use std::env::current_dir;
use std::fs::{File, metadata};
use std::io::{BufReader, ErrorKind as IoErrorKind, read_to_string};
use std::path::PathBuf;

use crate_config::*;
use crate_cursor::*;
use crate_formats::write::WriteTo;

use clap::Parser;

use crate::args::{Cli, Command};
use crate::asset::Asset;
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

      for (name, cursor_config) in config.cursors {
        let cursor_path = target_directory.join(&name);
        let cursor = cursor_from_config(cursor_config)?;

        if all || scalable {
          todo!()
        }

        if all || windows {
          let cursor_path = cursor_path
            .with_extension(if cursor.is_animated() { "ani" } else { "cur" });

          WindowsCursor(&cursor).write_to(File::create(cursor_path)?)?;
        }

        if all || xcursor {
          X11Cursor(&cursor).write_to(File::create(cursor_path)?)?;
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
  Ok(match cursor_config {
    CursorConfig::ScaledStatic(value) => {
      let ScaledStaticCursorConfig {
        nominal,
        hotspot,
        asset: asset_config,
        aliases: _,
      } = value;

      let image =
        Asset::from_config(nominal, hotspot, &asset_config)?.into_image();

      // TODO: scale image for various DPIs
      let images = vec![image];

      let frame = CursorFrame {
        images,
        duration: None,
      };

      Cursor {
        frames: vec![frame],
        metadata: None,
      }
    }
    CursorConfig::ScaledAnimated(value) => {
      let ScaledAnimatedCursorConfig {
        nominal,
        hotspot,
        duration,
        durations,
        sequence,
        assets,
        aliases: _,
      } = value;

      if duration.is_none() && durations.is_none_or(|v| v.is_empty()) {
        todo!()
      }

      let num_frames = sequence.len();

      let duration_split = duration.and_then(|d| {
        Some(CursorDuration::from_milliseconds(
          d.milliseconds() / num_frames as u32,
        ))
      });

      let mut frames = Vec::with_capacity(num_frames);

      for frame in sequence.iter() {
        let asset_config = assets.get(frame.asset).unwrap();

        let image = Asset::from_config(
          frame.nominal.unwrap_or(nominal),
          frame.hotspot.unwrap_or(hotspot),
          asset_config,
        )?
        .into_image();

        // TODO: scale image for various DPIs
        let images = vec![image];
        let duration = frame.duration.or(duration_split);

        if duration.is_none() {
          panic!("duration should be Some")
        }

        frames.push(CursorFrame { images, duration });
      }

      Cursor {
        frames,
        metadata: None,
      }
    }
  })
}
