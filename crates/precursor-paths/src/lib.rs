use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

/// The directory that the program is working in.
static WORKING_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the current working directory as a static [`PathBuf`].
///
/// # Panics
///
/// Panics if the current directory does not exist at time of initialisation.
pub fn working_dir() -> &'static PathBuf {
  WORKING_DIR.get_or_init(|| {
    env::current_dir().expect("failed to access working directory")
  })
}

/// The directory to output files to.
static TARGET_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the target directory as a static [`PathBuf`].
///
/// # Panics
///
/// Panics under the same conditions as [`working_dir`].
pub fn target_dir() -> &'static PathBuf {
  TARGET_DIR.get_or_init(|| working_dir().join("out"))
}

/// Initialises the contents of the `TARGET_DIR` cell.
///
/// # Panics
///
/// Panics if the cell was already initialised.
pub fn set_target_dir(value: PathBuf) {
  TARGET_DIR
    .set(working_dir().join(value))
    .expect("TARGET_DIR should be uninitialised")
}

/// Whether or not cross-platform cursors are being built.
static IS_CROSS_PLATFORM: OnceLock<bool> = OnceLock::new();

/// Returns whether or not cross-platform cursors are being built.
pub fn is_cross_platform() -> bool {
  *IS_CROSS_PLATFORM.get_or_init(|| false)
}

/// Initialises the contents of the `IS_CROSS_PLATFORM` cell.
///
/// # Panics
///
/// Panics if the cell was already initialised.
pub fn set_is_cross_platform(value: bool) {
  IS_CROSS_PLATFORM
    .set(value)
    .expect("IS_CROSS_PLATFORM should be uninitialised")
}

/// The target directory for Linux cursors.
static LINUX_TARGET_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the Linux target directory as a static [`PathBuf`].
///
/// # Panics
///
/// Panics under the same conditions as [`target_dir`].
pub fn linux_target_dir() -> &'static PathBuf {
  LINUX_TARGET_DIR.get_or_init(|| {
    let base_dir = target_dir();
    if is_cross_platform() {
      base_dir.join("linux")
    } else {
      base_dir.to_path_buf()
    }
  })
}

/// The target directory for Windows cursors.
static WINDOWS_TARGET_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the Windows target directory as a static [`PathBuf`].
///
/// # Panics
///
/// Panics under the same conditions as [`target_dir`].
pub fn windows_target_dir() -> &'static PathBuf {
  WINDOWS_TARGET_DIR.get_or_init(|| {
    let base_dir = target_dir();
    if is_cross_platform() {
      base_dir.join("windows")
    } else {
      base_dir.to_path_buf()
    }
  })
}

/// The target directory for X11 cursors.
static X11_TARGET_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the X11 cursor directory as a static [`PathBuf`].
///
/// # Panics
///
/// Panics under the same conditions as [`linux_target_dir`].
pub fn x11_cursor_dir() -> &'static PathBuf {
  X11_TARGET_DIR.get_or_init(|| linux_target_dir().join("cursors"))
}

/// The target directory for SVG cursors.
static SVG_TARGET_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Returns the SVG cursor directory as a static [`PathBuf`].
///
/// # Panics
///
/// Panics under the same conditions as [`linux_target_dir`].
pub fn svg_cursor_dir() -> &'static PathBuf {
  SVG_TARGET_DIR.get_or_init(|| linux_target_dir().join("cursors_scalable"))
}
