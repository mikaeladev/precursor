use std::io;
use std::path::Path;

use crate::args::BuildArgs;
use crate::cursor::Cursor;
use crate::error::PrecursorResult;
use crate::filesys;
use crate::paths;

pub fn build(
  BuildArgs {
    config_file_input,
    target_dir_path,
    target_types,
    empty,
    force,
  }: BuildArgs,
) -> PrecursorResult {
  let working_dir_path = paths::get_working_dir_path()?;

  let target_dir_path = working_dir_path.join(target_dir_path);

  if empty {
    filesys::ensure_dir_empty(&target_dir_path, "target")?;
  } else {
    filesys::ensure_dir(&target_dir_path, "target")?;
  }

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
    let file_kind = "linux theme index";

    let mut file = if force {
      filesys::create_file(file_path, file_kind)
    } else {
      filesys::create_new_file(file_path, file_kind)
    }?;

    filesys::write_icon_theme_index(&mut file, config.package)?;
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
  let cursor_metadata = cursor.metadata();
  let cursor_name = cursor_metadata.windows_name();

  let mut file_path = base_path.join(cursor_name);
  file_path.set_extension(if cursor.is_animated() { "ani" } else { "cur" });

  let mut file = if force {
    filesys::create_file(file_path, "cursor")
  } else {
    filesys::create_new_file(file_path, "cursor")
  }?;

  if cursor.is_animated() {
    cursor.to_windows_ani()?.write(&mut file)?;
  } else {
    cursor.to_windows_cur()?.write(&mut file)?;
  }

  Ok(())
}

fn build_x11_cursor(build_args: BuildCursorArgs) -> PrecursorResult {
  let BuildCursorArgs {
    cursor,
    base_path,
    force,
  } = build_args;
  let cursor_metadata = cursor.metadata();
  let cursor_name = cursor_metadata.linux_name();

  let file_path = base_path.join(cursor_name);

  let mut file = if force {
    filesys::create_file(file_path, "cursor")
  } else {
    filesys::create_new_file(file_path, "cursor")
  }?;

  cursor.to_xcursor().write(&mut file)?;

  #[cfg(target_family = "unix")]
  symlink_linux_aliases(build_args)?;

  Ok(())
}

#[cfg(target_family = "unix")]
fn symlink_linux_aliases(
  BuildCursorArgs {
    cursor,
    base_path,
    force,
  }: BuildCursorArgs,
) -> io::Result<()> {
  let cursor_metadata = cursor.metadata();
  let cursor_name = cursor_metadata.linux_name();
  let cursor_aliases = cursor_metadata.linux_aliases();

  if !cursor_aliases.is_empty() {
    for alias_name in cursor_aliases {
      filesys::symlink_cursor(base_path, cursor_name, alias_name, force)?;
    }
  }

  Ok(())
}
