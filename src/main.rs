mod args;
mod config;
mod cursors;
mod error;
mod images;

use std::env::current_dir;
use std::fs::File;
use std::io::{ErrorKind as IoErrorKind, read_to_string};

use clap::Parser;

use crate::args::*;
use crate::config::Config;
use crate::cursors::*;
use crate::error::Error;
use crate::images::PngImage;

fn main() -> Result<(), Error> {
  let args = Cli::parse();

  match args.command {
    Command::Build {
      input,
      output,
      cursor_types,
      cursor_args,
    } => {
      let input_reader = input.open()?;

      let target_path = match output {
        Some(path) => (path, cursor_types.len() != 1),
        None => (current_dir()?, true),
      };

      if let (path, is_dir) = &target_path
        && *is_dir
        && !path.is_dir()
      {
        return Err(IoErrorKind::NotADirectory.into());
      }

      if let Some(BuildCursorArgs {
        name,
        size,
        hotspot,
      }) = cursor_args
      {
        // assume a single PNG for testing purposes
        let rgba = PngImage::decode(input_reader)?.into();

        let image = CursorImage {
          nominal: size as u32,
          hotspot,
          rgba: &rgba,
        };

        let frame = CursorFrame {
          images: &[image],
          duration: None,
        };

        let cursor = Cursor {
          frames: vec![frame],
          metadata: None,
        };

        for cursor_type in cursor_types {
          let mut cursor_path = target_path.0.clone();

          if target_path.1 {
            cursor_path.push(&name);
          }

          match cursor_type {
            CursorType::Windows => {
              cursor_path.set_extension("cur");
              WindowsCursor::from(&cursor).write(File::create(cursor_path)?)?;
            }
            CursorType::Xcursor => {
              XCursor::from(&cursor).write(File::create(cursor_path)?)?;
            }
            _ => todo!(),
          }
        }
      } else {
        let config_str = read_to_string(input_reader)?;
        let config = toml::from_str::<Config>(&config_str)?;

        for (name, cursor) in config.cursors {
          let mut cursor_path = target_path.0.clone();

          if target_path.1 {
            cursor_path.push(&name);
          }

          todo!()
        }
      };
    }

    Command::Check { input } => {
      let input_reader = input.open()?;

      let config_str = read_to_string(input_reader)?;
      let config = toml::from_str::<Config>(&config_str);

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
