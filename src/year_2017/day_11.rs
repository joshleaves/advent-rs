use itertools::Itertools;

#[inline]
fn parse_position(position: (i32, i32)) -> u32 {
  (position.0.unsigned_abs() + position.1.unsigned_abs()) / 2
}

fn traverse(input: &str) -> (u32, u32) {
  let directions = input.trim_end().split(',').collect_vec();
  let mut position: (i32, i32) = (0, 0);
  let mut distance_max = 0;
  for direction in directions {
    let add_position = match direction {
      "n" => (0, -2),
      "s" => (0, 2),
      "nw" => (-1, -1),
      "ne" => (1, -1),
      "sw" => (-1, 1),
      "se" => (1, 1),
      _ => panic!("Invalid input: {}", direction),
    };
    position.0 += add_position.0;
    position.1 += add_position.1;
    distance_max = std::cmp::max(distance_max, parse_position(position));
  }
  (parse_position(position), distance_max)
}

pub fn day_11_v1(input: impl Into<String>) -> u32 {
  traverse(&input.into()).0
}

pub fn day_11_v2(input: impl Into<String>) -> u32 {
  traverse(&input.into()).1
}

solvable!(day_11, day_11_v1, day_11_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 4] = [
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
