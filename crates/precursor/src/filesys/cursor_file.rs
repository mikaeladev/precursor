use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

use crate::args::InputArg;
use crate::debug;
use crate::path_error_msg;

use super::EntityKind;

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

// TODO: doc
pub fn prepare_cursor_file(
  path: impl AsRef<Path>,
  force: bool,
) -> io::Result<()> {
  super::prepare_file(path, "cursor file", force)
}

#[cfg(target_family = "unix")]
use std::ffi::OsStr;

// TODO: doc
#[cfg(target_family = "unix")]
pub fn symlink_cursor_file(
  base_path: impl AsRef<Path>,
  cursor_name: impl AsRef<OsStr>,
  alias_name: impl AsRef<OsStr>,
) -> io::Result<()> {
  use io::ErrorKind;
  use std::os::unix::fs as unix_fs;

  let base_path = base_path.as_ref();
  let cursor_name = cursor_name.as_ref();
  let alias_name = alias_name.as_ref();

  let cursor_path = base_path.join(cursor_name);
  let link_path = base_path.join(alias_name);

  if !super::entity_exists(EntityKind::Either, &cursor_path)? {
    return Err(io::Error::new(
      ErrorKind::NotFound,
      path_error_msg!(not_found: "cursor file or directory", cursor_path),
    ));
  }

  if super::entity_exists(EntityKind::Either, &link_path)? {
    return Err(io::Error::new(
      ErrorKind::AlreadyExists,
      path_error_msg!(already_exists: "cursor file or directory", cursor_path),
    ));
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
