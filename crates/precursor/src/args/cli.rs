use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::args::input::InputArg;

#[derive(Parser)]
#[command(version, about, long_about, propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,

  /// Enable debug logs.
  #[arg(short = 'd', long)]
  pub debug: bool,
}

#[derive(Subcommand)]
pub enum Command {
  /// Build cursor files from a config file.
  #[command(visible_alias = "b")]
  Build(BuildArgs),

  /// Check a config file for errors.
  #[command(visible_alias = "c")]
  Check(CheckArgs),

  /// Inspect a cursor file for metadata.
  #[command(visible_alias = "i")]
  Inspect(InspectArgs),

  /// Extract frames from a cursor file.
  #[command(visible_alias = "x")]
  Extract(ExtractArgs),
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
pub struct BuildArgs {
  #[arg(
    short = 'c',
    long = "config-file",
    value_name = "FILE_PATH",
    default_value = "./precursor.toml"
  )]
  #[arg(help = CONFIG_INPUT_HELP)]
  pub config_file_input: InputArg,

  /// Specify the target directory.
  #[arg(
    short = 't',
    long = "target-directory",
    value_name = "DIRECTORY",
    default_value = "./out"
  )]
  pub target_dir_path: PathBuf,

  #[command(flatten)]
  pub target_types: BuildTargetTypeArgs,

  /// Remove contents of DIRECTORY before building.
  #[arg(short = 'e', long, alias = "clear")]
  pub empty: bool,

  /// Remove existing destination files.
  #[arg(short = 'f', long)]
  pub force: bool,
}

#[derive(Args, Clone, Copy)]
#[group(id = "target_types", multiple = true, required = true)]
pub struct BuildTargetTypeArgs {
  /// Build all targets (-swx).
  #[arg(short = 'A', long, group = "target_types")]
  pub all: bool,

  /// Build SVG cursors.
  #[arg(short = 's', long, group = "target_types", conflicts_with = "all")]
  pub scalable: bool,

  /// Build Windows cursors.
  #[arg(short = 'w', long, group = "target_types", conflicts_with = "all")]
  pub windows: bool,

  /// Build X11 cursors.
  #[arg(short = 'x', long, group = "target_types", conflicts_with = "all")]
  pub xcursor: bool,
}

#[derive(Args)]
pub struct CheckArgs {
  #[arg(
    short = 'c',
    long = "config-file",
    value_name = "FILE_PATH",
    default_value = "./precursor.toml"
  )]
  #[arg(help = CONFIG_INPUT_HELP)]
  pub config_file_input: InputArg,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CursorKindHint {
  #[clap(alias = "a")]
  Ani,
  #[clap(alias = "c")]
  Cur,
  #[clap(aliases = ["s", "scalable"])]
  Svg,
  #[clap(aliases = ["x", "x11", "xcursor"])]
  Xcur,
}

#[derive(Args)]
pub struct ExtractArgs {
  #[arg(value_name = "FILE_PATH")]
  #[arg(help = CURSOR_INPUT_HELP)]
  pub cursor_file_input: InputArg,

  /// Specify the target directory.
  #[arg(
    short = 't',
    long = "target-directory",
    value_name = "DIRECTORY",
    default_value = "./out"
  )]
  pub target_dir_path: PathBuf,

  /// Specify the kind of cursor to expect.
  #[arg(short = 'k', long = "kind", value_name = "KIND")]
  pub kind_hint: Option<CursorKindHint>,

  /// Remove contents of DIRECTORY before building.
  #[arg(short = 'e', long, alias = "clear")]
  pub empty: bool,

  /// Remove existing destination files.
  #[arg(short = 'f', long)]
  pub force: bool,
}

#[derive(Args)]
pub struct InspectArgs {
  #[arg(value_name = "FILE_PATH")]
  #[arg(help = CURSOR_INPUT_HELP)]
  pub cursor_file_input: InputArg,
}

/// Parse from `std::env::args_os()`, exit on error.
pub fn parse() -> Cli {
  Cli::parse()
}
