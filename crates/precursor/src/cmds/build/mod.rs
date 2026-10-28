mod args;

use std::path::Path;

use precursor_cursor::Cursor;
use precursor_error::{ErrorMessage, Result};
use precursor_filesys as filesys;
use precursor_filesys::metapath::AsMetaPath;
use precursor_paths as paths;

pub use args::*;

const CONSIDER_DEBUG_FLAG: &str =
  "consider passing the '--debug' flag to see the underlying error";

pub fn build(
  BuildArgs {
    config_file_input,
    target_dir_path,
    target_types,
    empty,
    force,
  }: BuildArgs,
) -> Result<()> {
  paths::set_target_dir(target_dir_path);
  paths::set_is_cross_platform(target_types.is_cross_platform());

  let target_dir_meta_path =
    paths::target_dir().as_meta_path("target directory");

  if !filesys::ensure_dir(target_dir_meta_path)? && empty {
    filesys::empty_dir(target_dir_meta_path)?;
  }

  if paths::is_cross_platform() {
    filesys::ensure_dir(
      paths::linux_target_dir().as_meta_path("linux target directory"),
    )?;
    filesys::ensure_dir(
      paths::windows_target_dir().as_meta_path("windows target directory"),
    )?;
  }

  if target_types.includes_scalable() {
    filesys::ensure_dir(
      paths::svg_cursor_dir().as_meta_path("SVG cursor directory"),
    )?;
  }

  if target_types.includes_xcursor() {
    filesys::ensure_dir(
      paths::x11_cursor_dir().as_meta_path("X11 cursor directory"),
    )?;
  }

  let config = filesys::get_config(config_file_input)?;

  for cursor_config in config.cursors {
    let cursor = Cursor::from_config(cursor_config)?;

    if target_types.includes_scalable() {
      todo!("SVG cursors not yet implemented")
    }

    if target_types.includes_xcursor() {
      build_x11_cursor(&cursor, force)?;
    }

    if target_types.includes_windows() {
      build_windows_cursor(&cursor, force)?;
    }
  }

  if target_types.includes_linux() {
    let path = paths::linux_target_dir().join("index.theme");
    let meta_path = path.as_meta_path("linux theme index");

    let mut file = if force {
      filesys::create_file(meta_path)
    } else {
      filesys::create_new_file(meta_path)
    }?;

    // TODO: move this out of filesys
    filesys::write_theme_index(&mut file, config.package)?;
  }

  Ok(())
}

fn make_help() -> Option<&'static str> {
  if log::max_level() < log::LevelFilter::Debug {
    Some(CONSIDER_DEBUG_FLAG)
  } else {
    None
  }
}

fn make_context(
  cursor: &Cursor,
  file_path: impl AsRef<Path>,
) -> [(&str, String); 2] {
  [
    ("name", cursor.metadata().name.to_string()),
    ("path", file_path.as_ref().display().to_string()),
  ]
}

fn build_windows_cursor(cursor: &Cursor, force: bool) -> Result<()> {
  let cursor_metadata = cursor.metadata();
  let cursor_name = cursor_metadata.windows_name();

  let mut path = paths::windows_target_dir().join(cursor_name);
  path.set_extension(if cursor.is_animated() { "ani" } else { "cur" });

  let meta_path = path.as_meta_path("ANI cursor");

  let mut file = if force {
    filesys::create_file(meta_path)
  } else {
    filesys::create_new_file(meta_path)
  }?;

  let make_context = || make_context(cursor, &path);

  if cursor.is_animated() {
    let ani = cursor.to_windows_cur().map_err(|src| {
      ErrorMessage::new("failed to construct ANI cursor")
        .with_subheading(src.to_string())
        .with_help::<&str>(make_help())
        .with_context(make_context())
        .into_error()
        .with_source(src)
    })?;

    ani.write(&mut file).map_err(|src| {
      ErrorMessage::new("failed to write ANI cursor")
        .with_subheading(src.kind())
        .with_help::<&str>(make_help())
        .with_context(make_context())
        .into_error()
        .with_source(src)
    })?;
  } else {
    let cur = cursor.to_windows_cur().map_err(|src| {
      ErrorMessage::new("failed to construct CUR cursor")
        .with_subheading(src.to_string())
        .with_help::<&str>(make_help())
        .with_context(make_context())
        .into_error()
        .with_source(src)
    })?;

    cur.write(&mut file).map_err(|src| {
      ErrorMessage::new("failed to write CUR cursor")
        .with_subheading(src.kind())
        .with_help::<&str>(make_help())
        .with_context(make_context())
        .into_error()
        .with_source(src)
    })?;
  }

  Ok(())
}

fn build_x11_cursor(cursor: &Cursor, force: bool) -> Result<()> {
  let cursor_metadata = cursor.metadata();
  let cursor_name = cursor_metadata.linux_name();

  let path = paths::x11_cursor_dir().join(cursor_name);
  let meta_path = path.as_meta_path("X11 cursor");

  let mut file = if force {
    filesys::create_file(meta_path)
  } else {
    filesys::create_new_file(meta_path)
  }?;

  cursor.to_xcursor().write(&mut file).map_err(|src| {
    ErrorMessage::new("failed to write X11 cursor")
      .with_subheading(src.kind())
      .with_help::<&str>(make_help())
      .with_context(make_context(cursor, &path))
      .into_error()
      .with_source(src)
  })?;

  #[cfg(target_family = "unix")]
  symlink_linux_aliases(paths::x11_cursor_dir(), cursor, force)?;

  Ok(())
}

#[cfg(target_family = "unix")]
fn symlink_linux_aliases(
  base_path: impl AsRef<Path>,
  cursor: &Cursor,
  force: bool,
) -> Result<()> {
  let cursor_metadata = cursor.metadata();
  let cursor_name = cursor_metadata.linux_name();
  let cursor_aliases = cursor_metadata.linux_aliases();

  if !cursor_aliases.is_empty() {
    for alias_name in cursor_aliases {
      filesys::symlink_cursor(&base_path, cursor_name, alias_name, force)?;
    }
  }

  Ok(())
}
