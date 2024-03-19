use std::collections::VecDeque;

use itertools::Itertools;
use md5::{digest::core_api::CoreWrapper, Digest, Md5, Md5Core};

fn multi_hash(input: &str) -> String {
  let mut digest: String = input.to_string();
  for _ in 1..=2016 {
    let mut hasher = Md5::new();
    hasher.update(digest);
    digest = format!("{:x}", hasher.finalize());
  }
  digest
}

fn generate_hashes_v2(
  hasher: CoreWrapper<Md5Core>,
  starter: usize,
  ending: usize,
) -> VecDeque<String> {
  let mut hashes: Vec<String> = Vec::new();
  for idx in starter..=ending {
    let mut hasher_num = hasher.clone();
    hasher_num.update(&idx.to_string());
    hashes.push(multi_hash(&format!("{:x}", hasher_num.finalize())));
  }

  VecDeque::from(hashes)
}

fn generate_hashes(
  hasher: CoreWrapper<Md5Core>,
  starter: usize,
  ending: usize,
) -> VecDeque<String> {
  let mut hashes: Vec<String> = Vec::new();
  for idx in starter..=ending {
    let mut hasher_num = hasher.clone();
    hasher_num.update(&idx.to_string());
    hashes.push(format!("{:x}", hasher_num.finalize()));
  }

  VecDeque::from(hashes)
}

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

fn got_quintuplet(input: &str, needle: char) -> bool {
  input.chars().collect_vec().windows(5).any(|letters| {
    letters[0] == needle
      && letters[1] == needle
      && letters[2] == needle
      && letters[3] == needle
      && letters[4] == needle
  })
}

pub fn day_14_v1(input: impl Into<String>) -> usize {
  let binding = &input.into();
  let data = binding.lines().next().unwrap();
  let hasher = build_hasher(data);
  let mut starter: usize = 0;

  let mut hash_queue: VecDeque<String> = VecDeque::new();
  hash_queue.append(&mut generate_hashes(hasher.clone(), 0, 1000));
  let mut keys_count = 0;

  loop {
    starter += 1;
    if hash_queue.is_empty() {
      hash_queue.append(&mut generate_hashes(
        hasher.clone(),
        starter,
        starter + 1000,
      ));
    }
    let digest = hash_queue.pop_front().unwrap();
    if let Some(triplet) = find_triplet(&digest) {
      let from = starter + 1 + hash_queue.len();
      let to = from + (starter + 1000) - hash_queue.len();
      hash_queue.append(&mut generate_hashes(hasher.clone(), from, to));
      for idx in 0..1000 {
        if got_quintuplet(&hash_queue[idx], triplet) {
          if keys_count == 63 {
            return starter;
          }
          keys_count += 1;
          break;
        }
      }
    }
  }
}

pub fn day_14_v2(input: impl Into<String>) -> usize {
  let binding = &input.into();
  let data = binding.lines().next().unwrap();
  let hasher = build_hasher(data);
  let mut starter: usize = 0;

  let mut hash_queue: VecDeque<String> = VecDeque::new();
  hash_queue.append(&mut generate_hashes_v2(hasher.clone(), 0, 1000));
  let mut keys_count = 0;

  loop {
    starter += 1;
    if hash_queue.is_empty() {
      hash_queue.append(&mut generate_hashes_v2(
        hasher.clone(),
        starter,
        starter + 1000,
      ));
    }
    let digest = hash_queue.pop_front().unwrap();
    if let Some(triplet) = find_triplet(&digest) {
      let from = starter + 1 + hash_queue.len();
      let to = from + (starter + 1000) - hash_queue.len();
      hash_queue.append(&mut generate_hashes_v2(hasher.clone(), from, to));
      for idx in 0..1000 {
        if got_quintuplet(&hash_queue[idx], triplet) {
          if keys_count == 63 {
            return starter;
          }
          keys_count += 1;
          break;
        }
      }
    }
  }
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
