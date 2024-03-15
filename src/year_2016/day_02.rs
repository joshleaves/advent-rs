fn matcher_v1(curbut: char, line: &str) -> char {
  let mut nextbut: char = curbut;
  for direction in line.chars() {
    nextbut = match (nextbut, direction) {
      ('1', 'R') => '2',
      ('1', 'D') => '4',
      ('2', 'L') => '1',
      ('2', 'R') => '3',
      ('2', 'D') => '5',
      ('3', 'L') => '2',
      ('3', 'D') => '6',
      ('4', 'U') => '1',
      ('4', 'R') => '5',
      ('4', 'D') => '7',
      ('5', 'U') => '2',
      ('5', 'L') => '4',
      ('5', 'R') => '6',
      ('5', 'D') => '8',
      ('6', 'L') => '5',
      ('6', 'U') => '3',
      ('6', 'D') => '9',
      ('7', 'R') => '8',
      ('7', 'U') => '4',
      ('8', 'L') => '7',
      ('8', 'R') => '9',
      ('8', 'U') => '5',
      ('9', 'L') => '8',
      ('9', 'U') => '6',
      _ => nextbut,
    }
  }
  nextbut
}

fn matcher_v2(curbut: char, line: &str) -> char {
  let mut nextbut: char = curbut;
  for direction in line.chars() {
    nextbut = match (nextbut, direction) {
      ('1', 'D') => '3',
      ('2', 'R') => '3',
      ('2', 'D') => '6',
      ('3', 'L') => '2',
      ('3', 'U') => '1',
      ('3', 'D') => '7',
      ('3', 'R') => '4',
      ('4', 'L') => '3',
      ('4', 'D') => '8',
      ('5', 'R') => '6',
      ('6', 'U') => '2',
      ('6', 'L') => '5',
      ('6', 'R') => '7',
      ('6', 'D') => 'A',
      ('7', 'U') => '3',
      ('7', 'L') => '6',
      ('7', 'R') => '8',
      ('7', 'D') => 'B',
      ('8', 'U') => '4',
      ('8', 'L') => '7',
      ('8', 'R') => '9',
      ('8', 'D') => 'C',
      ('9', 'L') => '8',
      ('A', 'R') => 'B',
      ('A', 'U') => '6',
      ('B', 'L') => 'A',
      ('B', 'U') => '7',
      ('B', 'D') => 'D',
      ('B', 'R') => 'C',
      ('C', 'L') => 'B',
      ('C', 'U') => '8',
      ('D', 'U') => 'B',
      _ => nextbut,
    }
  }
  nextbut
}

pub fn day_02_v1(input: impl Into<String>) -> String {
  let mut result: Vec<char> = vec![];
  let mut curbut = '5';
  for line in input.into().lines() {
    curbut = matcher_v1(curbut, line);
    result.push(curbut);
  }

  result.iter().collect::<String>()
}

pub fn day_02_v2(input: impl Into<String>) -> String {
  let mut result: Vec<char> = vec![];
  let mut curbut = '5';
  for line in input.into().lines() {
    curbut = matcher_v2(curbut, line);
    result.push(curbut);
  }

  result.iter().collect::<String>()
}

solvable!(day_02, day_02_v1, day_02_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_02_v1(SAMPLE), "1985");
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_02_v2(SAMPLE), "5DB3");
  }
}
