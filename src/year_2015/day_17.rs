//! Advent of Code 2015: Day 17: No Such Thing as Too Much

use itertools::Itertools;

const EGGNOG_LITERS: u16 = 150;

fn build_solutions_v1(liters: u16, containers: &[u16]) -> u16 {
  let mut answers = vec![0; (liters + 1) as usize];
  answers[0] = 1;
  for container_size in containers {
    for next_size in (*container_size..=liters).rev() {
      answers[next_size as usize] += answers[(next_size - container_size) as usize];
    }
  }
  answers[liters as usize]
}

fn build_solutions_v2(liters: u16, containers: &[u16]) -> u16 {
  let mut answers = 0;
  let mut sol_len = 2;
  while answers == 0 && sol_len < containers.len() {
    answers = containers
      .iter()
      .combinations(sol_len)
      .filter(|combo| combo.iter().map(|c| **c).sum::<u16>() == liters)
      .count() as u16;
    sol_len += 1;
  }
  answers
}

/// Solve exercise for year 2015, day 17 (part 1).
pub fn day_17_v1(input: impl Into<String>) -> u16 {
  let containers: Vec<u16> = input
    .into()
    .lines()
    .map(|line| line.parse::<u16>().unwrap())
    // .sorted()
    .collect();
  build_solutions_v1(EGGNOG_LITERS, &containers)
}

/// Solve exercise for year 2015, day 17 (part 2).
pub fn day_17_v2(input: impl Into<String>) -> u16 {
  let containers: Vec<u16> = input
    .into()
    .lines()
    .map(|line| line.parse::<u16>().unwrap())
    .collect();
  build_solutions_v2(EGGNOG_LITERS, &containers)
}

solvable!(day_17, day_17_v1, day_17_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample: Vec<u16> = vec![5, 5, 10, 15, 20];
    assert_eq!(build_solutions_v1(25, &sample), 4);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample: Vec<u16> = vec![20, 15, 10, 5, 5];
    assert_eq!(build_solutions_v2(25, &sample), 3);
  }
}
