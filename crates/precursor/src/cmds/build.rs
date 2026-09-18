use std::io;
use std::path::Path;

use crate_formats::cursors::CursorFile;
use crate_formats::cursors::ani::AniFile;
use crate_formats::cursors::cur::CurFile;
use crate_formats::cursors::xcursor::XcursorFile;

use crate::args::BuildArgs;
use crate::cursor::{Cursor, FromCursor};
use crate::error::PrecursorResult;
use crate::filesys;
use crate::paths;

pub fn build(
  BuildArgs {
    config_file_input,
    target_dir_path,
    target_types,
    force,
  }: BuildArgs,
) -> PrecursorResult {
  let working_dir_path = paths::get_working_dir_path()?;

  let target_dir_path = working_dir_path.join(target_dir_path);
  filesys::ensure_dir(&target_dir_path, "target")?;

  let linux_base_path =
    paths::get_linux_base_path(&target_dir_path, target_types);

  let windows_base_path =
    paths::get_windows_base_path(target_dir_path, target_types);

  let mut linux_base_path_svg = None;
  let mut linux_base_path_x11 = None;

  if let Some(base_path) = &linux_base_path {
    filesys::ensure_dir(base_path, "linux target")?;

    if target_types.all || target_types.scalable {
      let svg_base_path = base_path.join("cursors_scalable");

      filesys::ensure_dir(&svg_base_path, "scalable linux cursors")?;
      linux_base_path_svg = Some(svg_base_path);
    }

    if target_types.all || target_types.xcursor {
      let x11_base_path = base_path.join("cursors");

      filesys::ensure_dir(&x11_base_path, "x11 linux cursors")?;
      linux_base_path_x11 = Some(x11_base_path);
    }
  }

  if let Some(base_path) = &windows_base_path {
    filesys::ensure_dir(base_path, "windows target")?;
  }

  let config = filesys::get_config(working_dir_path, config_file_input)?;

  for cursor_config in config.cursors {
    let cursor = &Cursor::from_config(cursor_config)?;

    if let Some(base_path) = &linux_base_path_svg {
      build_svg_cursor(BuildCursorArgs {
        cursor,
        base_path,
        force,
      })?;
    }

    if let Some(base_path) = &linux_base_path_x11 {
      build_x11_cursor(BuildCursorArgs {
        cursor,
        base_path,
        force,
      })?;
    }

    if let Some(base_path) = &windows_base_path {
      build_windows_cursor(BuildCursorArgs {
        cursor,
        base_path,
        force,
      })?;
    }
  }

  if let Some(base_path) = linux_base_path {
    let file_path = base_path.join("index.theme");

    filesys::prepare_file(&file_path, "linux theme index", force)?;

    filesys::write_icon_theme_index(
      &mut filesys::create_new_file(file_path, "linux theme index")?,
      config.package,
    )?;
  }

  Ok(())
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

    filesys::prepare_cursor_file(&cursor_path, force)?;

    AniFile::from_cursor(&cursor)?
      .write(&mut filesys::create_new_file(cursor_path, "cursor")?)?;
  } else {
    cursor_path.set_extension("cur");

    filesys::prepare_cursor_file(&cursor_path, force)?;

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

  filesys::prepare_cursor_file(&cursor_path, force)?;

  XcursorFile::from_cursor(&cursor)
    .unwrap() // infallible
    .write(&mut filesys::create_new_file(cursor_path, "cursor")?)?;

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

      filesys::prepare_cursor_file(&alias_path, force)?;

      filesys::symlink_cursor_file(base_path, cursor_name, alias_name)?;
    }
  }

  Ok(())
}
