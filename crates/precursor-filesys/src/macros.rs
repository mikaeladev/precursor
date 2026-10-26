macro_rules! action_denied {
  ($action:expr) => {
    ErrorMessage::new(format!("insufficient permissions to {}", $action))
  };
}

pub(crate) use action_denied;

macro_rules! action_failed {
  ($action:expr) => {
    ErrorMessage::new(format!("failed to {}", $action))
  };

  ($action:expr, $reason:expr) => {
    ErrorMessage::new(format!("failed to {}", $action)).with_subheading($reason)
  };
}

pub(crate) use action_failed;

macro_rules! expected_utf8 {
  ($thing:expr) => {
    ErrorMessage::new(format!("expected {} to be valid UTF-8", $thing))
  };
}

pub(crate) use expected_utf8;
