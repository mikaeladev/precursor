use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

use crate::args::InputArg;
use crate::debug;
use crate::path_error_msg;

pub enum CursorReader {
  File(BufReader<File>),
  #[cfg(target_family = "unix")]
  Stdin(io::Cursor<Vec<u8>>),
}

impl Read for CursorReader {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    match self {
      Self::File(f) => f.read(buf),
      Self::Stdin(v) => v.read(buf),
    }
  }
}

impl Seek for CursorReader {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    match self {
      Self::File(f) => f.seek(pos),
      Self::Stdin(v) => v.seek(pos),
    }
  }
}

// TODO: doc
pub fn get_cursor_reader(
  working_dir_path: impl AsRef<Path>,
  input_arg: InputArg,
) -> io::Result<CursorReader> {
  match input_arg {
    InputArg::Path(path) => {
      let file_path = match path.is_absolute() {
        true => path,
        false => working_dir_path.as_ref().join(path),
      };

      let file = File::open(&file_path).map_err(|err| {
        let kind = "cursor file";
        let action = format_args!("read {kind}");

        let err_kind = err.kind();

        match err_kind {
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
      })?;

      Ok(CursorReader::File(BufReader::new(file)))
    }

    #[cfg(target_family = "unix")]
    InputArg::Stdin => {
      let mut lock = io::stdin().lock();
      let mut buffer = Vec::new();

      lock.read_to_end(&mut buffer).map_err(|err| {
        debug!("{err}");
        io::Error::new(err.kind(), "failed to read stdin to end")
      })?;

      Ok(CursorReader::Stdin(io::Cursor::new(buffer)))
    }
  }
}

#[cfg(target_family = "unix")]
mod unix {
  use std::convert::Infallible;
  use std::ffi::OsStr;

  use crate::filesys;

  use super::*;

  // TODO: doc
  pub fn symlink_cursor(
    base_path: impl AsRef<Path>,
    cursor_name: impl AsRef<OsStr>,
    alias_name: impl AsRef<OsStr>,
    force: bool,
  ) -> io::Result<()> {
    use io::ErrorKind;
    use std::os::unix::fs as unix_fs;

    let base_path = base_path.as_ref();
    let cursor_name = cursor_name.as_ref();
    let alias_name = alias_name.as_ref();

    let cursor_path = base_path.join(cursor_name);
    let link_path = base_path.join(alias_name);

    if let Err(err) = filesys::get_metadata(&cursor_path)
      && err.kind() == ErrorKind::NotFound
    {
      return Err(io::Error::new(
        ErrorKind::NotFound,
        path_error_msg!(not_found: "cursor file or directory", cursor_path),
      ));
    }

    match filesys::get_metadata(&link_path) {
      Ok(meta) => {
        if !force {
          return Err(io::Error::new(
            ErrorKind::AlreadyExists,
            path_error_msg!(already_exists: "cursor file or directory", cursor_path),
          ));
        } else if meta.is_dir() {
          return Err(io::Error::new(
            ErrorKind::IsADirectory,
            path_error_msg!(expected_found: "optional file or symlink", "a directory", cursor_path),
          ));
        } else {
          filesys::remove_file::<Infallible>(&link_path, None)?;
        }
      }
      Err(err) if err.kind() == ErrorKind::NotFound => (),
      Err(err) => return Err(err),
    }

    unix_fs::symlink(cursor_name, &link_path).map_err(|err| {
      let action = "symlink cursor file or directory";
      let err_kind = err.kind();

      match err_kind {
        ErrorKind::PermissionDenied => io::Error::new(
          err_kind,
          path_error_msg!(action_denied: action, link_path),
        ),
        _ => {
          debug!("{err}");

          io::Error::new(
            err_kind,
            path_error_msg!(action_failed: action, link_path),
          )
        }
      }
    })
  }
}

#[cfg(target_family = "unix")]
pub use unix::*;
