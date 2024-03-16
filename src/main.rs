use advent_rs;
use clap::Parser;
use std::path::PathBuf;

mod common;

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

  /// The part of the exercise, 1 or 2
  #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8))]
  part: u8,

  // File name
  #[arg(value_parser = clap::value_parser!(PathBuf))]
  input: Option<PathBuf>,
}

fn main() {
  let args = Args::parse();
  let input: String = match common::fetch_input(args.input) {
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
