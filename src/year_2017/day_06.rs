use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[inline]
fn parse_input(input: &str) -> Vec<u8> {
  input
    .split_whitespace()
    .map(|word| word.parse::<u8>().unwrap())
    .collect_vec()
}

#[inline]
fn max_idx(input: &[u8]) -> (usize, usize) {
  let mut max: usize = 0;
  let mut idx: usize = 0;
  for (itr, num) in input.iter().enumerate() {
    if *num as usize > max {
      max = *num as usize;
      idx = itr;
    }
  }
  (idx, max)
}

pub fn day_06_v1(input: impl Into<String>) -> usize {
  let mut towers = parse_input(&input.into());
  let towers_len = towers.len();
  let mut positions: HashSet<Vec<u8>> = HashSet::new();
  let mut moves = 0;

  while positions.insert(towers.clone()) {
    moves += 1;
    let (idx, max) = max_idx(&towers);
    towers[idx] = 0;
    for itr in 1..=max {
      towers[(idx + itr) % towers_len] += 1;
    }
  }

  moves
}

pub fn day_06_v2(input: impl Into<String>) -> usize {
  let mut towers = parse_input(&input.into());
  let towers_len = towers.len();
  let mut positions: HashMap<Vec<u8>, usize> = HashMap::new();
  let mut moves = 0;

  loop {
    if let Some(result) = positions.insert(towers.clone(), moves) {
      return moves - result;
    }
    moves += 1;
    let (idx, max) = max_idx(&towers);
    towers[idx] = 0;
    for itr in 1..=max {
      towers[(idx + itr) % towers_len] += 1;
    }
  }
}

solvable!(day_06, day_06_v1, day_06_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_06_v1("0 2 7 0"), 5);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_06_v2("0 2 7 0"), 4);
  }
}
