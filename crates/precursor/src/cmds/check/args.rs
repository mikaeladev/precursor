use precursor_filesys::InputArg;

use clap::Args;

const CONFIG_INPUT_HELP: &str = cfg_select! {
  unix => "Specify the config file (or '-' for standard input)",
  _ => "Specify the config file"
};

#[derive(Args)]
pub struct CheckArgs {
  /// See the value of [`CONFIG_INPUT_HELP`].
  #[arg(
    short = 'c',
    long = "config-file",
    value_name = "FILE",
    default_value = "./precursor.toml"
  )]
  #[arg(help = CONFIG_INPUT_HELP)]
  pub config_file_input: InputArg,
}
