use std::ffi::OsStr;
use std::io::{self, Read, Seek};
use std::path::PathBuf;

use precursor_filesys::InputArg;

use clap::{Args, ValueEnum};

const CURSOR_INPUT_HELP: &str = cfg_select! {
  unix => "Specify the cursor file (or '-' for standard input)",
  _ => "Specify the cursor file"
};

#[derive(Args)]
pub struct ExtractArgs {
  /// See the value of [`CURSOR_INPUT_HELP`].
  #[arg(value_name = "FILE")]
  #[arg(help = CURSOR_INPUT_HELP)]
  pub cursor_file_input: InputArg,

  /// Specify the kind of cursor to expect.
  #[arg(short = 'k', value_name = "KIND")]
  pub cursor_kind: Option<CursorKind>,

  /// Specify the target directory.
  #[arg(
    short = 't',
    long = "target-directory",
    value_name = "DIRECTORY",
    default_value = "./out"
  )]
  pub target_dir_path: PathBuf,

  /// Remove contents of DIRECTORY before building.
  #[arg(short = 'e', long, alias = "clear")]
  pub empty: bool,

  /// Replace existing destination files.
  #[arg(short = 'f', long)]
  pub force: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CursorKind {
  ANI,
  CUR,
  #[clap(aliases = ["scalable"])]
  SVG,
  #[clap(aliases = ["xcur", "xcursor"])]
  X11,
}

impl CursorKind {
  const ANI_EXT: &str = "ani";
  const CUR_EXT: &str = "cur";
  const SVG_EXT: &str = "svg";

  pub fn from_ext(ext: &OsStr) -> Option<Self> {
    if ext == OsStr::new(Self::ANI_EXT) {
      Some(Self::ANI)
    } else if ext == OsStr::new(Self::CUR_EXT) {
      Some(Self::CUR)
    } else if ext == OsStr::new(Self::SVG_EXT) {
      Some(Self::SVG)
    } else {
      None
    }
  }

  pub fn from_bytes(buf: &[u8]) -> Option<Self> {
    // order by highest confidence
    if buf.len() < 12 {
      None
    } else if &buf[0..3] == b"RIFF" && &buf[8..11] == b"ACON" {
      Some(Self::ANI)
    } else if &buf[0..3] == b"Xcur" {
      Some(Self::X11)
    } else if &buf[0..4] == &[0, 0, 2, 0] {
      Some(Self::CUR)
    } else {
      None
    }
  }

  pub fn from_reader<R: Read + Seek>(
    reader: &mut R,
  ) -> io::Result<Option<Self>> {
    let mut buf = [0; 12];
    reader.read_exact(&mut buf)?;
    reader.rewind()?;
    Ok(Self::from_bytes(&buf))
  }
}
