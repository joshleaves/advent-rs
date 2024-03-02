//! Advent of Code 2015: Day 16: Aunt Sue

use std::collections::HashMap;

const TICKER_TAPE: &str = "children: 3\n\
  cats: 7\n\
  samoyeds: 2\n\
  pomeranians: 3\n\
  akitas: 0\n\
  vizslas: 0\n\
  goldfish: 5\n\
  trees: 3\n\
  cars: 2\n\
  perfumes: 1";

fn build_ticker_tape() -> HashMap<String, i8> {
  let mut data: HashMap<String, i8> = HashMap::<String, i8>::new();
  for line in TICKER_TAPE.lines() {
    let parts: Vec<_> = line.split(": ").collect();
    let key = parts[0].to_string();
    let Ok(value) = parts[1].parse::<i8>() else {
      panic!("Invalid ticker: {}", line)
    };
    data.insert(key, value);
  }
  data
}

fn compare_aunts<CmpGt, CmpLt>(
  ticker_tape: &HashMap<String, i8>,
  aunt: &str,
  cats: &CmpGt,
  pomeranians: &CmpLt,
) -> bool
where
  CmpGt: Fn(i8, i8) -> bool,
  CmpLt: Fn(i8, i8) -> bool,
{
  let mut parts: Vec<_> = aunt.split(": ").collect();
  parts.remove(0);
  parts = parts.iter().flat_map(|part| part.split(", ")).collect();
  let mut idx = 0;
  while idx < parts.len() {
    let Some(ticker_value) = ticker_tape.get(parts[idx]) else {
      panic!("Invalid part: {:?}", parts[idx]);
    };
    idx += 1;
    let Ok(aunt_value) = parts[idx].parse::<i8>() else {
      panic!("Invalid value: {}", parts[idx])
    };
    match parts[idx - 1] {
      "cats" | "trees" => {
        if !cats(*ticker_value, aunt_value) {
          return false;
        }
      }
      "pomeranians" | "goldfish" => {
        if !pomeranians(*ticker_value, aunt_value) {
          return false;
        }
      }
      "samoyeds" | "akitas" | "vizslas" | "cars" | "perfumes" | "children" => {
        if aunt_value != *ticker_value {
          return false;
        }
      }
      _ => {
        panic!("Invalid aunt: {}", parts[idx - 1]);
      }
    }
    idx += 1;
  }

  true
}

pub fn day_16_v1(input: impl Into<String>) -> usize {
  let ticker_tape = build_ticker_tape();
  let mut index = 0;
  for aunt in input.into().lines() {
    index += 1;
    if compare_aunts(
      &ticker_tape,
      &aunt,
      &|ticker_value, aunt_value| ticker_value == aunt_value,
      &|ticker_value, aunt_value| ticker_value == aunt_value,
    ) {
      return index;
    }
  }
  index
}

pub fn day_16_v2(input: impl Into<String>) -> usize {
  let ticker_tape = build_ticker_tape();
  let mut index = 0;
  for aunt in input.into().lines() {
    index += 1;
    if compare_aunts(
      &ticker_tape,
      aunt,
      &|ticker_value, aunt_value| aunt_value > ticker_value,
      &|ticker_value, aunt_value| aunt_value < ticker_value,
    ) {
      return index;
    }
  }
  index
}

solvable!(day_16, day_16_v1, day_16_v2, usize);
