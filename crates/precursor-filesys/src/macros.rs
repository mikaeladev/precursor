macro_rules! action_denied {
  ($action:expr) => {
    format!("insufficient permissions to {}", $action)
  };
}

pub(crate) use action_denied;

macro_rules! action_failed {
  ($action:expr) => {
    format!("failed to {}", $action)
  };

  ($action:expr, $reason:expr) => {
    format!("failed to {}; {}", $action, $reason)
  };
}

pub(crate) use action_failed;

macro_rules! expected_utf8 {
  ($thing:expr) => {
    format!("expected {} to be valid UTF-8", $thing)
  };
}

pub(crate) use expected_utf8;
