use std::collections::HashSet;
use std::collections::VecDeque;

enum PositionOrCount {
  Position((usize, usize)),
  Count(u8),
}

impl PositionOrCount {
  fn reached_position(&self, reach: (usize, usize)) -> bool {
    match self {
      PositionOrCount::Position(target) => *target == reach,
      PositionOrCount::Count(_) => false,
    }
  }

  fn reached_count(&self, steps_count: u8) -> bool {
    match self {
      PositionOrCount::Position(_) => false,
      PositionOrCount::Count(count) => steps_count > *count,
    }
  }
}

fn cell_is_wall(favorite: usize, position: (usize, usize)) -> bool {
  let x = position.0;
  let y = position.1;
  let value = x * x + 3 * x + 2 * x * y + y + y * y + favorite;
  return value.count_ones() % 2 == 1;
}

fn next_moves(position: (usize, usize)) -> Vec<(usize, usize)> {
  let mut new_moves: Vec<(usize, usize)> = vec![];
  if position.0 > 0 {
    new_moves.push((position.0 - 1, position.1));
  }
  new_moves.push((position.0 + 1, position.1));
  if position.1 > 0 {
    new_moves.push((position.0, position.1 - 1));
  }
  new_moves.push((position.0, position.1 + 1));

  new_moves
}

fn traverse_until(favorite: usize, reach_condition: PositionOrCount) -> u8 {
  let mut visited: HashSet<(usize, usize)> = HashSet::from([(1, 1)]);
  let mut queue: VecDeque<((usize, usize), u8)> = VecDeque::from([((1, 1), 0)]);

  while let Some((position, depth)) = queue.pop_front() {
    if reach_condition.reached_position(position) {
      return depth;
    }
    if reach_condition.reached_count(depth) {
      return visited.len() as u8;
    }

    visited.insert(position);
    for next_move in next_moves(position) {
      if visited.contains(&next_move) || cell_is_wall(favorite, next_move) {
        continue;
      }
      queue.push_back((next_move, depth + 1));
    }
  }

  0
}

pub fn day_13_v1(input: impl Into<String>) -> u8 {
  let binding = input.into();
  let input_parts: Vec<_> = binding.trim_end().split(",").collect();
  let favorite = input_parts[0].parse::<usize>().unwrap();
  let reach_x = input_parts[1].parse::<usize>().unwrap();
  let reach_y = input_parts[2].parse::<usize>().unwrap();

  traverse_until(favorite, PositionOrCount::Position((reach_x, reach_y)))
}

pub fn day_13_v2(input: impl Into<String>) -> u8 {
  let binding = input.into();
  let input_parts: Vec<_> = binding.split(",").collect();
  let favorite = input_parts[0].parse::<usize>().unwrap();

  traverse_until(favorite, PositionOrCount::Count(50))
}

solvable!(day_13, day_13_v1, day_13_v2, u8);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_13_v1("10,7,4"), 11);
  }
}
