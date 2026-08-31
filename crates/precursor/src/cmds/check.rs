use crate::args::CheckArgs;
use crate::config;
use crate::error::PrecursorResult;

pub fn check(CheckArgs { input }: CheckArgs) -> PrecursorResult {
  let config = config::read(input.open()?);

  match config {
    Ok(_) => println!("Success!"),
    Err(err) => eprintln!("Error parsing config: {err}"),
  }

  Ok(())
}
