//! Advent of Code 2015: Day 25: Let It Snow

use itertools::Itertools;

const STARTER: u64 = 20_151_125;
const MULT: u64 = 252_533;
const MODULO: u64 = 33_554_393;

fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
  if modulus == 1 {
    return 0;
  }
  let mut result: u64 = 1;
  base %= modulus;
  while exp > 0 {
    if exp % 2 == 1 {
      result = result * base % modulus;
    }
    exp >>= 1;
    base = base * base % modulus
  }
  result
}

fn find_iterations_count(col: u64, row: u64) -> u64 {
  ((((col + row - 1) * (col + row)) / 2) - row) + 1
}

fn parse_input(input: &str) -> (u64, u64) {
  input
    .lines()
    .next()
    .unwrap()
    .split(',')
    .map(|p| p.parse::<u64>().unwrap())
    .collect_tuple()
    .expect("Wrong input")
}

pub fn day_25(input: impl Into<String>) -> u64 {
  let (column, row) = parse_input(&input.into());
  let itr = find_iterations_count(column, row) - 1;
  let exp = mod_exp(MULT, itr, MODULO);

  (STARTER * exp) % MODULO
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples() {
    let sample: [[u64; 6]; 6] = [
      [
        20_151_125,
        18_749_137,
        17_289_845,
        30_943_339,
        10_071_777,
        33_511_524,
      ],
      [
        31_916_031,
        21_629_792,
        16_929_656,
        7_726_640,
        15_514_188,
        4_041_754,
      ],
      [
        16_080_970,
        8_057_251,
        1_601_130,
        7_981_243,
        11_661_866,
        16_474_243,
      ],
      [
        24_592_653,
        32_451_966,
        21_345_942,
        9_380_097,
        10_600_672,
        31_527_494,
      ],
      [
        77_061,
        17_552_253,
        28_094_349,
        6_899_651,
        9_250_759,
        31_663_883,
      ],
      [
        33_071_741,
        6_796_745,
        25_397_450,
        24_659_492,
        1_534_922,
        27_995_004,
      ],
    ];
    for (i, numbers) in sample.iter().enumerate() {
      for (j, number) in numbers.iter().enumerate() {
        assert_eq!(day_25(format!("{},{}", j + 1, i + 1)), *number);
      }
    }
  }
}
