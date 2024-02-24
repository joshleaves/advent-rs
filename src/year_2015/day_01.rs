pub fn day_01_v1(input: &str) -> i32 {
  let right: Vec<&str> = input.matches("(").collect();
  let left: Vec<&str> = input.matches(")").collect();
  let rlen = right.len() as i32;
  let llen = left.len() as i32;

  rlen - llen
}

pub fn day_01_v2(input: &str) -> i32 {
  let mut lvl = 0;

  for (idx, chr) in input.chars().enumerate() {
    if chr == '(' {
      lvl += 1;
    }
    if chr == ')' {
      lvl -= 1;
    }
    if lvl < 0 {
      return idx as i32 + 1;
    }
  }
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, i32); 9] = [
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
      assert_eq!(day_01_v1(sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, i32); 2] = [(")", 1), ("()())", 5)];
    for (sample, result) in sample_two.iter() {
      assert_eq!(day_01_v2(sample), *result);
    }
  }
}
