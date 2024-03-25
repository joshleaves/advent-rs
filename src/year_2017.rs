//! Year 2017
//!
#![doc = include_str!("../NOTES_2017.md")]

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

pub fn solve(day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  if part != 1 && part != 2 {
    return None;
  }

  match day {
    1 => Some(day_01::day_01(part, input).to_string()),
    2 => Some(day_02::day_02(part, input).to_string()),
    3 => Some(day_03::day_03(part, input).to_string()),
    4 => Some(day_04::day_04(part, input).to_string()),
    5 => Some(day_05::day_05(part, input).to_string()),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_01() {
    let input = include_str!("../inputs/year_2017/day_01_input");
    assert_eq!(day_01::day_01_v1(input), 1_069);
    assert_eq!(day_01::day_01_v2(input), 1_268);
  }

  #[test]
  fn day_02() {
    let input = include_str!("../inputs/year_2017/day_02_input");
    assert_eq!(day_02::day_02_v1(input), 53_978);
    assert_eq!(day_02::day_02_v2(input), 314);
  }

  #[test]
  fn day_03() {
    let input = include_str!("../inputs/year_2017/day_03_input");
    assert_eq!(day_03::day_03_v1(input), 552);
    assert_eq!(day_03::day_03_v2(input), 330_785);
  }

  #[test]
  fn day_04() {
    let input = include_str!("../inputs/year_2017/day_04_input");
    assert_eq!(day_04::day_04_v1(input), 466);
    assert_eq!(day_04::day_04_v2(input), 251);
  }

  #[test]
  fn day_05() {
    let input = include_str!("../inputs/year_2017/day_05_input");
    assert_eq!(day_05::day_05_v1(input), 373_160);
    assert_eq!(day_05::day_05_v2(input), 26_395_586);
  }
}
