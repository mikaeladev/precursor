use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate_config::Config;

use crate::args::InputArg;
use crate::debug;
use crate::error::PrecursorResult;
use crate::path_error_msg;

// TODO: doc
pub fn get_config(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> PrecursorResult<Config> {
  let config_string = get_config_string(working_dir_path, input_arg)?;
  let config_value = toml::from_str(&config_string)?;

  Ok(config_value)
}

// TODO: doc
pub fn get_config_string(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> io::Result<String> {
  match input_arg {
    InputArg::Path(path) => {
      let file_path = match path.is_absolute() {
        true => path,
        false => working_dir_path.as_ref().join(path),
      };

      fs::read_to_string(&file_path).map_err(|err| {
        let kind = "config file";
        let action = format_args!("read {kind}");

        let err_kind = err.kind();

        match err_kind {
          ErrorKind::InvalidData => io::Error::new(
            err_kind,
            path_error_msg!(invalid_data: kind, file_path),
          ),
          ErrorKind::NotFound => io::Error::new(
            err_kind,
            path_error_msg!(not_found: kind, file_path),
          ),
          ErrorKind::PermissionDenied => io::Error::new(
            err_kind,
            path_error_msg!(action_denied: action, file_path),
          ),
          _ => {
            debug!("{err}");

            io::Error::new(
              err_kind,
              path_error_msg!(action_failed: action, file_path),
            )
          }
        }
      })
    }

    #[cfg(target_family = "unix")]
    InputArg::Stdin => {
      let lock = io::stdin().lock();

      io::read_to_string(lock).map_err(|err| {
        let err_kind = err.kind();

        match err_kind {
          ErrorKind::InvalidData => {
            io::Error::new(err_kind, "expected stdin to be valid UTF-8")
          }
          _ => {
            debug!("{err}");
            io::Error::new(err_kind, "failed to read config from stdin")
          }
        }
      })
    }
  }
}
