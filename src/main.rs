use advent_rs;
use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Solver for advent of code exercises
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
  /// Year of the exercise
  #[arg(short, long, value_parser = clap::value_parser!(u16))]
  year: u16,

  /// Day of the exercise
  #[arg(short, long, value_parser = clap::value_parser!(u8))]
  day: u8,

  /// Version of the exercise
  #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8))]
  version: u8,

  // File name
  #[arg(value_parser = clap::value_parser!(PathBuf))]
  input: Option<PathBuf>,
}

fn main() {
  let args = Args::parse();

  let input = match args.input {
    Some(filename) => match fs::read_to_string(filename.clone()) {
      Ok(file_data) => file_data,
      Err(error) => {
        eprintln!("advent-rs: Could not read {filename:#?} ({error:#})");
        std::process::exit(1)
      }
    },
    None => io::read_to_string(io::stdin())
      .expect("Unable to read STDIN")
      .to_string(),
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
