use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, Result as IoResult, stdin};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Precursor is a tool for building cross-platform cursor themes.
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
  /// Build cursor files.
  #[command(visible_alias = "b")]
  #[group(id = "types", multiple = true, required = true)]
  Build {
    /// Path to a config file, or '-' for stdin.
    #[clap(default_value = "./precursor.toml")]
    input: InputArg,

    /// Specify the directory in which to create the cursors.
    #[arg(short = 't', long, value_name = "DIRECTORY")]
    target_directory: Option<PathBuf>,

    /// Build scalable cursors.
    #[arg(short = 's', long, group = "types", conflicts_with = "all")]
    scalable: bool,

    /// Build windows cursors.
    #[arg(short = 'w', long, group = "types", conflicts_with = "all")]
    windows: bool,

    /// Build X11 cursors.
    #[arg(short = 'x', long, group = "types", conflicts_with = "all")]
    xcursor: bool,

    /// Build all cursor types (i.e. -swx).
    #[arg(short = 'A', long, group = "types")]
    all: bool,
  },

  /// Check a config file for errors.
  #[command(visible_alias = "c")]
  Check {
    /// Path to a config file, or '-' for stdin.
    #[clap(default_value = "./precursor.toml")]
    input: InputArg,
  },

  /// Extract frames from a cursor.
  #[command(visible_alias = "x")]
  Extract {
    /// Path to the cursor file, or '-' for stdin.
    input: InputArg,

    /// Image frame(s) to extract by indices (0-based).
    frames: Option<u32>,
  },

  /// Inspect a cursor for metadata.
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
