//! Year 2017
//!
#![doc = include_str!("../NOTES_2017.md")]

pub mod day_01;

pub fn solve(day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  if part > 2 {
    return None;
  }

  match day {
    1 => Some(day_01::day_01(part, input).to_string()),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_01() {
    let input = include_str!("../inputs/year_2018/day_01_input");
    assert_eq!(day_01::day_01_v1(input), 484);
    assert_eq!(day_01::day_01_v2(input), 367);
  }
}
