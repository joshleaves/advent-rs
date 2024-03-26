use itertools::Itertools;

#[inline]
fn parse_input(input: &str) -> Vec<&str> {
  input.trim_end().split(',').collect_vec()
}

fn parse_position(position: (i32, i32)) -> usize {
  ((position.0.unsigned_abs() + position.1.unsigned_abs()) / 2) as usize
}

fn traverse(input: &str) -> (usize, usize) {
  let directions = parse_input(input);
  let mut position: (i32, i32) = (0, 0);
  let mut distance_max = 0;
  for direction in directions {
    match direction {
      "n" => position.1 -= 2,
      "s" => position.1 += 2,
      "nw" => {
        position.0 -= 1;
        position.1 -= 1;
      }
      "ne" => {
        position.0 += 1;
        position.1 -= 1;
      }
      "sw" => {
        position.0 -= 1;
        position.1 += 1;
      }
      "se" => {
        position.0 += 1;
        position.1 += 1;
      }
      _ => panic!("Invalid input: {}", direction),
    }
    distance_max = std::cmp::max(distance_max, parse_position(position));
  }
  (parse_position(position), distance_max)
}

pub fn day_11_v1(input: impl Into<String>) -> usize {
  traverse(&input.into()).0
}

pub fn day_11_v2(input: impl Into<String>) -> usize {
  traverse(&input.into()).1
}

solvable!(day_11, day_11_v1, day_11_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, usize); 4] = [
      ("ne,ne,ne", 3),
      ("ne,ne,sw,sw", 0),
      ("ne,ne,s,s", 2),
      ("se,sw,se,sw,sw", 3),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_11_v1(sample), result);
    }
  }
}
