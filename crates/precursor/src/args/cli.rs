use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use super::InputArg;

#[derive(Parser)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
  /// Build cursor files.
  #[command(visible_alias = "b")]
  Build(BuildArgs),
  /// Check a config file for errors.
  #[command(visible_alias = "c")]
  Check(CheckArgs),
  /// Extract frames from a cursor file.
  #[command(visible_alias = "x")]
  Extract(ExtractArgs),
  /// Inspect a cursor file for metadata.
  #[command(visible_alias = "i")]
  Inspect(InspectArgs),
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

#[derive(Args)]
#[group(id = "target_types", multiple = true, required = true)]
pub struct BuildArgs {
  #[clap(default_value = "./precursor.toml", help = CONFIG_INPUT_HELP)]
  pub input: InputArg,

  /// Specify the target directory.
  #[arg(short = 't', long = "target-directory", value_name = "DIRECTORY")]
  pub target_dir: Option<PathBuf>,

  /// Build SVG cursors.
  #[arg(short = 's', long, group = "target_types", conflicts_with = "all")]
  pub scalable: bool,

  /// Build Windows cursors.
  #[arg(short = 'w', long, group = "target_types", conflicts_with = "all")]
  pub windows: bool,

  /// Build X11 cursors.
  #[arg(short = 'x', long, group = "target_types", conflicts_with = "all")]
  pub xcursor: bool,

  /// Equivalent to setting -swx.
  #[arg(short = 'A', long, group = "target_types")]
  pub all: bool,
}

#[derive(Args)]
pub struct CheckArgs {
  #[clap(default_value = "./precursor.toml", help = CONFIG_INPUT_HELP)]
  pub input: InputArg,
}

#[derive(Args)]
pub struct ExtractArgs {
  #[clap(help = CURSOR_INPUT_HELP)]
  pub input: InputArg,

  /// Specify frames to extract (0-based).
  #[arg(short = 'f', long, value_name = "INDICES")]
  pub frames: Option<u32>,
}

#[derive(Args)]
pub struct InspectArgs {
  #[clap(help = CURSOR_INPUT_HELP)]
  pub input: InputArg,
}

/// Parse from `std::env::args_os()`, exit on error.
pub fn parse() -> Cli {
  // TODO: return a result
  Cli::parse()
}
