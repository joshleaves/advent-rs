#![doc = include_str!("../README.md")]

use std::fs;
use std::io;
use std::path::PathBuf;

macro_rules! solvable {
  ($day_xx:tt, $day_v1:tt, $day_v2:tt, $retype:ty) => {
    /// Stub function calling the _v1 or _v2 variant
    pub fn $day_xx(part: u8, input: impl Into<String>) -> $retype {
      match part {
        1 => $day_v1(input),
        2 => $day_v2(input),
        _ => {
          panic!("Invalid part: {}", part)
        }
      }
    }
  };
}

pub mod year_2015;

/// Turns a file into a String input
pub fn fetch_input_from_file(filename: PathBuf) -> Result<String, std::io::Error> {
  match fs::read_to_string(filename) {
    Ok(file_data) => Ok(file_data),
    Err(error) => Err(error),
  }
}

/// Turns STDIN into a String input
#[mutants::skip] // I will do this later
pub fn fetch_input_from_stdin() -> Result<String, std::io::Error> {
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

/// Returns the solution for a specified exercise and input.
///
/// # Arguments
/// * `year` - The year of the exercise, from 2015.
/// * `day` - The day of the exercise, from 1 to 25.
/// * `part` - The part of the exercise, 1 or 2.
/// * `input` - The input to the exercise.
///
/// # Examples
/// Missing year/day/part exercise
/// ```
/// use advent_rs::solve;
/// assert_eq!(solve(2014, 1, 1, ""), None);
/// assert_eq!(solve(2015, 1, 3, ""), None);
/// assert_eq!(solve(2015, 26, 1, ""), None);
/// ```
///
/// Available year/day/part exercise returns a String
/// ```
/// use advent_rs::solve;
/// let solution = solve(2015, 1, 1, "(())");
/// assert_eq!(solution, Some("0".to_string()));
/// ```
pub fn solve(year: u16, day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  match year {
    2015 => return year_2015::solve(day, part, input),
    _ => return None,
  }
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

  #[test]
  fn solve_for_invalid_year() {
    assert_eq!(solve(2014, 1, 1, "".to_string()), None);
  }

  #[test]
  fn solve_for_year_2015_day_01v1() {
    assert_eq!(solve(2015, 1, 1, "(())".to_string()), Some("0".to_string()));
  }
}
