use itertools::Itertools;
use std::collections::VecDeque;

pub fn day_19_v1(input: impl Into<String>) -> u32 {
  let number = input.into().parse::<u32>().unwrap();
  let mut binary = format!("{number:b}").chars().collect_vec();
  binary.rotate_left(1);

  u32::from_str_radix(&binary.iter().collect::<String>(), 2).unwrap()
}

pub fn day_19_v2(input: impl Into<String>) -> u32 {
  let number = input.into().parse::<u32>().unwrap();
  let mid = number.div_ceil(2);
  let mut arr_v1: VecDeque<u32> = (1..mid).collect();
  let mut arr_v2: VecDeque<u32> = (mid..(number + 1)).collect();
  loop {
    if arr_v2.len() >= arr_v1.len() {
      arr_v2.pop_front();
      if arr_v2.is_empty() {
        return arr_v1[0];
      }
    } else {
      arr_v1.pop_back();
    }
    arr_v1.push_back(arr_v2.pop_front().unwrap());
    arr_v2.push_back(arr_v1.pop_front().unwrap());
  }
}

solvable!(day_19, day_19_v1, day_19_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_19_v1("5"), 3);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_19_v2("5"), 2);
  }
}
