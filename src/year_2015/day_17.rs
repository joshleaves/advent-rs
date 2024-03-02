//! Advent of Code 2015: Day 17: No Such Thing as Too Much

const EGGNOG_LITERS: usize = 150;

fn parse_file(input: String) -> Vec<usize> {
  let mut inputs: Vec<usize> = vec![];
  for line in input.lines() {
    let Ok(value) = line.parse::<usize>() else {
      panic!("Invalid number: {}", line)
    };
    inputs.push(value);
  }

  inputs
}

fn build_combinations(
  liters: usize,
  containers_left: Vec<usize>,
  containers_build: Vec<usize>,
) -> Vec<Vec<usize>> {
  let mut solutions: Vec<Vec<usize>> = vec![];
  let container_size: usize = containers_build.iter().sum::<usize>();
  if container_size >= liters || containers_left.is_empty() {
    if container_size == liters {
      solutions.push(containers_build);
    }
    return solutions;
  }
  for idx in 0..containers_left.len() {
    let next_containers_left: Vec<usize> = containers_left[idx + 1..].to_owned();
    let mut next_containers_build: Vec<usize> = containers_build.clone();
    next_containers_build.push(containers_left[idx]);
    let mut new_solutions = build_combinations(liters, next_containers_left, next_containers_build);
    solutions.append(&mut new_solutions);
  }

  solutions
}

fn build_solutions(liters: usize, containers: &Vec<usize>) -> Vec<Vec<usize>> {
  let mut solutions: Vec<Vec<usize>> = vec![];
  let mut start: usize = 0;
  loop {
    if containers[start..].iter().sum::<usize>() < liters {
      return solutions;
    }
    let containers_left: Vec<usize> = containers[start + 1..].to_owned();
    let containers_build: Vec<usize> = vec![containers[start]];
    let mut new_solutions = build_combinations(liters, containers_left, containers_build);
    solutions.append(&mut new_solutions);
    start += 1;
  }
}

/// Solve exercise for year 2015, day 17 (part 1).
pub fn day_17_v1(input: impl Into<String>) -> usize {
  let containers: Vec<usize> = parse_file(input.into());
  let solutions = build_solutions(EGGNOG_LITERS, &containers);

  solutions.len()
}

/// Solve exercise for year 2015, day 17 (part 2).
pub fn day_17_v2(input: impl Into<String>) -> usize {
  let containers: Vec<usize> = parse_file(input.into());
  let mut solutions = build_solutions(EGGNOG_LITERS, &containers);
  let Some(min_solution) = solutions.iter().map(|s| s.len()).min() else {
    panic!("No minimum size: {:?}", solutions)
  };
  solutions.retain(|s| s.len() == min_solution);

  solutions.len()
}

solvable!(day_17, day_17_v1, day_17_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample: Vec<usize> = vec![20, 15, 10, 5, 5];
    let solutions = build_solutions(25, &sample);
    assert_eq!(solutions.len(), 4);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample: Vec<usize> = vec![20, 15, 10, 5, 5];
    let mut solutions = build_solutions(25, &sample);
    let Some(min_solution) = solutions.iter().map(|s| s.len()).min() else {
      panic!("No minimum size: {:?}", solutions)
    };
    solutions.retain(|s| s.len() == min_solution);
    assert_eq!(solutions.len(), 3);
  }
}
