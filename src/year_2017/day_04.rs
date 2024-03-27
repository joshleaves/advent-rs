use itertools::Itertools;

pub fn day_04_v1(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| {
      line
        .split_whitespace()
        .sorted()
        .collect::<Vec<_>>()
        .windows(2)
        .all(|pair| pair[0] != pair[1])
    })
    .count() as u16
}

pub fn day_04_v2(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| {
      line
        .split_whitespace()
        .map(|word| word.chars().sorted().collect::<Vec<_>>())
        .sorted()
        .collect::<Vec<_>>()
        .windows(2)
        .all(|pair| pair[0] != pair[1])
    })
    .count() as u16
}

solvable!(day_04, day_04_v1, day_04_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u16); 3] = [
      ("aa bb cc dd ee", 1),
      ("aa bb cc dd aa", 0),
      ("aa bb cc dd aaa", 1),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_04_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, u16); 5] = [
      ("abcde fghij", 1),
      ("abcde xyz ecdab", 0),
      ("a ab abc abd abf abj", 1),
      ("iiii oiii ooii oooi oooo", 1),
      ("oiii ioii iioi iiio", 0),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_04_v2(sample), result);
    }
  }
}
