use std::fs::File;

use crate_config::{Config, CursorTargets};

use crate_formats::cursors::CursorFile;
use crate_formats::cursors::ani::AniFile;
use crate_formats::cursors::cur::CurFile;
use crate_formats::cursors::xcursor::XcursorFile;

use crate::args::BuildArgs;
use crate::cursor::{Cursor, FromCursor};
use crate::error::PrecursorResult;
use crate::filesys;

pub fn build(
  BuildArgs {
    config_file_input,
    target_dir_path,
    force,
    all,
    scalable,
    windows,
    xcursor,
  }: BuildArgs,
) -> PrecursorResult {
  let working_dir_path = filesys::get_working_dir_path()?;

  let config_string = filesys::read_config_string_from_input(
    &working_dir_path,
    config_file_input,
  )?;

  let config: Config = toml::from_str(&config_string)?;

  let target_dir = match target_dir_path {
    None => working_dir_path,
    Some(dir_path) => {
      filesys::check_target_dir_path(&dir_path)?;
      dir_path
    }
  };

  for cursor_config in config.cursors {
    let cursor_name = cursor_config.name.clone();
    let cursor_targets = cursor_config.targets.clone();
    let cursor = Cursor::from_config(cursor_config)?;

    #[cfg(target_family = "unix")]
    let linux_aliases = get_linux_aliases(&cursor_targets);

    if all || scalable {
      // TODO
    }

    if all || windows {
      let mut cursor_path = target_dir
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
      let cursor_path = target_dir
        .join(get_linux_name(&cursor_targets).unwrap_or(&cursor_name));

      filesys::prepare_target_file_path(&cursor_path, force)?;

      XcursorFile::from_cursor(&cursor)
        .unwrap() // infallible
        .write(&mut File::create_new(&cursor_path)?)?;

      #[cfg(target_family = "unix")]
      if let Some(aliases) = linux_aliases {
        use std::os::unix::fs as unix_fs;

        for alias in aliases {
          let alias_path = target_dir.join(alias);

          filesys::prepare_target_file_path(&alias_path, force)?;

          unix_fs::symlink(&cursor_path.file_name().unwrap(), alias_path)?;
        }
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
