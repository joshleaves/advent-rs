#![doc = include_str!("../NOTES_2016.md")]

//! Year 2016
//!
pub mod day_01;
pub mod day_02;

pub fn solve(day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  if part != 1 && part != 2 {
    return None;
  }
  match day {
    1 => Some(format!("{}", day_01::day_01(part, input))),
    2 => Some(format!("{}", day_02::day_02(part, input))),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_01() {
    let input = include_str!("../inputs/year_2016/day_01_input");
    assert_eq!(day_01::day_01_v1(input), 146);
    assert_eq!(day_01::day_01_v2(input), 131);
  }

  #[test]
  fn day_02() {
    let input = include_str!("../inputs/year_2016/day_02_input");
    assert_eq!(day_02::day_02_v1(input), "45973");
    assert_eq!(day_02::day_02_v2(input), "27CA4");
  }
}
