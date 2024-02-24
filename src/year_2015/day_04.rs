use md5::{Digest, Md5};

pub fn loop_until_hash(input: &str, stopper: &str) -> u32 {
  let mut starter = 0;
  loop {
    let hash = format!("{:X}", Md5::digest(format!("{input}{starter}")));
    if hash.starts_with(stopper) {
      return starter;
    }

    starter += 1;
  }
}

pub fn day_04_v1(input: &str) -> u32 {
  let clean_str = input.lines().next().expect("OK");
  return loop_until_hash(clean_str, "00000");
}

pub fn day_04_v2(input: &str) -> u32 {
  let clean_str = input.lines().next().expect("OK");
  return loop_until_hash(clean_str, "000000");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 2] = [("abcdef", 609043), ("pqrstuv", 1048970)];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_04_v1(sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_04_v2("abcdef"), 6742839);
  }
}
