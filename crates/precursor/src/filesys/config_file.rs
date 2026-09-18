use std::fs;
use std::io;
use std::path::Path;

use crate_config::Config;

use crate::args::InputArg;
use crate::error::PrecursorResult;
use crate::{debug, path_error_msg};

pub fn get_config(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> PrecursorResult<Config> {
  let config_string = get_config_string(working_dir_path, input_arg)?;
  let config_value = toml::from_str(&config_string)?;

  Ok(config_value)
}

pub fn get_config_string(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> io::Result<String> {
  use io::ErrorKind::{InvalidData, NotFound, PermissionDenied};

  match input_arg {
    InputArg::Path(path) => {
      let file_path = match path.is_absolute() {
        true => path,
        false => working_dir_path.as_ref().join(path),
      };

      if !super::entity_exists(super::EntityKind::File, &file_path)? {
        return Err(io::Error::new(
          NotFound,
          path_error_msg!(not_found: "config file", file_path),
        ));
      }

      let string_result = fs::read_to_string(&file_path);

      if let Err(err) = string_result {
        let err_kind = err.kind();

        Err(match err_kind {
          InvalidData => io::Error::new(
            err_kind,
            path_error_msg!(invalid_data: "config file", file_path),
          ),
          PermissionDenied => io::Error::new(
            err_kind,
            path_error_msg!(action_denied: "read config file", file_path),
          ),
          _ => {
            debug!("{err}");

            io::Error::new(
              err_kind,
              path_error_msg!(action_failed: "read config file", file_path),
            )
          }
        })
      } else {
        string_result
      }
    }
    #[cfg(target_family = "unix")]
    InputArg::Stdin => {
      let lock = io::stdin().lock();
      let string_result = io::read_to_string(lock);

      if let Err(err) = string_result {
        let err_kind = err.kind();

        Err(match err_kind {
          InvalidData => io::Error::new(
            err_kind,
            "expected config from stdin to be valid UTF-8",
          ),
          _ => {
            debug!("{err}");
            io::Error::new(err_kind, "failed to read config from stdin")
          }
        })
      } else {
        string_result
      }
    }
  }
}
