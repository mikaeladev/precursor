use std::env;
use std::fs::{self, File};
use std::io::{ErrorKind as IoErrorKind, Result as IoResult};
use std::path::PathBuf;

use crate_config::CursorTargets;
use crate_formats::write::WriteTo;
use crate_formats::{AniFile, CurFile, XcursorFile};

use crate::args::BuildArgs;
use crate::config;
use crate::cursor::{Cursor, FromCursor};
use crate::error::PrecursorResult;

pub fn build(
  BuildArgs {
    input,
    target_dir,
    scalable,
    windows,
    xcursor,
    all,
  }: BuildArgs,
) -> PrecursorResult {
  let config = config::read(input.open()?)?;
  let target_dir = get_target_dir(target_dir)?;

  for cursor_config in config.cursors {
    let cursor_name = cursor_config.name.clone();
    let cursor_targets = cursor_config.targets.clone();
    let cursor = Cursor::from_config(cursor_config)?;

    if all || scalable {
      // TODO
    }

    if all || windows {
      let mut cursor_path = target_dir
        .join(get_windows_name(&cursor_targets).unwrap_or(&cursor_name));

      if cursor.is_animated() {
        cursor_path.set_extension("ani");
        AniFile::from_cursor(&cursor)?.write_to(File::create(cursor_path)?)?;
      } else {
        cursor_path.set_extension("cur");
        CurFile::from_cursor(&cursor)?.write_to(File::create(cursor_path)?)?;
      }
    }

    if all || xcursor {
      let cursor_path = target_dir
        .join(get_linux_name(&cursor_targets).unwrap_or(&cursor_name));

      XcursorFile::from_cursor(&cursor)
        .unwrap() // infallible
        .write_to(File::create(cursor_path)?)?;
    }

    // TODO: name aliasing
  }

  Ok(())
}

/// Returns a path to the target directory.
///
/// Falls back to the current working directory if the value is `None`.
fn get_target_dir(target_dir: Option<PathBuf>) -> IoResult<PathBuf> {
  if let Some(value) = target_dir {
    if !fs::metadata(&value)?.is_dir() {
      Err(IoErrorKind::NotADirectory.into())
    } else {
      Ok(value)
    }
  } else {
    env::current_dir()
  }
}

fn get_windows_name(cursor_targets: &Option<CursorTargets>) -> Option<&String> {
  if let Some(targets) = &cursor_targets
    && let Some(windows) = &targets.windows
    && let Some(name) = &windows.name
  {
    Some(name)
  } else {
    None
  }
}

fn get_linux_name(cursor_targets: &Option<CursorTargets>) -> Option<&String> {
  if let Some(targets) = &cursor_targets
    && let Some(linux) = &targets.linux
    && let Some(name) = &linux.name
  {
    Some(name)
  } else {
    None
  }
}
