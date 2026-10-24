use std::path::PathBuf;

use precursor_cursor::CursorKind;
use precursor_filesys::InputArg;

use clap::Args;

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
