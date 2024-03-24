//! Advent of Code 2015: Day 24: It Hangs in the Balance

use itertools::Itertools;

fn solve(numbers: &[u64], magic: u64) -> u64 {
  let mut combos: Vec<Vec<&u64>> = vec![];
  for i in 1..=numbers.len() {
    let mut results: Vec<Vec<&u64>> = numbers.iter().combinations(i).collect_vec();
    results.retain(|arr| arr.iter().copied().sum::<u64>() == magic);
    if !results.is_empty() {
      combos = results;
      break;
    }
  }
  let combos_values: Vec<u64> = combos
    .iter()
    .map(|combo| combo.iter().fold(1, |acc, elt| acc * *elt))
    .collect_vec();

  *combos_values.iter().min().unwrap()
}

fn parse_input(input: &str) -> Vec<u64> {
  let mut numbers: Vec<u64> = vec![];
  for line in input.lines() {
    let Ok(number) = line.parse::<u64>() else {
      panic!("Invalid input: {}", line)
    };
    numbers.push(number);
  }

  numbers
}

pub fn day_24_v1(input: impl Into<String>) -> u64 {
  let numbers = parse_input(&input.into());
  let magic = numbers.iter().sum::<u64>() / 3;

  solve(&numbers, magic)
}

pub fn day_24_v2(input: impl Into<String>) -> u64 {
  let numbers = parse_input(&input.into());
  let magic = numbers.iter().sum::<u64>() / 4;

  solve(&numbers, magic)
}

solvable!(day_24, day_24_v1, day_24_v2, u64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";
    assert_eq!(day_24_v1(sample_one), 99);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_one = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";
    assert_eq!(day_24_v2(sample_one), 44);
  }
}
