//! Advent of Code 2015: Day 19: Medicine for Rudolph

use std::collections::{HashMap, HashSet};

fn parse_molecules(input: &str) -> (String, String) {
  let parts: Vec<_> = input.split(" => ").collect();
  (parts[0].to_string(), parts[1].to_string())
}

fn parse_input(input: &str) -> (HashMap<String, Vec<String>>, String) {
  let mut molecules: HashMap<String, Vec<String>> = HashMap::new();
  let mut starter: String = String::new();
  let mut reached_target = false;
  for line in input.lines() {
    if line.is_empty() {
      reached_target = true;
      continue;
    }
    if reached_target {
      starter.push_str(line);
      continue;
    }
    let (from, to) = parse_molecules(line);
    molecules.entry(from).or_insert_with(Vec::new).push(to);
  }

  (molecules, starter)
}

fn do_permutations(starter: &str, input: &str, replacements: &Vec<String>) -> HashSet<String> {
  let mut permutations: HashSet<String> = HashSet::new();
  for (idx, _) in starter.match_indices(input) {
    for replacement in replacements.iter() {
      let new_permutation = format!(
        "{}{}{}",
        &starter[..idx],
        replacement,
        &starter[idx + input.len()..]
      );
      permutations.insert(new_permutation);
    }
  }

  permutations
}

fn calculate_permutations(molecules: &HashMap<String, Vec<String>>, starter: &str) -> usize {
  let mut permutations: HashSet<String> = HashSet::new();
  for (molecule, replacements) in molecules {
    let new_permutations = do_permutations(&starter, &molecule, &replacements);
    permutations.extend(new_permutations);
  }
  permutations.len()
}

pub fn day_19_v1(input: impl Into<String>) -> usize {
  let (molecules, starter) = parse_input(&input.into());

  calculate_permutations(&molecules, &starter)
}

pub fn day_19_v2(input: impl Into<String>) -> usize {
  let (_molecules, starter) = parse_input(&input.into());

  let mut count_az = 0;
  let mut count_rn = 0;
  let mut count_ar = 0;
  let mut count_y = 0;
  let letters: Vec<_> = starter.chars().collect();
  for (idx, chr) in letters.iter().enumerate() {
    match chr {
      'A' => {
        count_az += 1;
        if letters[idx + 1] == 'r' {
          count_ar += 1;
        }
      }
      'R' => {
        count_az += 1;
        if letters[idx + 1] == 'n' {
          count_rn += 1;
        }
      }
      'Y' => {
        count_az += 1;
        count_y += 1;
      }
      _ => {
        if *chr >= 'A' && *chr <= 'Z' {
          count_az += 1;
        }
      }
    }
  }

  count_az - count_rn - count_ar - (2 * count_y) - 1
}

solvable!(day_19, day_19_v1, day_19_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "H => HO\nH => OH\nO => HH\n\n\nHOH";
    assert_eq!(day_19_v1(sample_one), 4);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO;";
    assert_eq!(day_19_v2(sample_two), 5);
  }
}
