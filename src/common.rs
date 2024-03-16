use std::fs;
use std::io;
use std::path::PathBuf;

/// Turns a file into a String input
fn fetch_input_from_file(filename: PathBuf) -> Result<String, std::io::Error> {
  match fs::read_to_string(filename) {
    Ok(file_data) => Ok(file_data),
    Err(error) => Err(error),
  }
}

/// Turns STDIN into a String input
#[mutants::skip] // I will do this later
fn fetch_input_from_stdin() -> Result<String, std::io::Error> {
  match io::read_to_string(io::stdin()) {
    Ok(io_data) => Ok(io_data),
    Err(error) => Err(error),
  }
}

/// Returns a `String` input to use with a test.
///
/// If no argument is provided, the input will be read from STDIN.
///
/// # Arguments
/// * `file_path` - File input to read from.
pub fn fetch_input(file_path: Option<PathBuf>) -> Result<String, std::io::Error> {
  return match file_path {
    Some(filename) => fetch_input_from_file(filename),
    None => fetch_input_from_stdin(),
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::ErrorKind;

  #[test]
  fn fetch_input_from_inexisting_file() {
    let path: Option<PathBuf> = Some("foo.txt".into());
    let error = fetch_input(path.clone()).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::NotFound);
  }
}
