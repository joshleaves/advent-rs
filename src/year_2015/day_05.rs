fn string_is_nice_v1(input: &str) -> bool {
  let mut vowels: u8 = 0;
  let mut repeated = false;
  let no_naughty_sequences = input.chars().try_fold(' ', |acc, elt| {
    if acc == 'a' && elt == 'b' {
      return Err('-');
    }
    if acc == 'c' && elt == 'd' {
      return Err('-');
    }
    if acc == 'p' && elt == 'q' {
      return Err('-');
    }
    if acc == 'x' && elt == 'y' {
      return Err('-');
    }
    if acc == elt {
      repeated = true
    }
    if elt == 'a' || elt == 'e' || elt == 'i' || elt == 'o' || elt == 'u' {
      vowels += 1
    }
    return Ok(elt);
  });
  return no_naughty_sequences != Err('-') && repeated && vowels >= 3;
}

fn string_is_nice_v2(input: &str) -> bool {
  let mut twice_pair = false;
  let mut repeated = false;
  let letters: Vec<_> = input.chars().collect();
  for i in 0..(input.len() - 2) {
    if letters[i] == letters[i + 2] {
      repeated = true;
    }
    if twice_pair {
      continue;
    }
    for j in i + 2..input.len() - 1 {
      if letters[i] == letters[j] && letters[i + 1] == letters[j + 1] {
        twice_pair = true;
      }
    }
  }
  return twice_pair && repeated;
}

pub fn day_05_v1(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  return input_str
    .lines()
    .filter(|line| string_is_nice_v1(line))
    .count() as u32;
}

pub fn day_05_v2(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  return input_str
    .lines()
    .filter(|line| string_is_nice_v2(line))
    .count() as u32;
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
      assert_eq!(string_is_nice_v1(*sample), *result);
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
      assert_eq!(string_is_nice_v2(*sample), *result);
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
