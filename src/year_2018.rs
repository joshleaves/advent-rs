//! Year 2018
//!
#![doc = include_str!("../NOTES_2018.md")]

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn solve(day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  if part > 2 {
    return None;
  }

  match day {
    1 => Some(day_01::day_01(part, input).to_string()),
    2 => Some(day_02::day_02(part, input).to_string()),
    3 => Some(day_03::day_03(part, input).to_string()),
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

  #[test]
  fn day_02() {
    let input = include_str!("../inputs/year_2018/day_02_input");
    assert_eq!(day_02::day_02_v1(input), "7872");
    assert_eq!(day_02::day_02_v2(input), "tjxmoewpdkyaihvrndfluwbzc");
  }

  #[test]
  fn day_03() {
    let input = include_str!("../inputs/year_2018/day_03_input");
    assert_eq!(day_03::day_03_v1(input), 101_565);
    assert_eq!(day_03::day_03_v2(input), 656);
  }
}
