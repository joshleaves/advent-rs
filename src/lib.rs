use std::fs;
use std::io;
use std::path::PathBuf;

pub mod year_2015;

pub fn fetch_input_from_file(filename: PathBuf) -> Result<String, std::io::Error> {
  match fs::read_to_string(filename) {
    Ok(file_data) => Ok(file_data),
    Err(error) => Err(error),
  }
}

pub fn fetch_input_from_stdin() -> Result<String, std::io::Error> {
  match io::read_to_string(io::stdin()) {
    Ok(io_data) => Ok(io_data),
    Err(error) => Err(error),
  }
}

pub fn fetch_input(file_path: Option<PathBuf>) -> Result<String, std::io::Error> {
  return match file_path {
    Some(filename) => fetch_input_from_file(filename),
    None => fetch_input_from_stdin(),
  };
}

/// Returns the solution for a specified exercise and input.
///
/// # Arguments
///
/// * `year` - The year of the exercise, from 2015.
/// * `day` - The day of the exercise, from 1 to 25.
/// * `version` - The version of the exercise, 1 or 2.
/// * `input` - The input to the problem.
///
/// # Example
/// ```
/// use advent_of_code::solve;
/// let solution = solve(2019, 1, 1, "14");
/// assert_eq!(solution, "2"));
/// ```
pub fn solve(year: u16, day: u8, version: u8, input: String) -> String {
  match year {
    2015 => return year_2015::solve(day, version, input),
    _ => {
      eprintln!("advent-rs: Not implemented (Year 2015 Day {day:02}v{version})");
      std::process::exit(1)
    }
  }
}
