mod config;
mod cursors;

use std::env::current_dir;
use std::fs::{File, read_to_string};
use std::io::{BufReader, Result as IoResult};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::config::Config;
use crate::cursors::{CursorImage, CursorType, StaticWindowsCursor};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  /// Build a cursor file.
  #[command(visible_alias = "b")]
  Build {
    /// Paths to asset files.
    #[arg(num_args = 1..)]
    asset_paths: Vec<PathBuf>,

    /// Types of cursors to build, delimited by ','.
    #[arg(short = 't', long = "type", num_args = 1.., value_delimiter = ',', required = true)]
    cursor_types: Vec<CursorType>,

    /// Name of the cursor.
    #[arg(short = 'n', long = "name")]
    cursor_name: String,

    /// Nominal width/height of the cursor.
    #[arg(short = 's', long = "size")]
    nominal_size: usize,

    /// Hotspot co-ordinate for the x-axis.
    #[arg(short = 'X', long = "hotx", default_value_t = 0)]
    hotspot_x: usize,

    /// Hotspot co-ordinate for the y-axis.
    #[arg(short = 'Y', long = "hoty", default_value_t = 0)]
    hotspot_y: usize,

    /// Where to output the built cursor file(s).
    ///
    /// If multiple types are provided, this will be treated as a directory.
    #[arg(short = 'o', long = "output")]
    target_path: Option<PathBuf>,
  },

  /// Check a config file for syntax errors.
  #[command(visible_alias = "c")]
  Check {
    /// Path to the config file.
    #[clap(default_value = "./precursor.toml")]
    config_path: PathBuf,
  },

  /// Extract image frame(s) from a cursor file.
  #[command(visible_alias = "x")]
  Extract {
    /// Path to the cursor file.
    cursor_path: PathBuf,

    /// Image frame(s) to extract by indices (0-based).
    frames: Option<u32>,
  },

  /// Inspect a cursor file for metadata.
  #[command(visible_alias = "i")]
  Inspect {
    /// Path to the cursor file.
    cursor_path: PathBuf,
  },
}

fn main() -> IoResult<()> {
  let args = Args::parse();

  match args.command {
    Command::Build {
      asset_paths,
      cursor_types,
      cursor_name,
      nominal_size,
      hotspot_x,
      hotspot_y,
      target_path,
    } => {
      let is_static = true; // always true for now

      let output_path = match target_path {
        Some(path) => (path, cursor_types.len() != 1),
        None => (current_dir()?, true),
      };

      let mut cursor_images = Vec::with_capacity(asset_paths.len());

      for asset_path in asset_paths {
        let asset_reader = BufReader::new(File::open(&asset_path)?);

        // assume PNG for testing purposes
        let cursor_image = CursorImage::from_png(
          nominal_size as u16,
          (hotspot_x as u16, hotspot_y as u16),
          asset_reader,
        )?;

        cursor_images.push(cursor_image);
      }

      if let (path, is_dir) = &output_path
        && *is_dir
        && !path.is_dir()
      {
        // TODO: handle elegantly
        panic!("output directory not found")
      }

      for cursor_type in cursor_types {
        let mut cursor_path = output_path.0.clone();

        if output_path.1 {
          cursor_path.push(&cursor_name);
        }

        match cursor_type {
          CursorType::Windows => {
            if is_static {
              cursor_path.set_extension("cur");

              StaticWindowsCursor::new(cursor_images.to_vec())
                .write(File::create(cursor_path)?)?;
            } else {
              cursor_path.set_extension("ani");

              todo!()
            }
          }
          _ => todo!(),
        }
      }
    }

    Command::Check { config_path } => {
      let config_str = read_to_string(config_path)?;
      let config = toml::from_str::<Config>(&config_str);

      match config {
        Ok(_) => println!("Success!"),
        Err(err) => eprintln!("Error parsing config: {err}"),
      }
    }

    Command::Extract {
      cursor_path: _,
      frames: _,
    } => {
      todo!()
    }

    Command::Inspect { cursor_path: _ } => {
      todo!()
    }
  }

  Ok(())
}
