use advent_rs;
use clap::Parser;
use std::path::PathBuf;

/// Solver for advent of code exercises
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
  /// The year of the exercise, from 2015
  #[arg(short, long, value_parser = clap::value_parser!(u16))]
  year: u16,

  /// The day of the exercise, from 1 to 25
  #[arg(short, long, value_parser = clap::value_parser!(u8))]
  day: u8,

  /// The version of the exercise, 1 or 2
  #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8))]
  version: u8,

  // File name
  #[arg(value_parser = clap::value_parser!(PathBuf))]
  input: Option<PathBuf>,
}

fn main() {
  let args = Args::parse();
  let input: String = match advent_rs::fetch_input(args.input) {
    Ok(input_data) => input_data,
    Err(error) => {
      eprintln!("advent-rs: {}", error);
      std::process::exit(1);
    }
  };

  match args.year {
    2015 => println!(
      "Result: {}",
      advent_rs::year_2015::solve(args.day, args.version, input)
    ),
    _ => {
      eprintln!(
        "advent-rs: Not implemented (Year {} Day {:02}v{})",
        args.year, args.day, args.version
      );
      std::process::exit(1)
    }
  };
}

// fn fetch_input(file_path: Option<PathBuf>) -> Result<String, std::io::Error> {
//   return match file_path {
//     Some(filename) => match fs::read_to_string(filename.clone()) {
//       Ok(file_data) => Ok(file_data),
//       Err(error) => Err(error),
//     },
//     None => match io::read_to_string(io::stdin()) {
//       Ok(io_data) => Ok(io_data),
//       Err(error) => Err(error),
//     },
//   };
// }

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::ErrorKind;

  #[test]
  fn fetch_input_from_inexisting_file() {
    let path: Option<PathBuf> = Some("foo.txt".into());
    let error = advent_rs::fetch_input(path.clone()).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::NotFound);
  }
}
