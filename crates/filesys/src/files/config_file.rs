use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use precursor_config::Config;
use precursor_error::{Error, Result};

use crate::error::ErrorMessageWithPath;
use crate::input::InputArg;
use crate::macros::{action_denied, action_failed, expected_utf8};

// TODO: doc
pub fn get_config(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> Result<Config> {
  let config_string = get_config_string(working_dir_path, input_arg)?;

  match toml::from_str(&config_string) {
    Ok(config) => Ok(config),
    Err(src) => Err(Error::new(action_failed!("parse config from TOML"), src)),
  }
}

// TODO: doc
pub fn get_config_string(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> Result<String> {
  match input_arg {
    InputArg::Path(path) => {
      let file_path = match path.is_absolute() {
        true => path,
        false => working_dir_path.as_ref().join(path),
      };

      fs::read_to_string(&file_path).map_err(|src| {
        let kind = "config file";
        let action = format_args!("read {kind}");

        let src_kind = src.kind();

        let msg = match src_kind {
          ErrorKind::PermissionDenied => action_denied!(action),
          ErrorKind::InvalidData => expected_utf8!(kind),
          _ => action_failed!(action, src_kind),
        };

        Error::new(ErrorMessageWithPath::new(msg, file_path), src)
      })
    }

    #[cfg(target_family = "unix")]
    InputArg::Stdin => {
      let lock = io::stdin().lock();

      io::read_to_string(lock).map_err(|src| {
        let msg = match src.kind() {
          ErrorKind::InvalidData => expected_utf8!("stdin"),
          _ => action_failed!("read config from stdin"),
        };

        Error::new(msg, src)
      })
    }
  }
}
