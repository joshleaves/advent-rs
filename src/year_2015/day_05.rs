//! Advent of Code 2015: Day 5: Doesn't He Have Intern-Elves For This?
//!
//! # Original exercise:
//! > ## --- Day 5: Doesn't He Have Intern-Elves For This? ---
//! > Santa needs help figuring out which strings in his text file are naughty or nice.
//! >
//! > A nice string is one with all of the following properties:
//! >
//! > - It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
//! > - It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`), or `aabbccdd` (`aa`, `bb`, `cc`, or `dd`).
//! > - It does not contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of the other requirements.
//! >
//! > For example:
//! >
//! > - `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double letter (`...dd...`), and none of the disallowed substrings.
//! > - `aaa` is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
//! > - `jchzalrnumimnmhp` is naughty because it has no double letter.
//! > - `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
//! > - `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
//! >
//! > How many strings are nice?
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//! >
//! > ## --- Part Two ---
//! > Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.
//! >
//! > Now, a nice string is one with all of the following properties:
//! >
//! > - It contains a pair of any two letters that appears at least twice in the string without overlapping, like `xyxy` (`xy`) or `aabcdefgaa` (`aa`), but not like `aaa` (`aa`, but it overlaps).
//! > - It contains at least one letter which repeats with exactly one letter between them, like `xyx`, `abcdefeghi` (`efe`), or even `aaa`.
//! >
//! > For example:
//! >
//! > - `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter that repeats with exactly one letter between them (`zxz`).
//! > - `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
//! > - `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single letter between them.
//! > - `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`), but no pair that appears twice.
//! >
//! > How many strings are nice under these new rules?
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.

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
    Ok(elt)
  });
  no_naughty_sequences != Err('-') && repeated && vowels >= 3
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
  twice_pair && repeated
}

pub fn day_05_v1(input: impl Into<String>) -> usize {
  let input_str = input.into();
  return input_str
    .lines()
    .filter(|line| string_is_nice_v1(line))
    .count();
}

pub fn day_05_v2(input: impl Into<String>) -> usize {
  let input_str = input.into();
  return input_str
    .lines()
    .filter(|line| string_is_nice_v2(line))
    .count();
}

solvable!(day_05, day_05_v1, day_05_v2, usize);

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
