use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum InputArg {
  Path(PathBuf),
  #[cfg(target_family = "unix")]
  Stdin,
}

#[cfg(target_family = "unix")]
impl Default for InputArg {
  fn default() -> Self {
    Self::Stdin
  }
}

impl InputArg {
  /// Attempts to open the input in read-only mode.
  pub fn open(&self) -> io::Result<BufReader<File>> {
    Ok(BufReader::new(match self {
      Self::Path(buf) => File::open(buf)?,
      #[cfg(target_family = "unix")]
      Self::Stdin => {
        use std::io::stdin;
        use std::os::fd::{AsRawFd, FromRawFd};

        let lock = stdin().lock();
        let fd = lock.as_raw_fd();

        unsafe { File::from_raw_fd(fd) }
      }
    }))
  }
}

impl From<OsString> for InputArg {
  fn from(value: OsString) -> Self {
    #[cfg(target_family = "unix")]
    if value == "-" {
      return Self::Stdin;
    }

    Self::Path(PathBuf::from(value))
  }
}
