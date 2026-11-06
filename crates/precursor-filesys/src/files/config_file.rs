use std::io::{self, ErrorKind};

use precursor_config::Config;
use precursor_error::{ErrorMessage, Result};
use precursor_paths as paths;

use crate::input::InputArg;
use crate::macros::{action_failed, expected_utf8};
use crate::metapath::AsMetaPath;

// TODO: doc
pub fn get_config(input_arg: InputArg) -> Result<Config> {
  let config_string = read_config_file(input_arg)?;

  match toml::from_str(&config_string) {
    Ok(config) => Ok(config),
    Err(src) => Err(
      action_failed!("parse config from TOML")
        .into_error()
        .with_source(src),
    ),
  }
}

// TODO: doc
fn read_config_file(input_arg: InputArg) -> Result<String> {
  match input_arg {
    InputArg::Path(path) => crate::read_file_to_string(
      match path.is_absolute() {
        true => path,
        false => paths::working_dir().join(path),
      }
      .as_meta_path("config file"),
    ),

    #[cfg(target_family = "unix")]
    InputArg::Stdin => {
      let lock = io::stdin().lock();

      io::read_to_string(lock).map_err(|src| {
        match src.kind() {
          ErrorKind::InvalidData => expected_utf8!("stdin"),
          _ => action_failed!("read config from stdin"),
        }
        .into_error()
        .with_source(src)
      })
    }
  }
}
