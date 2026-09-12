use std::fs;
use std::io;
use std::path::PathBuf;

use crate::args::InputArg;
use crate::debug;
use crate::path_error_msg;

pub fn read_config_string_from_input(
  working_dir_path: &PathBuf,
  input_arg: InputArg,
) -> io::Result<String> {
  use io::ErrorKind::{InvalidData, NotFound, PermissionDenied};

  match input_arg {
    InputArg::Path(path) => {
      let file_path = match path.is_absolute() {
        true => path,
        false => working_dir_path.join(path),
      };

      check_config_file_path(&file_path)?;

      let string_result = fs::read_to_string(&file_path);

      if let Err(err) = string_result {
        let err_kind = err.kind();

        Err(match err_kind {
          InvalidData => io::Error::new(
            err_kind,
            path_error_msg!(invalid_data: "config file", file_path),
          ),
          NotFound => io::Error::new(
            err_kind,
            path_error_msg!(not_found: "config file", file_path),
          ),
          PermissionDenied => io::Error::new(
            err_kind,
            path_error_msg!(denied_action: "read config file", file_path),
          ),
          _ => {
            debug!("{err}");

            io::Error::new(
              err_kind,
              path_error_msg!(failed_action: "read config file", file_path),
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

pub fn check_config_file_path(file_path: &PathBuf) -> io::Result<()> {
  if !super::entity_exists(super::EntityKind::File, "config", &file_path)? {
    Err(io::Error::new(
      io::ErrorKind::NotFound,
      path_error_msg!(not_found: "config file", file_path),
    ))
  } else {
    Ok(())
  }
}
