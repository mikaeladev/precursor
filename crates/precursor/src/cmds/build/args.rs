use std::path::PathBuf;

use precursor_filesys::InputArg;

use clap::Args;

const CONFIG_INPUT_HELP: &str = cfg_select! {
  unix => "Specify the config file ('-' for standard input)",
  _ => "Specify the config file"
};

#[derive(Args)]
pub struct BuildArgs {
  /// See the value of [`CONFIG_INPUT_HELP`].
  #[arg(
    short = 'c',
    long = "config-file",
    value_name = "FILE",
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

  /// Replace existing destination files.
  #[arg(short = 'f', long)]
  pub force: bool,
}

#[derive(Args, Clone, Copy)]
#[group(id = "target_types", multiple = true, required = true)]
pub struct BuildTargetTypeArgs {
  /// Build all targets (-swx).
  #[arg(short = 'a', long, group = "target_types")]
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

impl BuildTargetTypeArgs {
  /// Returns whether or not cross-platform cursors are to be built.
  pub fn is_cross_platform(self) -> bool {
    self.all || ((self.scalable || self.xcursor) && self.windows)
  }

  pub fn includes_linux(self) -> bool {
    self.all || self.scalable || self.xcursor
  }

  pub fn includes_scalable(self) -> bool {
    self.all || self.scalable
  }

  pub fn includes_windows(self) -> bool {
    self.all || self.windows
  }

  pub fn includes_xcursor(self) -> bool {
    self.all || self.xcursor
  }
}
