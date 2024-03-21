//! Advent of Code 2015: Day 4: The Ideal Stocking Stuffer
//!
//! # Original exercise:
//! > ## --- Day 4: The Ideal Stocking Stuffer ---
//! > Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.
//! >
//! > To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.
//! >
//! > For example:
//! >
//! > - If your secret key is `abcdef`, the answer is `609043`, because the MD5 hash of `abcdef609043` starts with five zeroes (`000001dbbfa...`), and it is the lowest such number to do so.
//! > - If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970` looks like `000006136ef...`.
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//! >
//! > ## --- Part Two ---
//! > Now find one that starts with six zeroes.
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//!

use md5::{Digest, Md5};

#[mutants::skip] // Don't even try this hahaha
fn loop_until_hash(input: &str, stop_value: u8) -> u32 {
  let mut starter: u32 = 0;
  let mut hasher = Md5::new();
  hasher.update(input);
  loop {
    let mut hasher_num = hasher.clone();
    hasher_num.update(starter.to_string());
    let hash = hasher_num.finalize();
    if hash[..2] == [0, 0] && hash[2] <= stop_value {
      return starter;
    }

    starter += 1;
  }
}

pub fn day_04_v1(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  let clean_str = input_str.trim_end();
  return loop_until_hash(clean_str, 15);
}

pub fn day_04_v2(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  let clean_str = input_str.trim_end();
  return loop_until_hash(clean_str, 0);
}

solvable!(day_04, day_04_v1, day_04_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore = "Too slow for CI"]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 2] = [
      ("abcdef", 609043),
      ("pqrstuv", 1048970),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_04_v1(*sample), *result);
    }
  }

  #[test]
  #[ignore = "Too slow for CI"]
  fn works_with_samples_v2() {
    assert_eq!(day_04_v2("abcdef"), 6742839);
  }
}
