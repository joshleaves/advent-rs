use std::collections::VecDeque;

use itertools::Itertools;
use md5::{digest::core_api::CoreWrapper, Digest, Md5, Md5Core};

#[inline]
fn multi_hash(hasher_base: &CoreWrapper<Md5Core>, input: &str) -> String {
  let mut digest: String = input.to_string();
  for _ in 1..=2016 {
    let mut hasher = hasher_base.clone();
    hasher.update(digest);
    digest = format!("{:x}", hasher.finalize());
  }
  digest
}

fn fill_hashes_v2(md5: &CoreWrapper<Md5Core>, start: usize, end: usize) -> VecDeque<String> {
  let mut hashes: VecDeque<String> = VecDeque::new();
  for idx in start..=end {
    let mut md5_num = md5.clone();
    md5_num.update(&idx.to_string());
    let md5_str = Md5::new();
    hashes.push_back(multi_hash(&md5_str, &format!("{:x}", md5_num.finalize())));
  }

  hashes
}

#[inline]
fn build_hasher(input: &str) -> CoreWrapper<Md5Core> {
  let mut hasher = Md5::new();
  let data = input;
  hasher.update(data);
  hasher
}

fn find_triplet(input: &str) -> Option<char> {
  input
    .chars()
    .collect_vec()
    .windows(3)
    .find(|letters| letters[0] == letters[1] && letters[0] == letters[2])
    .map(|letters| letters[0])
}

fn quintuplet(input: &str, needle: char) -> bool {
  input.chars().collect_vec().windows(5).any(|letters| {
    letters[0] == needle
      && letters[1] == needle
      && letters[2] == needle
      && letters[3] == needle
      && letters[4] == needle
  })
}

#[inline]
fn chr_to_chars(chr: &u8) -> [u8; 2] {
  [
    (chr & 0xF0) >> 4,
    chr & 0x0F,
  ]
}

fn find_triplet_u8(input: &[u8]) -> Option<u8> {
  input
    .iter()
    .flat_map(chr_to_chars)
    .collect_vec()
    .windows(3)
    .find(|chars| chars[0] == chars[1] && chars[0] == chars[2])
    .map(|chars| chars[0])
}

fn got_quintuplet_u8(input: &[u8], needle: u8) -> bool {
  input
    .iter()
    .flat_map(chr_to_chars)
    .collect_vec()
    .windows(5)
    .any(|chars| chars.iter().all(|chr| *chr == needle))
}

fn fill_hashes(md5: &CoreWrapper<Md5Core>, counter: usize, hashes: usize) -> VecDeque<Vec<u8>> {
  let start = counter + 1 + hashes;
  let end = start + (counter + 1000) - hashes;
  let mut hashes: VecDeque<Vec<u8>> = VecDeque::new();
  for idx in start..=end {
    let mut md5_num = md5.clone();
    md5_num.update(&idx.to_string());
    hashes.push_back(md5_num.finalize().to_vec());
  }

  hashes
}

pub fn day_14_v1(input: impl Into<String>) -> usize {
  let hasher = build_hasher(input.into().trim_end());
  let mut counter: usize = 0;
  let mut hash_queue: VecDeque<Vec<u8>> = fill_hashes(&hasher, 0, 0);
  let mut keys_count = 0;

  while keys_count < 64 {
    if let Some(digest) = hash_queue.pop_front() {
      counter += 1;
      if let Some(triplet) = find_triplet_u8(&digest) {
        hash_queue.append(&mut fill_hashes(&hasher, counter, hash_queue.len()));
        for hash in hash_queue.iter().take(1000) {
          if got_quintuplet_u8(hash, triplet) {
            keys_count += 1;
          }
        }
      }
    } else {
      hash_queue.append(&mut fill_hashes(&hasher, counter, 0));
    }
  }
  counter
}

pub fn day_14_v2(input: impl Into<String>) -> usize {
  let hasher = build_hasher(input.into().trim_end());
  let mut counter: usize = 0;
  let mut hash_queue: VecDeque<String> = fill_hashes_v2(&hasher, 0, 1000);
  let mut keys_count = 0;

  while keys_count < 64 {
    if let Some(digest) = hash_queue.pop_front() {
      counter += 1;
      if let Some(triplet) = find_triplet(&digest) {
        let from = counter + 1 + hash_queue.len();
        let to = from + (counter + 1000) - hash_queue.len();
        hash_queue.append(&mut fill_hashes_v2(&hasher, from, to));
        for hash in hash_queue.iter().take(1000) {
          if quintuplet(hash, triplet) {
            keys_count += 1;
          }
        }
      }
    } else {
      hash_queue.append(&mut fill_hashes_v2(&hasher, counter, counter + 1000));
    }
  }
  counter
}

solvable!(day_14, day_14_v1, day_14_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_14_v1("abc"), 22_728);
  }

  #[test]
  #[ignore = "Too slow for CI"]
  fn works_with_samples_v2() {
    assert_eq!(day_14_v2("abc"), 22_551);
  }
}
