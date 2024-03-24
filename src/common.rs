use std::fs;
use std::io;
use std::path::Path;

/// Returns a `String` input to use with a test.
///
/// If no argument is provided, the input will be read from STDIN.
///
/// # Arguments
/// * `file_path` - File input to read from.
pub fn fetch_input(file_path: Option<impl AsRef<Path>>) -> Result<String, std::io::Error> {
  match file_path {
    Some(filename) => fs::read_to_string(filename),
    None => io::read_to_string(io::stdin()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::ErrorKind;
  use std::path::PathBuf;

  #[test]
  fn fetch_input_from_inexisting_file() {
    let path: Option<PathBuf> = Some("foo.txt".into());
    let error = fetch_input(path).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::NotFound);
  }
}
