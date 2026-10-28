use std::ops::Deref;
use std::path::Path;

/// A wrapper around [`Path`] that provides context as to its usage.
///
/// Consumers of this type can use its metadata to construct more-descriptive
/// error messages and logs.
///
/// # Examples
///
/// A manually constructed `MetaPath` looks like this:
///
/// ```
/// # use std::path::PathBuf;
/// #
/// # use precursor_filesys::metapath::MetaPath;
/// #
/// MetaPath {
///   path: &PathBuf::from("./precursor.toml"),
///   kind: "config file",
/// };
/// ```
///
/// Alternatively, the [`AsMetaPath`] trait may be used to convert from an
/// existing path:
///
/// ```
/// # use std::path::PathBuf;
/// #
/// # use precursor_filesys::metapath::{AsMetaPath, MetaPath};
/// #
/// PathBuf::from("./precursor.toml").as_meta_path("config file");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaPath<'a> {
  pub path: &'a Path,
  pub kind: &'a str,
}

pub trait AsMetaPath {
  /// Converts this type into a [`MetaPath`].
  fn as_meta_path<'a>(&'a self, kind: &'a str) -> MetaPath<'a>;
}

impl<T: Sized + Deref<Target = Path>> AsMetaPath for T {
  fn as_meta_path<'a>(&'a self, kind: &'a str) -> MetaPath<'a> {
    MetaPath { path: self, kind }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::path::PathBuf;

  #[test]
  fn as_meta_path() {
    const PATH: &str = "./foo/bar";
    const KIND: &str = "foobar";

    assert_eq!(
      PathBuf::from(PATH).as_meta_path(KIND),
      MetaPath {
        path: &PathBuf::from(PATH),
        kind: KIND
      }
    )
  }
}
