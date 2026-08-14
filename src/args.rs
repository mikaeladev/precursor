use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, Result as IoResult, stdin};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::cursors::CursorHotspot;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Args)]
pub struct BuildCursorArgs {
  /// Name of the cursor.
  #[arg(short = 'n', long = "name", requires_all = [ "size", "hotspot" ], required = false)]
  pub name: String,

  /// Width/height of the cursor.
  #[arg(short = 's', long = "size", requires_all = [ "name", "hotspot" ], required = false)]
  pub size: u16,

  /// Hotspot co-ordinates along the x and y axes.
  #[arg(
    short = 'H',
    long = "hotspot",
    value_name = "X,Y",
    requires_all = [ "name", "size" ],
    required = false
  )]
  pub hotspot: CursorHotspot,
}

#[derive(Subcommand)]
pub enum Command {
  /// Build some cursor files.
  #[command(visible_alias = "b")]
  Build {
    /// Path to an input file, or '-' for stdin.
    ///
    /// Input files may be either configs (.toml) or assets (.png). If an asset
    /// is passed, it must also be accompanied by '--name', '--size', and
    /// '--hotspot'.
    #[arg(group = "build-type")]
    input: InputArg,

    /// Where to output the built cursors.
    ///
    /// If multiple types are provided, this is assumed to be a directory.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Types of cursors to build (delimited by ',').
    #[arg(short = 't', long = "types", value_name = "TYPES", num_args = 1.., value_delimiter = ',', required = true)]
    cursor_types: Vec<CursorType>,

    #[command(flatten)]
    cursor_args: Option<BuildCursorArgs>,
  },

  /// Check a config file for syntax errors.
  #[command(visible_alias = "c")]
  Check {
    /// Path to the config file, or '-' for stdin.
    #[clap(default_value = "./precursor.toml")]
    input: InputArg,
  },

  /// Extract image frame(s) from a cursor file.
  #[command(visible_alias = "x")]
  Extract {
    /// Path to the cursor file, or '-' for stdin.
    input: InputArg,

    /// Image frame(s) to extract by indices (0-based).
    frames: Option<u32>,
  },

  /// Inspect a cursor file for metadata.
  #[command(visible_alias = "i")]
  Inspect {
    /// Path to the cursor file, or '-' for stdin.
    input: InputArg,
  },
}

#[derive(Clone)]
pub enum InputArg {
  Path(PathBuf),
  Stdin,
}

impl InputArg {
  pub fn open(&self) -> IoResult<BufReader<File>> {
    // https://github.com/rust-lang/rust/issues/72802#issue-627867529
    Ok(BufReader::new(match self {
      Self::Path(buf) => File::open(buf)?,
      Self::Stdin => {
        let lock = stdin().lock();

        #[cfg(any(target_family = "unix"))]
        let seekable_stdin = unsafe {
          use std::os::unix::io::{AsRawFd, FromRawFd};
          File::from_raw_fd(lock.as_raw_fd())
        };

        #[cfg(target_family = "windows")]
        let seekable_stdin = unsafe {
          use std::os::windows::io::{AsRawHandle, FromRawHandle};
          File::from_raw_handle(lock.as_raw_handle())
        };

        seekable_stdin
      }
    }))
  }
}

impl From<OsString> for InputArg {
  fn from(value: OsString) -> Self {
    if value == "-" {
      Self::Stdin
    } else {
      Self::Path(PathBuf::from(value))
    }
  }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum CursorType {
  Scalable,
  Windows,
  Xcursor,
}
