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
//! # Implementation tricks
//!  
//! Since the [md-5 crate](https://crates.io/crates/md-5) used returns hashes as
//! an array of hex(`u4`) values joined by two into a single `u8`, we can either
//! turn the hash into a string and check for leading `0` characters, which will
//! be costly, or we can limit ourselves to checking the first three `u8``.
//!
//! To match the first '00' hexadecimal pair, we can match it to `0u8`. Same will
//! go for the second pair.
//!
//! For the third one, we're in a bit of a pickle: we must match either one part
//! of the number, or all of it. The most obvious approach is using bitmasks, or
//! we can simply compare the value of the hash. Since the only values producing
//! a hash starting with (at least one) zero are all inferior to `0x10` (`16` in
//! an u8 format), we can match with any number inferior to `16`.
//!
//! # Samples
//! ```
//! /// Import the module
//! use advent_rs::year_2015::day_04;
//!
//! assert_eq!(day_04::day_04_v1("abcdef"), 609_043);
//! assert_eq!(day_04::day_04_v1("pqrstuv"), 1_048_970);
//! assert_eq!(day_04::day_04_v2("abcdef"), 6_742_839);
//! ```

use md5::Digest;
use md5::Md5;

fn find_hash(input: &str, stop_value: u8) -> u32 {
  let mut md5 = Md5::new();
  md5.update(input);
  (0..=u32::MAX)
    .find(|counter| {
      let mut hasher = md5.clone();
      hasher.update(counter.to_string());
      let hash = hasher.finalize();
      hash[0] == 0 && hash[1] == 0 && hash[2] < stop_value
    })
    .unwrap()
}

pub fn day_04_v1(input: impl Into<String>) -> u32 {
  find_hash(input.into().trim_end(), 16)
}

pub fn day_04_v2(input: impl Into<String>) -> u32 {
  find_hash(input.into().trim_end(), 1)
}

solvable!(day_04, day_04_v1, day_04_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore = "Too slow for CI"]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 2] = [
      ("abcdef", 609_043),
      ("pqrstuv", 1_048_970),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_04_v1(*sample), *result);
    }
  }

  #[test]
  #[ignore = "Too slow for CI"]
  fn works_with_samples_v2() {
    assert_eq!(day_04_v2("abcdef"), 6_742_839);
  }
}
