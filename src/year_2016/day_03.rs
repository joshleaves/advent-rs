use itertools::Itertools;

fn is_triangle_possible(triangle: [u16; 3]) -> bool {
  let sides: Vec<_> = triangle.iter().sorted().collect();

  (*sides[0] + *sides[1]) > *sides[2]
}

pub fn day_03_v1(input: impl Into<String>) -> u16 {
  let mut result: u16 = 0;
  for line in input.into().lines() {
    let parts: Vec<_> = line.split_whitespace().collect();
    let numbers: [u16; 3] = [
      parts[0].parse::<u16>().unwrap(),
      parts[1].parse::<u16>().unwrap(),
      parts[2].parse::<u16>().unwrap(),
    ];
    if is_triangle_possible(numbers) {
      result += 1;
    }
  }

  result
}

pub fn day_03_v2(input: impl Into<String>) -> u16 {
  let mut result: u16 = 0;
  for mut lines in &input.into().lines().chunks(3) {
    let parts_one: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    let parts_two: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    let parts_tre: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    for i in 0..=2 {
      let numbers: [u16; 3] = [
        parts_one[i].parse::<u16>().unwrap(),
        parts_two[i].parse::<u16>().unwrap(),
        parts_tre[i].parse::<u16>().unwrap(),
      ];
      if is_triangle_possible(numbers) {
        result += 1;
      }
    }
  }

  result
}

solvable!(day_03, day_03_v1, day_03_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn checks_if_triangle_is_possible() {
    let sample: [u16; 3] = [5, 10, 25];
    assert_eq!(is_triangle_possible(sample), false);
  }

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "5 10 25";
    assert_eq!(day_03_v1(sample_one), 0);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "101 301 501\n\
      102 302 502\n\
      103 303 503\n\
      201 401 601\n\
      202 402 602\n\
      203 403 603";
    assert_eq!(day_03_v2(sample_two), 6);
  }
}
