use clap::{Parser, Subcommand};

use crate::cmds::{BuildArgs, CheckArgs, ExtractArgs};

#[derive(Parser)]
#[command(version, about, long_about, propagate_version = true)]
#[group(id = "log_level", multiple = false)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,

  /// Silence all logs.
  ///
  /// Conflicts with '--debug' and '--verbose'.
  #[arg(short = 'q', long, global = true, group = "log_level")]
  pub quiet: bool,

  /// Print debug logs.
  ///
  /// Conflicts with '--quiet' and '--verbose'.
  #[arg(short = 'd', long, global = true, group = "log_level")]
  pub debug: bool,

  /// Print verbose logs.
  ///
  /// Conflicts with '--quiet' and '--debug'.
  #[arg(short = 'v', long, global = true, group = "log_level")]
  pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
  /// Build cursors from a config file.
  #[command(visible_alias = "b")]
  Build(BuildArgs),

  /// Check a config file for errors.
  #[command(visible_alias = "c")]
  Check(CheckArgs),

  /// Inspect a cursor for metadata.
  #[command(visible_alias = "i")]
  Inspect, // TODO

  /// Extract image frames from a cursor.
  #[command(visible_alias = "x")]
  Extract(ExtractArgs),
}

/// Parse from `std::env::args_os()`, exit on error.
pub fn parse() -> Cli {
  Cli::parse()
}
