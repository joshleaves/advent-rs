use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

/// Solver for advent of code exercises
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The year of the exercise, from 2015 to today
  #[arg(short, long, value_parser = clap::value_parser!(u16))]
  year: u16,

  /// The day of the exercise, from 1 to 25
  #[arg(short, long, value_parser = clap::value_parser!(u8))]
  day: u8,

  /// The part of the exercise, 1 or 2
  #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8))]
  part: u8,

  // File name
  #[arg(help = "Input file path (will read from STDIN if empty)", value_parser = clap::value_parser!(PathBuf))]
  input: Option<PathBuf>,
}

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

/// Software entry point
///
/// The arguments are taken from the command line inputs, parsed by Clap.
///
/// Arguments
/// * `year` - The year of the exercise, from 2015 to today.
/// * `day` - The day of the exercise, from 1 to 25.
/// * `part` -  The part of the exercise, 1 or 2.
/// * `input` - Input file path (will read from STDIN if empty).
fn main() {
  let args = Args::parse();
  let input: String = match fetch_input(args.input) {
    Ok(input_data) => input_data,
    Err(error) => {
      eprintln!("advent-rs: {}", error);
      std::process::exit(1);
    }
  };
  match advent_rs::solve(args.year, args.day, args.part, input) {
    Some(result) => println!("Result: {}", result),
    None => {
      eprintln!(
        "advent-rs: Not implemented (Year {} Day {:02}v{})",
        args.year, args.day, args.part
      );
      std::process::exit(1)
    }
  };
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
