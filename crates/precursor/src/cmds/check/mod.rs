mod args;

use precursor_error::Result;
use precursor_filesys as filesys;

pub use args::*;

pub fn check(CheckArgs { config_file_input }: CheckArgs) -> Result<()> {
  filesys::get_config(config_file_input)?;

  println!("Success!");
  Ok(())
}
