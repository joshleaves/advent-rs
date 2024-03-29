//! Advent of Code 2015: Day 8: Matchsticks

/// Tip: you can cheat here:
/// `line.len() as u16`
fn memory_line_len(line: &str) -> u16 {
  let mut line_len = line.len() as u16 - 2;
  let mut idx: usize = 1;
  while idx < (line.len() - 2) {
    let next_tokens = &line[idx..=idx + 1];
    match next_tokens {
      r#"\\"# | r#"\""# => {
        line_len -= 1;
        idx += 2;
      }
      r#"\x"# => {
        line_len -= 3;
        idx += 4;
        // }
      }
      _ => idx += 1,
    }
  }

  line_len
}

fn dumped_line_len(line: &str) -> u16 {
  let mut line_len = line.len() as u16 + 2_u16;
  for chr in line.bytes() {
    if chr == b'"' || chr == b'\\' {
      line_len += 1;
    }
  }
  line_len
}

pub fn day_08_v1(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .map(|line| line.len() as u16 - memory_line_len(line))
    .sum()
}

pub fn day_08_v2(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .map(|line| dumped_line_len(line) - line.len() as u16)
    .sum()
}

solvable!(day_08, day_08_v1, day_08_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculates_length_of_memory_strings() {
    let sample_one: [(&str, u16); 4] = [
      (r#""""#, 0),
      (r#""abc""#, 3),
      (r#""aaa\"aaa""#, 7),
      (r#""\x27""#, 1),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(memory_line_len(sample), *result);
    }
  }

  #[test]
  fn calculates_length_of_dumped_strings() {
    let sample_one: [(&str, u16); 4] = [
      (r#""""#, 6),
      (r#""abc""#, 9),
      (r#""aaa\"aaa""#, 16),
      (r#""\x27""#, 11),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(dumped_line_len(sample), *result);
    }
  }

  #[test]
  fn calculates_length_of_code_minus_memory() {
    let sample_one: [(&str, u16); 4] = [
      (r#""""#, 2),
      (r#""abc""#, 2),
      (r#""aaa\"aaa""#, 3),
      (r#""\x27""#, 5),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_08_v1(*sample), *result);
    }
  }

  #[test]
  fn calculates_length_of_dumped_minus_code() {
    let sample_one: [(&str, u16); 4] = [
      (r#""""#, 4),
      (r#""abc""#, 4),
      (r#""aaa\"aaa""#, 6),
      (r#""\x27""#, 5),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_08_v2(*sample), *result);
    }
  }

  // #[test]
  // fn works_with_samples_v1() {
  //   let sample_one = "\"\"\n\
  //     \"abc\"\n\
  //     \"aaa\"aaa\"\n\
  //     \"\\x27\"";
  //   assert_eq!(day_08_v1(sample_one), 12);
  // }

  // #[test]
  // fn works_with_samples_v2() {
  //   let sample_two = r#"""\n\
  //     "abc"\n\
  //     "aaa\"aaa"\n\
  //     "\x27""#;
  //   assert_eq!(day_08_v2(sample_two), 19);
  // }
}
