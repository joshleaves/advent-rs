//! Advent of Code 2015: Day 16: Aunt Sue

use std::collections::HashMap;

type Ticker = HashMap<String, i8>;

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

fn build_ticker_tape() -> Ticker {
  TICKER_TAPE
    .lines()
    .map(|line| {
      let kv = line.split_once(':').unwrap();
      let key = kv.0.to_string();
      let value = kv.1.trim().parse::<i8>().unwrap();
      (key, value)
    })
    .collect::<Ticker>()
}

fn compare_aunts<CmpGt, CmpLt>(ticker: &Ticker, aunt: &str, cmp_gt: &CmpGt, cmp_lt: &CmpLt) -> bool
where
  CmpGt: Fn(i8, i8) -> bool,
  CmpLt: Fn(i8, i8) -> bool,
{
  aunt
    .split(',')
    .map(|pair| pair.split(':').map(&str::trim).collect::<Vec<_>>())
    .all(|chunk| {
      assert_eq!(chunk.len(), 2);
      let ticker_value = ticker.get(chunk[0]).unwrap();
      let aunt_value = chunk[1].parse::<i8>().unwrap();

      match chunk[0] {
        "cats" | "trees" => cmp_gt(*ticker_value, aunt_value),
        "pomeranians" | "goldfish" => cmp_lt(*ticker_value, aunt_value),
        // "samoyeds" | "akitas" | "vizslas" | "cars" | "perfumes" | "children" => {
        _ => *ticker_value == aunt_value,
      }
    })
}

pub fn day_16_v1(input: impl Into<String>) -> u16 {
  let ticker_tape = build_ticker_tape();
  input
    .into()
    .lines()
    .position(|aunt| {
      compare_aunts(
        &ticker_tape,
        aunt.split_once(':').unwrap().1,
        &|ticker_value, aunt_value| ticker_value == aunt_value,
        &|ticker_value, aunt_value| ticker_value == aunt_value,
      )
    })
    .unwrap() as u16
    + 1
}

pub fn day_16_v2(input: impl Into<String>) -> u16 {
  let ticker_tape = build_ticker_tape();
  input
    .into()
    .lines()
    .position(|aunt| {
      compare_aunts(
        &ticker_tape,
        aunt.split_once(':').unwrap().1,
        &|ticker_value, aunt_value| aunt_value > ticker_value,
        &|ticker_value, aunt_value| aunt_value < ticker_value,
      )
    })
    .unwrap() as u16
    + 1
}

solvable!(day_16, day_16_v1, day_16_v2, u16);
