use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

use precursor_error::{Error, Result};

use crate::error::ErrorMessageWithPath;
use crate::input::InputArg;
use crate::macros::{action_denied, action_failed};

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
) -> Result<CursorReader> {
  match input_arg {
    InputArg::Path(mut path) => {
      if path.is_relative() {
        path = working_dir_path.as_ref().join(path);
      }

      let file = File::open(&path).map_err(|src| {
        let action = "read cursor file";

        let src_kind = src.kind();

        let msg = match src_kind {
          ErrorKind::PermissionDenied => action_denied!(action),
          // ErrorKind::NotFound => action_failed!(action, src_kind),
          _ => action_failed!(action, src_kind),
        };

        Error::new(ErrorMessageWithPath::new(msg, path), src)
      })?;

      Ok(CursorReader::File(BufReader::new(file)))
    }

    #[cfg(target_family = "unix")]
    InputArg::Stdin => {
      use std::io::BufRead;

      let map_err_op =
        |src| Error::new(action_failed!("read stdin to end"), src);

      let mut lock = io::stdin().lock();

      let capacity = lock.fill_buf().map_err(map_err_op)?.len();
      let mut buffer = Vec::with_capacity(capacity);

      lock.read_to_end(&mut buffer).map_err(map_err_op)?;

      Ok(CursorReader::Stdin(io::Cursor::new(buffer)))
    }
  }
}

#[cfg(target_family = "unix")]
mod unix {
  use std::convert::Infallible;
  use std::ffi::OsStr;

  use super::*;

  // TODO: doc
  pub fn symlink_cursor(
    base_path: impl AsRef<Path>,
    cursor_name: impl AsRef<OsStr>,
    alias_name: impl AsRef<OsStr>,
    force: bool,
  ) -> Result<()> {
    use io::ErrorKind;
    use std::os::unix::fs as unix_fs;

    let base_path = base_path.as_ref();
    let cursor_name = cursor_name.as_ref();
    let alias_name = alias_name.as_ref();

    let cursor_path = base_path.join(cursor_name);
    let link_path = base_path.join(alias_name);

    crate::get_metadata(&cursor_path, false).map_err(|src| {
      let msg = ErrorMessageWithPath::new(
        action_failed!("query cursor path"),
        cursor_path,
      );

      Error::new(msg, src)
    })?;

    match crate::get_metadata(&link_path, false) {
      Ok(meta) => {
        if !force {
          Err(Error::new::<Infallible>(
            action_failed!("symlink cursor", "entity already exists at target"),
            None,
          ))
        } else {
          let kind = "existing cursor target";

          if meta.is_dir() {
            crate::remove_dir_all(&link_path, kind)
          } else {
            crate::remove_file(&link_path, kind)
          }
        }
      }
      Err(err)
        if let Some(src) = err.downcast::<io::Error>()
          && src.kind() == ErrorKind::NotFound =>
      {
        Ok(())
      }
      Err(err) => Err(err),
    }?;

    unix_fs::symlink(cursor_name, &link_path).map_err(|src| {
      let action = "symlink cursor file or directory";
      let src_kind = src.kind();

      let msg = match src_kind {
        ErrorKind::PermissionDenied => action_denied!(action),
        _ => action_failed!(action, src_kind),
      };

      Error::new(ErrorMessageWithPath::new(msg, link_path), src)
    })
  }
}

#[cfg(target_family = "unix")]
pub use unix::*;
