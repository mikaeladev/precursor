mod args;
mod asset;
mod cmds;
mod config;
mod cursor;
mod error;

use crate::args::Command;
use crate::error::PrecursorResult;

fn main() -> PrecursorResult {
  let args = args::parse();

  use Command::*;
  match args.command {
    Build(build_args) => cmds::build(build_args),
    Check(check_args) => cmds::check(check_args),
    Extract(_) => todo!(),
    Inspect(_) => todo!(),
  }?;

  Ok(())
}
