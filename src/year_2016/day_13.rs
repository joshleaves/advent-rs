use crate::bfs::BreadthFirstSearch;

fn cell_is_wall(favorite: usize, position: (usize, usize)) -> bool {
  let x = position.0;
  let y = position.1;
  let value = x * x + 3 * x + 2 * x * y + y + y * y + favorite;
  value.count_ones() % 2 == 1
}

fn next_moves(favorite: usize, position: (usize, usize)) -> Vec<(usize, usize)> {
  let mut new_moves: Vec<(usize, usize)> = vec![];
  if position.0 > 0 && !cell_is_wall(favorite, (position.0 - 1, position.1)) {
    new_moves.push((position.0 - 1, position.1));
  }
  if !cell_is_wall(favorite, (position.0 + 1, position.1)) {
    new_moves.push((position.0 + 1, position.1));
  }
  if position.1 > 0 && !cell_is_wall(favorite, (position.0, position.1 - 1)) {
    new_moves.push((position.0, position.1 - 1));
  }
  if !cell_is_wall(favorite, (position.0, position.1 + 1)) {
    new_moves.push((position.0, position.1 + 1));
  }

  new_moves
}

pub fn day_13_v1(input: impl Into<String>) -> u8 {
  let binding = input.into();
  let input_parts: Vec<_> = binding.trim_end().split(',').collect();
  let favorite = input_parts[0].parse::<usize>().unwrap();
  let reach_x = input_parts[1].parse::<usize>().unwrap();
  let reach_y = input_parts[2].parse::<usize>().unwrap();
  let mut bfs = BreadthFirstSearch::new((1, 1), |position| next_moves(favorite, position));
  bfs.traverse_until_position((reach_x, reach_y));

  bfs.depth as u8
}

pub fn day_13_v2(input: impl Into<String>) -> u8 {
  let binding = input.into();
  let input_parts: Vec<_> = binding.split(',').collect();
  let favorite = input_parts[0].parse::<usize>().unwrap();
  let mut bfs = BreadthFirstSearch::new((1, 1), |position| next_moves(favorite, position));
  bfs.traverse_until_depth(50);

  bfs.visited.len() as u8
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
