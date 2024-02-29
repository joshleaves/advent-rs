use md5::{Digest, Md5};

#[mutants::skip] // Don't even try this hahaha
pub fn loop_until_hash(input: &str, stop_value: u8) -> u32 {
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
  let clean_str = input_str.lines().next().expect("OK");
  return loop_until_hash(clean_str, 15);
}

pub fn day_04_v2(input: impl Into<String>) -> u32 {
  let input_str = input.into();
  let clean_str = input_str.lines().next().expect("OK");
  return loop_until_hash(clean_str, 0);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
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
  fn works_with_samples_v2() {
    assert_eq!(day_04_v2("abcdef"), 6742839);
  }
}
