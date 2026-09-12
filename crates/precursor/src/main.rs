mod args;
mod cmds;
mod cursor;
mod error;
mod filesys;

use std::process::exit;

use crate::args::Command;

fn main() {
  let args = args::parse();

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
