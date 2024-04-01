type Pos = (usize, usize);

enum Dir {
  Up,
  Down,
  Left,
  Right,
}

#[inline]
fn do_move(position: &Pos, direction: &Dir) -> Pos {
  match direction {
    Dir::Up => (position.0, position.1 - 1),
    Dir::Down => (position.0, position.1 + 1),
    Dir::Left => (position.0 - 1, position.1),
    Dir::Right => (position.0 + 1, position.1),
  }
}

#[inline]
fn can_move(map: &[Vec<char>], position: &Pos, direction: &Dir) -> bool {
  let new_position = do_move(position, direction);
  map[new_position.1][new_position.0] != ' '
}

fn traverse_map(input: &str) -> (String, u32) {
  let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  map.push(" ".repeat(map[0].len()).chars().collect());
  let mut path = String::new();
  let idx = map[0].iter().position(|chr| *chr == '|').unwrap();
  let mut step_count = 0u32;
  let mut direction = Dir::Down;
  let mut position = (idx, 0usize);
  loop {
    step_count += 1;
    if let 'A'..='Z' = map[position.1][position.0] {
      path.push(map[position.1][position.0]);
    }
    if !can_move(&map, &position, &direction) {
      match &direction {
        &Dir::Up | &Dir::Down if can_move(&map, &position, &Dir::Left) => direction = Dir::Left,
        &Dir::Up | &Dir::Down if can_move(&map, &position, &Dir::Right) => direction = Dir::Right,
        &Dir::Left | &Dir::Right if can_move(&map, &position, &Dir::Up) => direction = Dir::Up,
        &Dir::Left | &Dir::Right if can_move(&map, &position, &Dir::Down) => direction = Dir::Down,
        _ => break,
      }
    }
    position = do_move(&position, &direction);
  }

  (path, step_count)
}

pub fn day_19_v1(input: impl Into<String>) -> String {
  traverse_map(&input.into()).0
}

pub fn day_19_v2(input: impl Into<String>) -> String {
  traverse_map(&input.into()).1.to_string()
}
solvable!(day_19, day_19_v1, day_19_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = [
      "     |          ",
      "     |  +--+    ",
      "     A  |  C    ",
      " F---|----E|--+ ",
      "     |  |  |  D ",
      "     +B-+  +--+ ",
    ]
    .join("\n");
    assert_eq!(day_19_v1(sample_one), "ABCDEF");
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = [
      "     |          ",
      "     |  +--+    ",
      "     A  |  C    ",
      " F---|----E|--+ ",
      "     |  |  |  D ",
      "     +B-+  +--+ ",
    ]
    .join("\n");
    assert_eq!(day_19_v2(sample_two), "38");
  }
}
