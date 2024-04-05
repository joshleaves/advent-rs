//! Advent of Code 2015: Day 24: It Hangs in the Balance

use itertools::Itertools;

fn solve(numbers: &[u64], magic: u64) -> u64 {
  (2..=numbers.len())
    .find_map(|i| {
      numbers
        .iter()
        .combinations(i)
        .filter(|arr| arr.iter().copied().sum::<u64>() == magic)
        .map(|combo| combo.iter().copied().product::<u64>())
        .min()
    })
    .unwrap()
}

pub fn day_24_v1(input: impl Into<String>) -> u64 {
  let numbers: Vec<u64> = input
    .into()
    .lines()
    .map(|line| line.parse::<u64>().unwrap())
    .collect();
  let magic = numbers.iter().sum::<u64>() / 3;

  solve(&numbers, magic)
}

pub fn day_24_v2(input: impl Into<String>) -> u64 {
  let numbers: Vec<u64> = input
    .into()
    .lines()
    .map(|line| line.parse::<u64>().unwrap())
    .collect();
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
