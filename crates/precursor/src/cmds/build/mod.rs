mod args;
mod config;

use precursor_error::{ErrorMessage, Result};
use precursor_filesys as filesys;
use precursor_filesys::metapath::AsMetaPath;
use precursor_formats::ani::AniFile;
use precursor_formats::cur::CurFile;
use precursor_formats::xcur::XcursorFile;
use precursor_paths as paths;

pub use args::*;

pub(super) fn debug_help() -> Option<&'static str> {
  if log::max_level() < log::LevelFilter::Debug {
    Some("consider passing the '--debug' flag to see the underlying error")
  } else {
    None
  }
}

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
    let cursor = config::make_cursor(&cursor_config)?;

    if target_types.includes_scalable() {
      todo!("SVG cursors not yet implemented")
    }

    if target_types.includes_windows() {
      if cursor.frames.is_animated() {
        let ani = cursor.to_ani_file().map_err(|src| {
          ErrorMessage::new("failed to construct ANI cursor")
            .with_help::<&str>(debug_help())
            .with_context([("name", &cursor_config.name)])
            .into_error()
            .with_source(src)
        })?;

        write_ani_file(&cursor_config.name, ani, force)?;
      } else {
        let cur = cursor.to_cur_file().map_err(|src| {
          ErrorMessage::new("failed to construct CUR cursor")
            .with_help::<&str>(debug_help())
            .with_context([("name", &cursor_config.name)])
            .into_error()
            .with_source(src)
        })?;

        write_cur_file(&cursor_config.name, cur, force)?;
      }
    }

    if target_types.includes_xcursor() {
      write_xcur_file(&cursor_config.name, cursor.to_xcursor_file(), force)?;

      #[cfg(target_family = "unix")]
      for alias in cursor_config.aliases {
        filesys::symlink_cursor(&cursor_config.name, alias, force)?;
      }
    }
  }

  if target_types.includes_linux() {
    let path = paths::linux_target_dir().join("index.theme");
    let meta_path = path.as_meta_path("linux theme file");

    let mut file = if force {
      filesys::create_file(meta_path)
    } else {
      filesys::create_new_file(meta_path)
    }?;

    // TODO: move this out of filesys
    filesys::write_theme_file(&mut file, config.theme)?;
  }

  Ok(())
}

fn write_ani_file(name: &str, ani: AniFile, force: bool) -> Result<()> {
  let path = paths::windows_target_dir().join(format!("{}.ani", name));
  let meta_path = path.as_meta_path("ANI file");

  let mut file = if force {
    filesys::create_file(meta_path)
  } else {
    filesys::create_new_file(meta_path)
  }?;

  ani.write(&mut file).map_err(|src| {
    ErrorMessage::new("failed to write ANI file")
      .with_subheading(src.kind())
      .with_help::<&str>(debug_help())
      .with_context([
        ("name", name.to_string()),
        ("path", path.display().to_string()),
      ])
      .into_error()
      .with_source(src)
  })?;

  Ok(())
}

fn write_cur_file(name: &str, cur: CurFile, force: bool) -> Result<()> {
  let path = paths::windows_target_dir().join(format!("{}.cur", name));
  let meta_path = path.as_meta_path("CUR file");

  let mut file = if force {
    filesys::create_file(meta_path)
  } else {
    filesys::create_new_file(meta_path)
  }?;

  cur.write(&mut file).map_err(|src| {
    ErrorMessage::new("failed to write CUR file")
      .with_subheading(src.kind())
      .with_help::<&str>(debug_help())
      .with_context([
        ("name", name.to_string()),
        ("path", path.display().to_string()),
      ])
      .into_error()
      .with_source(src)
  })?;

  Ok(())
}

fn write_xcur_file(name: &str, xcur: XcursorFile, force: bool) -> Result<()> {
  let path = paths::x11_cursor_dir().join(name);
  let meta_path = path.as_meta_path("XCUR file");

  let mut file = if force {
    filesys::create_file(meta_path)
  } else {
    filesys::create_new_file(meta_path)
  }?;

  xcur.write(&mut file).map_err(|src| {
    ErrorMessage::new("failed to write XCUR file")
      .with_subheading(src.kind())
      .with_help::<&str>(debug_help())
      .with_context([
        ("name", name.to_string()),
        ("path", path.display().to_string()),
      ])
      .into_error()
      .with_source(src)
  })?;

  Ok(())
}
