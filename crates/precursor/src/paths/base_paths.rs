use std::path::{Path, PathBuf};

use crate::args::BuildTargetTypeArgs;

pub fn get_linux_base_path(
  target_dir_path: impl AsRef<Path>,
  BuildTargetTypeArgs {
    all,
    scalable,
    windows,
    xcursor,
  }: BuildTargetTypeArgs,
) -> Option<PathBuf> {
  if all || (windows && (scalable || xcursor)) {
    Some(target_dir_path.as_ref().join("linux"))
  } else if !all && !windows {
    Some(target_dir_path.as_ref().to_path_buf())
  } else {
    None
  }
}

pub fn get_windows_base_path(
  target_dir_path: impl AsRef<Path>,
  BuildTargetTypeArgs {
    all,
    scalable,
    windows,
    xcursor,
  }: BuildTargetTypeArgs,
) -> Option<PathBuf> {
  if all || (windows && (scalable || xcursor)) {
    Some(target_dir_path.as_ref().join("windows"))
  } else if !all && !scalable && !xcursor {
    Some(target_dir_path.as_ref().to_path_buf())
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use std::sync::LazyLock;

  use super::*;

  static TARGET_DIR_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from("test"));

  #[test]
  fn get_linux_only() {
    let target_types = BuildTargetTypeArgs {
      all: false,
      scalable: true,
      windows: false,
      xcursor: true,
    };

    let base_path = get_linux_base_path(TARGET_DIR_PATH.clone(), target_types);

    assert_eq!(base_path, Some(TARGET_DIR_PATH.clone()));
  }

  #[test]
  fn get_linux_forked() {
    let target_types = BuildTargetTypeArgs {
      all: false,
      scalable: true,
      windows: true,
      xcursor: true,
    };

    let base_path = get_linux_base_path(TARGET_DIR_PATH.clone(), target_types);

    assert_eq!(base_path, Some(TARGET_DIR_PATH.join("linux")));
  }

  #[test]
  fn get_linux_all() {
    let target_types = BuildTargetTypeArgs {
      all: true,
      scalable: false,
      windows: false,
      xcursor: false,
    };

    let base_path = get_linux_base_path(TARGET_DIR_PATH.clone(), target_types);

    assert_eq!(base_path, Some(TARGET_DIR_PATH.join("linux")));
  }

  #[test]
  fn get_windows_only() {
    let target_types = BuildTargetTypeArgs {
      all: false,
      scalable: false,
      windows: true,
      xcursor: false,
    };

    let base_path =
      get_windows_base_path(TARGET_DIR_PATH.clone(), target_types);

    assert_eq!(base_path, Some(TARGET_DIR_PATH.clone()));
  }

  #[test]
  fn get_windows_forked() {
    let target_types = BuildTargetTypeArgs {
      all: false,
      scalable: true,
      windows: true,
      xcursor: true,
    };

    let base_path =
      get_windows_base_path(TARGET_DIR_PATH.clone(), target_types);

    assert_eq!(base_path, Some(TARGET_DIR_PATH.join("windows")));
  }

  #[test]
  fn get_windows_all() {
    let target_types = BuildTargetTypeArgs {
      all: true,
      scalable: false,
      windows: false,
      xcursor: false,
    };

    let base_path =
      get_windows_base_path(TARGET_DIR_PATH.clone(), target_types);

    assert_eq!(base_path, Some(TARGET_DIR_PATH.join("windows")));
  }
}
