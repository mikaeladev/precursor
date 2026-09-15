use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use crate_config::Config;

use crate_formats::cursors::CursorFile;
use crate_formats::cursors::ani::AniFile;
use crate_formats::cursors::cur::CurFile;
use crate_formats::cursors::xcursor::XcursorFile;

use crate::args::{BuildArgs, BuildTargetTypeArgs};
use crate::cursor::{Cursor, FromCursor};
use crate::error::PrecursorResult;
use crate::filesys;

pub fn build(
  BuildArgs {
    config_file_input,
    target_dir_path,
    target_types,
    force,
  }: BuildArgs,
) -> PrecursorResult {
  let working_dir_path = filesys::get_working_dir_path()?;

  let config_string = filesys::read_config_string_from_input(
    &working_dir_path,
    config_file_input,
  )?;

  let config: Config = toml::from_str(&config_string)?;

  let base_path = match target_dir_path {
    None => working_dir_path,
    Some(dir_path) => {
      filesys::check_target_dir_path(&dir_path)?;
      dir_path
    }
  };

  let (linux_base_path, windows_base_path) =
    get_target_base_paths(target_types, base_path)?;

  for cursor_config in config.cursors {
    let cursor = &Cursor::from_config(cursor_config)?;

    if let Some(base_path) = &linux_base_path {
      let base_path = base_path.as_path();

      let build_cursor_args = BuildCursorArgs {
        cursor,
        base_path,
        force,
      };

      if target_types.all || target_types.scalable {
        build_svg_cursor(build_cursor_args)?;
      }

      if target_types.all || target_types.xcursor {
        build_x11_cursor(build_cursor_args)?;
      }
    }

    if let Some(base_path) = &windows_base_path {
      let base_path = base_path.as_path();

      build_windows_cursor(BuildCursorArgs {
        cursor,
        base_path,
        force,
      })?;
    }
  }

  Ok(())
}

fn get_target_base_paths(
  BuildTargetTypeArgs {
    all,
    scalable,
    windows,
    xcursor,
  }: BuildTargetTypeArgs,
  base_path: PathBuf,
) -> io::Result<(Option<PathBuf>, Option<PathBuf>)> {
  let mut linux_base_path = None;
  let mut windows_base_path = None;

  if !all && !windows {
    linux_base_path = Some(base_path);
  } else if !all && !scalable && !xcursor {
    windows_base_path = Some(base_path);
  } else {
    let linux_path = base_path.join("linux");
    let windows_path = base_path.join("windows");

    filesys::ensure_dir(&linux_path, "linux target")?;
    filesys::ensure_dir(&windows_path, "windows target")?;

    linux_base_path = Some(linux_path);
    windows_base_path = Some(windows_path);
  }

  Ok((linux_base_path, windows_base_path))
}

#[derive(Clone, Copy)]
struct BuildCursorArgs<'a> {
  cursor: &'a Cursor,
  base_path: &'a Path,
  force: bool,
}

fn build_svg_cursor(_: BuildCursorArgs) -> PrecursorResult {
  todo!("scalable cursors not yet implemented")
}

fn build_windows_cursor(
  BuildCursorArgs {
    cursor,
    base_path,
    force,
  }: BuildCursorArgs,
) -> PrecursorResult {
  let cursor_name = cursor
    .metadata
    .get_windows_name()
    .unwrap_or(&cursor.metadata.name);

  let mut cursor_path = base_path.join(cursor_name);

  if cursor.is_animated() {
    cursor_path.set_extension("ani");

    filesys::prepare_cursor_file_path(&cursor_path, force)?;

    AniFile::from_cursor(&cursor)?
      .write(&mut filesys::create_new_file(cursor_path, "cursor")?)?;
  } else {
    cursor_path.set_extension("cur");

    filesys::prepare_cursor_file_path(&cursor_path, force)?;

    CurFile::from_cursor(&cursor)?
      .write(&mut filesys::create_new_file(cursor_path, "cursor")?)?;
  }

  Ok(())
}

fn build_x11_cursor(build_args: BuildCursorArgs) -> PrecursorResult {
  let BuildCursorArgs {
    cursor,
    base_path,
    force,
  } = build_args;

  let cursor_name = cursor
    .metadata
    .get_linux_name()
    .unwrap_or(&cursor.metadata.name);

  let cursor_path = base_path.join(cursor_name);

  filesys::prepare_cursor_file_path(&cursor_path, force)?;

  XcursorFile::from_cursor(&cursor)
    .unwrap() // infallible
    .write(&mut File::create_new(&cursor_path)?)?;

  #[cfg(target_family = "unix")]
  symlink_linux_aliases(build_args)?;

  Ok(())
}

#[cfg(target_family = "unix")]
fn symlink_linux_aliases(
  BuildCursorArgs {
    cursor: Cursor { metadata, .. },
    base_path,
    force,
  }: BuildCursorArgs,
) -> io::Result<()> {
  let cursor_name = metadata.get_linux_name().unwrap_or(&metadata.name);
  let cursor_aliases = metadata.get_linux_aliases();

  if let Some(aliases) = cursor_aliases {
    for alias_name in aliases {
      let alias_path = base_path.join(alias_name);

      filesys::prepare_cursor_file_path(&alias_path, force)?;

      filesys::symlink_cursor_file(base_path, cursor_name, alias_name)?;
    }
  }

  Ok(())
}
