mod config_file;
mod entity_exists;
mod target_dir;
mod target_file;
mod working_dir;

pub use config_file::*;
pub use entity_exists::*;
pub use target_dir::*;
pub use target_file::*;
pub use working_dir::*;

#[macro_export]
macro_rules! path_error_msg {
  ($msg:expr, $path:expr) => {
    format!("{}:\n  '{}'", $msg, $path.display())
  };

  (denied_action: $action:expr, $path:expr) => {
    path_error_msg!(
      format_args!("insufficient permissions to {} at path", $action),
      $path
    )
  };

  (expected: $expected:expr, found: $found:expr, $path:expr) => {
    path_error_msg!(
      format_args!("expected a {} but got a {} at path", $expected, $found),
      $path
    )
  };

  (failed_action: $action:expr, $path:expr) => {
    path_error_msg!(format_args!("failed to {} at path", $action), $path)
  };

  (invalid_data: $file_type:expr, $path:expr) => {
    path_error_msg!(
      format_args!("expected {} to be valid UTF-8 at path", $file_type),
      $path
    )
  };

  (not_found: $expected:expr, $path:expr) => {
    path_error_msg!(format_args!("{} does not exist at path", $expected), $path)
  };
}
