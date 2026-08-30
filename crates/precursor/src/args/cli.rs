use std::path::PathBuf;

use clap::{Parser, Subcommand};

use super::InputArg;

/// Precursor is a tool for building cross-platform cursor themes.
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

macro_rules! const_input_help {
  ($var:ident, $data:literal) => {
    #[cfg(target_family = "unix")]
    const $var: &str =
      concat!("Specify the ", $data, " file ('-' for standard input)");

    #[cfg(not(target_family = "unix"))]
    const $var: &str = concat!("Specify the ", $data, " file");
  };
}

const_input_help!(CONFIG_INPUT_HELP, "config");
const_input_help!(CURSOR_INPUT_HELP, "cursor");

#[derive(Subcommand)]
pub enum Command {
  /// Build cursor files.
  #[command(visible_alias = "b")]
  #[group(id = "types", multiple = true, required = true)]
  Build {
    #[clap(default_value = "./precursor.toml", help = CONFIG_INPUT_HELP)]
    input: InputArg,

    /// Specify the target directory.
    #[arg(short = 't', long = "target-directory", value_name = "DIRECTORY")]
    target_dir: Option<PathBuf>,

    /// Build SVG cursors.
    #[arg(short = 's', long, group = "types", conflicts_with = "all")]
    scalable: bool,

    /// Build Windows cursors.
    #[arg(short = 'w', long, group = "types", conflicts_with = "all")]
    windows: bool,

    /// Build X11 cursors.
    #[arg(short = 'x', long, group = "types", conflicts_with = "all")]
    xcursor: bool,

    /// Equivalent to setting -swx.
    #[arg(short = 'A', long, group = "types")]
    all: bool,
  },

  /// Check a config file for errors.
  #[command(visible_alias = "c")]
  Check {
    #[clap(default_value = "./precursor.toml", help = CONFIG_INPUT_HELP)]
    input: InputArg,
  },

  /// Extract frames from a cursor file.
  #[command(visible_alias = "x")]
  Extract {
    #[clap(help = CURSOR_INPUT_HELP)]
    input: InputArg,

    /// Specify frames to extract (0-based).
    #[arg(short = 'f', long, value_name = "INDICES")]
    frames: Option<u32>,
  },

  /// Inspect a cursor file for metadata.
  #[command(visible_alias = "i")]
  Inspect {
    #[clap(help = CURSOR_INPUT_HELP)]
    input: InputArg,
  },
}

/// Parse from `std::env::args_os()`, exit on error.
pub fn parse() -> Cli {
  // TODO: return a result
  Cli::parse()
}
