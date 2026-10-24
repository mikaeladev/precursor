mod args;
mod cmds;

use std::env;
use std::process::exit;

use env_logger::{Builder, Env};
use log::{LevelFilter, debug, error, info};

use crate::args::{Cli, Command};

fn main() {
  let args = args::parse();

  init_logger(&args);

  let result = match args.command {
    Command::Build(build_args) => cmds::build(build_args),
    Command::Check(check_args) => cmds::check(check_args),
    Command::Extract(extract_args) => cmds::extract(extract_args),
    Command::Inspect => todo!(),
  };

  if let Err(err) = result {
    debug!("{err:#?}");
    error!("{err}");
    exit(1);
  };
}

// QUIET > DEBUG > VERBOSE > LOG_LEVEL
fn init_logger(args: &Cli) {
  let env = Env::new()
    .filter_or("LOG_LEVEL", "WARN")
    .write_style("LOG_STYLE");

  let mut builder = Builder::from_env(env);
  builder.format_indent(Some(2));

  if args.quiet || env::var_os("QUIET").is_some() {
    builder.filter_level(LevelFilter::Off);
  } else if args.debug || env::var_os("DEBUG").is_some() {
    builder.filter_level(LevelFilter::Debug);
  } else if args.verbose || env::var_os("VERBOSE").is_some() {
    builder.filter_level(LevelFilter::Info);
  }

  builder.init();

  info!("Logger initialised")
}
