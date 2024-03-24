use crate::bfs::BreadthFirstSearch;
use itertools::Itertools;
use std::collections::HashMap;

struct Labyrinth {
  width: usize,
  height: usize,
  cells: Vec<bool>,
  numbers: Vec<(usize, usize)>,
}

impl Labyrinth {
  fn new(input: &str) -> Self {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut nums = vec![];
    let mut cells = vec![true; height * width];
    for (y, line) in input.lines().enumerate() {
      for (x, character) in line.chars().enumerate() {
        match character {
          '.' => (),
          '#' => cells[y * width + x] = false,
          '0'..='9' => nums.push((character as u8 - 48, (x, y))),
          _ => panic!("Invalid character: {}", character),
        }
      }
    }
    nums.sort_by_key(|elt| elt.0);
    let numbers = nums.iter().map(|elt| elt.1).collect();
    Labyrinth {
      width,
      height,
      cells,
      numbers,
    }
  }

  pub fn next_moves(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut results = vec![];
    if position.0 > 0 && self.cells[position.1 * self.width + (position.0 - 1)] {
      results.push((position.0 - 1, position.1));
    }
    if position.0 < self.width && self.cells[position.1 * self.width + (position.0 + 1)] {
      results.push((position.0 + 1, position.1));
    }

    if position.1 > 0 && self.cells[(position.1 - 1) * self.width + position.0] {
      results.push((position.0, position.1 - 1));
    }
    if position.1 < self.height && self.cells[(position.1 + 1) * self.width + position.0] {
      results.push((position.0, position.1 + 1));
    }
    results
  }
}

#[inline]
fn get_distance(distances: &HashMap<(&usize, &usize), usize>, from: usize, to: usize) -> usize {
  match distances.get(&(&from, &to)) {
    Some(distance) => *distance,
    None => panic!("No distance between {from} and {to}!"),
  }
}

#[inline]
fn remove_position_from_vec(positions: &[usize], needle: &usize) -> Vec<usize> {
  let mut new_positions = positions.to_owned();
  new_positions.retain(|elt| elt != needle);
  new_positions
}

fn best_path_v1(
  distances: &HashMap<(&usize, &usize), usize>,
  position: usize,
  positions_left: Vec<usize>,
  path: usize,
  mut best_path: usize,
) -> usize {
  if path > best_path {
    return path;
  }
  if positions_left.is_empty() {
    return path;
  }
  for next_position in positions_left.iter() {
    let next_positions_left = remove_position_from_vec(&positions_left, next_position);
    let next_path = path + get_distance(distances, position, *next_position);
    let next_distance = best_path_v1(
      distances,
      *next_position,
      next_positions_left,
      next_path,
      best_path,
    );
    best_path = std::cmp::min(best_path, next_distance);
  }

  best_path
}

pub fn day_24_v1(input: impl Into<String>) -> usize {
  let labyrinth = Labyrinth::new(&input.into());
  let numbers: Vec<_> = (0..labyrinth.numbers.len()).collect();
  let mut distances = HashMap::new();
  let combos: Vec<_> = numbers.iter().combinations(2).collect();
  combos.iter().for_each(|pair| {
    let starter = labyrinth.numbers[*pair[0]];
    let ending = labyrinth.numbers[*pair[1]];
    let mut bfs = BreadthFirstSearch::new(starter, |position| labyrinth.next_moves(position));
    let distance = bfs.shortest_path_to(ending);
    distances.insert((pair[0], pair[1]), distance);
    distances.insert((pair[1], pair[0]), distance);
  });

  best_path_v1(
    &distances,
    0,
    (1..labyrinth.numbers.len()).collect(),
    0,
    usize::MAX,
  )
}

fn best_path_v2(
  distances: &HashMap<(&usize, &usize), usize>,
  position: usize,
  positions_left: Vec<usize>,
  path: usize,
  mut best_path: usize,
) -> usize {
  if path > best_path {
    return path;
  }
  if positions_left.is_empty() {
    return path + get_distance(distances, position, 0);
  }
  for next_position in positions_left.iter() {
    let next_positions_left = remove_position_from_vec(&positions_left, next_position);
    let next_path = path + get_distance(distances, position, *next_position);
    let next_distance = best_path_v2(
      distances,
      *next_position,
      next_positions_left,
      next_path,
      best_path,
    );
    best_path = std::cmp::min(best_path, next_distance);
  }

  best_path
}

pub fn day_24_v2(input: impl Into<String>) -> usize {
  let labyrinth = Labyrinth::new(&input.into());
  let numbers: Vec<_> = (0..labyrinth.numbers.len()).collect();
  let mut distances = HashMap::new();
  let combos: Vec<_> = numbers.iter().combinations(2).collect();
  combos.iter().for_each(|pair| {
    let starter = labyrinth.numbers[*pair[0]];
    let ending = labyrinth.numbers[*pair[1]];
    let mut bfs = BreadthFirstSearch::new(starter, |position| labyrinth.next_moves(position));
    let distance = bfs.shortest_path_to(ending);
    distances.insert((pair[0], pair[1]), distance);
    distances.insert((pair[1], pair[0]), distance);
  });

  best_path_v2(
    &distances,
    0,
    (1..labyrinth.numbers.len()).collect(),
    0,
    usize::MAX,
  )
}

solvable!(day_24, day_24_v1, day_24_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample = "###########\n\
      #0.1.....2#\n\
      #.#######.#\n\
      #4.......3#\n\
      ###########";
    assert_eq!(day_24_v1(sample), 14);
  }
}
