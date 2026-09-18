use crate::args::CheckArgs;
use crate::error::PrecursorResult;
use crate::filesys;
use crate::paths;

pub fn check(CheckArgs { config_file_input }: CheckArgs) -> PrecursorResult {
  let working_dir_path = paths::get_working_dir_path()?;

  filesys::get_config(&working_dir_path, config_file_input)?;

  println!("Success!");
  Ok(())
}
