extern crate lazy_static;
use lazy_static::lazy_static;

fn string_is_nice_v1(input: &str) -> bool {
  lazy_static! {
    static ref VOWELS: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
  }

  if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy") {
    return false;
  }
  let mut vowels = 0;
  let mut repeated = false;
  input.chars().fold(' ', |acc, elt| {
    if acc == elt {
      repeated = true
    }
    if VOWELS.contains(&elt) {
      vowels += 1
    }
    elt
  });

  return repeated && vowels >= 3;
}

fn string_is_nice_v2(input: &str) -> bool {
  let mut twice_pair = false;
  let mut repeated = false;
  let letters: Vec<_> = input.chars().collect();
  for i in 0..(input.len() - 2) {
    if letters[i] == letters[i + 2] {
      repeated = true;
    }
    let pattern = &input[i..(i + 2)];
    let ending = &input[i + 2..input.len()];
    if ending.contains(pattern) {
      twice_pair = true;
    }
  }
  return twice_pair && repeated;
}

pub fn day_05_v1(input: &str) -> u32 {
  return input.lines().filter(|line| string_is_nice_v1(line)).count() as u32;
}

pub fn day_05_v2(input: &str) -> u32 {
  return input.lines().filter(|line| string_is_nice_v2(line)).count() as u32;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn finds_nice_strings_v1() {
    let sample_one: [(&str, bool); 5] = [
      ("ugknbfddgicrmopn", true),
      ("aaa", true),
      ("jchzalrnumimnmhp", false),
      ("haegwjzuvuyypxyu", false),
      ("dvszwmarrgswjxmb", false),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(string_is_nice_v1(sample), *result);
    }
  }

  #[test]
  fn finds_nice_strings_v2() {
    let sample_two: [(&str, bool); 4] = [
      ("qjhvhtzxzqqjkmpb", true),
      ("xxyxx", true),
      ("uurcxstgmygtbstg", false),
      ("ieodomkazucvgmuy", false),
    ];
    for (sample, result) in sample_two.iter() {
      assert_eq!(string_is_nice_v2(sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb";
    assert_eq!(day_05_v1(sample_one), 2);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy\n";
    assert_eq!(day_05_v2(sample_two), 2);
  }
}
