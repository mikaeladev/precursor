use std::fs::File;
use std::io::{self, BufReader};

use crate_config::Config;

use crate::error::PrecursorResult;

pub fn read(reader: BufReader<File>) -> PrecursorResult<Config> {
  let config_str = io::read_to_string(reader)?;
  let config = toml::from_str::<Config>(&config_str)?;

  // TODO: error handling

  Ok(config)
}
