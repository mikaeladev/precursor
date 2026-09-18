mod config_file;
mod cursor_file;
mod theme_file;
mod wrappers;

pub use config_file::*;
pub use cursor_file::*;
pub use theme_file::*;
pub use wrappers::*;

#[macro_export]
macro_rules! path_error_msg {
  ($msg:expr, $path:expr) => {
    format!("{}:\n  '{}'", $msg, $path.display())
  };

  (access_denied: $path:expr) => {
    path_error_msg!("insufficient permissions to access the path at", $path)
  };

  (access_failed: $path:expr) => {
    path_error_msg!("failed to access the path at", $path)
  };

  (action_denied: $action:expr, $path:expr) => {
    path_error_msg!(
      format_args!("insufficient permissions to {} at path", $action),
      $path
    )
  };

  (action_failed: $action:expr, $path:expr) => {
    path_error_msg!(format_args!("failed to {} at path", $action), $path)
  };

  (expected_found: $expected:expr, $found:expr, $path:expr) => {
    path_error_msg!(
      format_args!("expected {} but got {} at path", $expected, $found),
      $path
    )
  };

  (invalid_data: $file_type:expr, $path:expr) => {
    path_error_msg!(
      format_args!("expected {} to be valid UTF-8 at path", $file_type),
      $path
    )
  };

  (already_exists: $action:expr, $path:expr) => {
    path_error_msg!(format_args!("{} already exists at path", $action), $path)
  };

  (not_found: $expected:expr, $path:expr) => {
    path_error_msg!(format_args!("{} does not exist at path", $expected), $path)
  };
}
