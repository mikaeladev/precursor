use std::fs::File;
use std::path::Path;

use crate_config::{Config, CursorConfig, CursorTargets};

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

  let target_dir_path = match target_dir_path {
    None => working_dir_path,
    Some(dir_path) => {
      filesys::check_target_dir_path(&dir_path)?;
      dir_path
    }
  };

  for cursor_config in config.cursors {
    build_cursor(BuildCursorArgs {
      cursor_config,
      target_dir_path: target_dir_path.as_path(),
      target_types: target_types.clone(),
      force,
    })?;
  }

  Ok(())
}

struct BuildCursorArgs<'a> {
  cursor_config: CursorConfig,
  target_types: BuildTargetTypeArgs,
  target_dir_path: &'a Path,
  force: bool,
}

fn build_cursor(
  BuildCursorArgs {
    cursor_config,
    target_types,
    target_dir_path,
    force,
  }: BuildCursorArgs,
) -> PrecursorResult {
  let cursor_name = cursor_config.name.clone();
  let cursor_targets = cursor_config.targets.clone();

  #[cfg(target_family = "unix")]
  let cursor_aliases = get_linux_aliases(&cursor_targets);

  let cursor = Cursor::from_config(cursor_config)?;

  let BuildTargetTypeArgs {
    all,
    scalable,
    windows,
    xcursor,
  } = target_types;

  if all || scalable {
    todo!("scalable cursors not yet implemented")
  }

  if all || windows {
    let mut cursor_path = target_dir_path
      .join(get_windows_name(&cursor_targets).unwrap_or(&cursor_name));

    if cursor.is_animated() {
      cursor_path.set_extension("ani");

      filesys::prepare_target_file_path(&cursor_path, force)?;

      AniFile::from_cursor(&cursor)?
        .write(&mut File::create_new(cursor_path)?)?;
    } else {
      cursor_path.set_extension("cur");

      filesys::prepare_target_file_path(&cursor_path, force)?;

      CurFile::from_cursor(&cursor)?
        .write(&mut File::create_new(cursor_path)?)?;
    }
  }

  if all || xcursor {
    let cursor_path = target_dir_path
      .join(get_linux_name(&cursor_targets).unwrap_or(&cursor_name));

    filesys::prepare_target_file_path(&cursor_path, force)?;

    XcursorFile::from_cursor(&cursor)
      .unwrap() // infallible
      .write(&mut File::create_new(&cursor_path)?)?;

    #[cfg(target_family = "unix")]
    if let Some(aliases) = cursor_aliases {
      use std::os::unix::fs as unix_fs;

      for alias in aliases {
        let alias_path = target_dir_path.join(alias);

        filesys::prepare_target_file_path(&alias_path, force)?;

        unix_fs::symlink(&cursor_path.file_name().unwrap(), alias_path)?;
      }
    }
  }

  Ok(())
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

#[cfg(target_family = "unix")]
fn get_linux_aliases(
  cursor_targets: &Option<CursorTargets>,
) -> Option<&Vec<String>> {
  if let Some(targets) = &cursor_targets
    && let Some(linux) = &targets.linux
    && let Some(aliases) = &linux.aliases
    && !aliases.is_empty()
  {
    Some(aliases)
  } else {
    None
  }
}
