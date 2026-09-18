mod args;
mod cmds;
mod cursor;
mod error;
mod filesys;
mod paths;

use std::env;
use std::process::exit;
use std::sync::OnceLock;

use crate::args::Command;

pub static DEBUG: OnceLock<bool> = OnceLock::new();

fn main() {
  let args = args::parse();

  if args.debug || env::var_os("DEBUG").is_some() {
    DEBUG.set(true).unwrap();
  }

  let result = match args.command {
    Command::Build(build_args) => cmds::build(build_args),
    Command::Check(check_args) => cmds::check(check_args),
    Command::Extract(_) => todo!(),
    Command::Inspect(_) => todo!(),
  };

  if let Err(err) = result {
    eprintln!("an error occurred: {err}");
    exit(1);
  };
}

#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {
    if crate::DEBUG.get().and_then(|v| Some(*v)).unwrap_or(false) {
      eprintln!("[DEBUG]: {}", format_args!($($arg)*))
    }
  };
}
