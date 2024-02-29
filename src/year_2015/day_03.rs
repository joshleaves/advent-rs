use std::collections::HashSet;

#[inline]
fn move_character(pos: (i8, i8), direction: char) -> (i8, i8) {
  let mut newpos = pos.clone();
  match direction {
    '>' => newpos.0 += 1,
    '<' => newpos.0 -= 1,
    'v' => newpos.1 += 1,
    '^' => newpos.1 -= 1,
    _ => panic!("Invalid direction character: {direction}"),
  }
  newpos
}

pub fn day_03_v1(input: impl Into<String>) -> usize {
  let mut santa: (i8, i8) = (0, 0);
  let mut houses = HashSet::from([santa]);

  for (_idx, chr) in input.into().chars().enumerate() {
    santa = move_character(santa, chr);
    houses.insert(santa);
  }
  return houses.len() as usize;
}

pub fn day_03_v2(input: impl Into<String>) -> usize {
  let mut santa: (i8, i8) = (0, 0);
  let mut robot: (i8, i8) = (0, 0);
  let mut houses = HashSet::from([santa]);
  let moves: Vec<char> = input.into().chars().collect();

  for chars in moves.chunks(2) {
    santa = move_character(santa, chars[0]);
    robot = move_character(robot, chars[1]);
    houses.insert(santa);
    houses.insert(robot);
  }
  return houses.len() as usize;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn moves_characters_properly() {
    let sample_moves: [((i8, i8), char, (i8, i8)); 4] = [
      ((0, 0), '>', (1, 0)),
      ((0, 0), '<', (-1, 0)),
      ((0, 0), '^', (0, -1)),
      ((0, 0), 'v', (0, 1)),
    ];

    for (sample, direction, result) in sample_moves.iter() {
      assert_eq!(move_character(*sample, *direction), *result);
    }
  }

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, usize); 3] = [
      (">", 2),
      ("^>v<", 4),
      ("^v^v^v^v^v", 2),
    ];

    for (sample, result) in sample_one.iter() {
      assert_eq!(day_03_v1(*sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, usize); 3] = [
      ("^v", 3),
      ("^>v<", 3),
      ("^v^v^v^v^v", 11),
    ];

    for (sample, result) in sample_two.iter() {
      assert_eq!(day_03_v2(*sample), *result);
    }
  }
}
