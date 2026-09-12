use crate_config::Config;

use crate::args::CheckArgs;
use crate::error::PrecursorResult;
use crate::filesys;

pub fn check(CheckArgs { config_file_input }: CheckArgs) -> PrecursorResult {
  let working_dir_path = filesys::get_working_dir_path()?;

  let config_string = filesys::read_config_string_from_input(
    &working_dir_path,
    config_file_input,
  )?;

  toml::from_str::<Config>(&config_string)?;

  println!("Success!");
  Ok(())
}
