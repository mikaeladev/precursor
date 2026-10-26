use std::fs;
use std::io::{self, ErrorKind};

use precursor_config::Config;
use precursor_error::{ErrorMessage, Result};
use precursor_paths as paths;

use crate::input::InputArg;
use crate::macros::{action_denied, action_failed, expected_utf8};

// TODO: doc
pub fn get_config(input_arg: InputArg) -> Result<Config> {
  let config_string = get_config_string(input_arg)?;

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
pub fn get_config_string(input_arg: InputArg) -> Result<String> {
  match input_arg {
    InputArg::Path(path) => {
      let file_path = match path.is_absolute() {
        true => path,
        false => paths::working_dir().join(path),
      };

      fs::read_to_string(&file_path).map_err(|src| {
        let kind = "config file";
        let action = format_args!("read {kind}");

        let src_kind = src.kind();

        match src_kind {
          ErrorKind::PermissionDenied => action_denied!(action),
          ErrorKind::InvalidData => expected_utf8!(kind),
          _ => action_failed!(action, src_kind),
        }
        .with_context([("path", file_path.display())])
        .into_error()
        .with_source(src)
      })
    }

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
