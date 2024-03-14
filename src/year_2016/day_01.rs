use std::collections::HashSet;

pub fn day_01_v1(input: impl Into<String>) -> i16 {
  let mut direction: i16 = 0;
  let mut position: [i16; 2] = [0, 0];
  for line in input.into().split(", ") {
    let mut line_chars = line.chars();
    let dir_mod = if line_chars.next().unwrap() == 'L' {
      3
    } else {
      5
    };
    direction = (direction + dir_mod) % 4;
    let steps = line_chars.as_str().parse::<i16>().unwrap();
    match direction {
      0 => position[1] -= steps,
      1 => position[0] += steps,
      2 => position[1] += steps,
      3 => position[0] -= steps,
      _ => panic!("No way"),
    }
  }
  position[0].abs() + position[1].abs()
}

pub fn day_01_v2(input: impl Into<String>) -> i16 {
  let mut direction: i16 = 0;
  let mut position: [i16; 2] = [0, 0];
  let mut streets: HashSet<[i16; 2]> = HashSet::from([position]);
  for line in input.into().split(", ") {
    let mut line_chars = line.chars();
    let dir_mod = if line_chars.next().unwrap() == 'L' {
      3
    } else {
      5
    };
    direction = (direction + dir_mod) % 4;
    let steps = line_chars.as_str().parse::<i16>().unwrap();
    for _i in 1..=steps {
      match direction {
        0 => position[1] -= 1,
        1 => position[0] += 1,
        2 => position[1] += 1,
        3 => position[0] -= 1,
        _ => panic!("No way"),
      }
      if streets.contains(&position) {
        return position[0].abs() + position[1].abs();
      }
      streets.insert(position);
    }
  }
  position[0].abs() + position[1].abs()
}

solvable!(day_01, day_01_v1, day_01_v2, i16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, i16); 3] = [
      ("R2, L3", 5),
      ("R2, R2, R2", 2),
      ("R5, L5, R5, R3", 12),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_01_v1(*sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "R8, R4, R4, R8";
    assert_eq!(day_01_v2(sample_two), 4);
  }
}
