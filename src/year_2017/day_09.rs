fn traverse_input(input: &str) -> (usize, usize) {
  let mut result = 0;
  let mut garbage = 0;
  let mut depth = 0;
  let mut in_cancel = false;
  let mut in_garbage = false;

  for chr in input.trim_end().chars() {
    if in_cancel {
      in_cancel = false;
      continue;
    }
    match (in_garbage, chr) {
      (_, '!') => in_cancel = true,
      (true, '>') => in_garbage = false,
      (true, _) => garbage += 1,
      (false, '}') => depth -= 1,
      (false, '<') => in_garbage = true,
      (false, '{') => {
        depth += 1;
        result += depth;
      }
      (false, _) => (),
    }
  }

  (result, garbage)
}

pub fn day_09_v1(input: impl Into<String>) -> usize {
  traverse_input(&input.into()).0
}

pub fn day_09_v2(input: impl Into<String>) -> usize {
  traverse_input(&input.into()).1
}

solvable!(day_09, day_09_v1, day_09_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, usize); 8] = [
      (r#"{}"#, 1),
      (r#"{{{}}}"#, 6),
      (r#"{{},{}}"#, 5),
      (r#"{{{},{},{{}}}}"#, 16),
      (r#"{<a>,<a>,<a>,<a>}"#, 1),
      (r#"{{<ab>},{<ab>},{<ab>},{<ab>}}"#, 9),
      (r#"{{<!!>},{<!!>},{<!!>},{<!!>}}"#, 9),
      (r#"{{<a!>},{<a!>},{<a!>},{<ab>}}"#, 3),
    ];

    for (sample, result) in sample_one {
      assert_eq!(day_09_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, usize); 7] = [
      (r#"<>"#, 0),
      (r#"<random characters>"#, 17),
      (r#"<<<<>"#, 3),
      (r#"<{!>}>"#, 2),
      (r#"<!!>"#, 0),
      (r#"<!!!>>"#, 0),
      (r#"<{o"i!a,<{i<a>"#, 10),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_09_v2(sample), result);
    }
  }
}
