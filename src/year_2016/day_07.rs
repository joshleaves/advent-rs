use itertools::Itertools;
use std::collections::HashSet;

fn address_supports_tls(address: &str) -> bool {
  let mut in_brackets = false;
  let mut got_abba = false;
  for tls_frame in address.chars().collect_vec().windows(4) {
    if tls_frame[0] == '[' || tls_frame[0] == ']' {
      in_brackets = tls_frame[0] == '[';
      continue;
    }
    if tls_frame[0] == tls_frame[1] || tls_frame[0] != tls_frame[3] || tls_frame[1] != tls_frame[2]
    {
      continue;
    }
    if in_brackets {
      return false;
    }
    got_abba = true
  }

  got_abba
}

fn address_supports_ssl(address: &str) -> bool {
  let mut in_brackets = false;
  let mut aba_list: HashSet<(char, char)> = HashSet::new();
  let mut bab_list: HashSet<(char, char)> = HashSet::new();
  for ssl_frame in address.chars().collect_vec().windows(3) {
    if ssl_frame[0] == '[' || ssl_frame[0] == ']' {
      in_brackets = ssl_frame[0] == '[';
      continue;
    }
    if ssl_frame[0] == ssl_frame[1] || ssl_frame[0] != ssl_frame[2] {
      continue;
    }
    if in_brackets {
      bab_list.insert((ssl_frame[0], ssl_frame[1]));
    } else {
      aba_list.insert((ssl_frame[1], ssl_frame[0]));
    }
  }

  aba_list.intersection(&bab_list).next().is_some()
}

pub fn day_07_v1(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| address_supports_tls(line))
    .count() as u16
}

pub fn day_07_v2(input: impl Into<String>) -> u16 {
  input
    .into()
    .lines()
    .filter(|line| address_supports_ssl(line))
    .count() as u16
}

solvable!(day_07, day_07_v1, day_07_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u16); 4] = [
      ("abba[mnop]qrst", 1),
      ("abcd[bddb]xyyx", 0),
      ("aaaa[qwer]tyui", 0),
      ("ioxxoj[asdfgh]zxcvbn", 1),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_07_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, u16); 4] = [
      ("aba[bab]xyz", 1),
      ("xyx[xyx]xyx", 0),
      ("aaa[kek]eke", 1),
      ("zazbz[bzb]cdb", 1),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_07_v2(sample), result);
    }
  }
}
