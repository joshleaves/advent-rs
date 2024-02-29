pub fn day_01_v1(input: impl Into<String>) -> i16 {
  let mut lvl: i16 = 0;

  for chr in input.into().chars() {
    match chr {
      '(' => lvl += 1,
      ')' => lvl -= 1,
      _ => panic!("Invalid character: {}", chr),
    }
  }
  lvl
}

pub fn day_01_v2(input: impl Into<String>) -> i16 {
  let mut lvl: i16 = 0;

  for (idx, chr) in input.into().chars().enumerate() {
    match chr {
      '(' => lvl += 1,
      ')' => lvl -= 1,
      _ => panic!("Invalid character: {}", chr),
    }
    if lvl < 0 {
      return idx as i16 + 1;
    }
  }
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, i16); 9] = [
      ("(())", 0),
      ("()()", 0),
      ("(((", 3),
      ("(()(()(", 3),
      ("))(((((", 3),
      ("())", -1),
      ("))(", -1),
      (")))", -3),
      (")())())", -3),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_01_v1(*sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, i16); 2] = [
      (")", 1),
      ("()())", 5),
    ];
    for (sample, result) in sample_two.iter() {
      assert_eq!(day_01_v2(*sample), *result);
    }
  }
}
