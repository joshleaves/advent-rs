#![doc = include_str!("../NOTES_2016.md")]

//! Year 2016
//!
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
    1 => Some(format!("{}", day_01::day_01(part, input))),
    2 => Some(format!("{}", day_02::day_02(part, input))),
    3 => Some(format!("{}", day_03::day_03(part, input))),
    4 => Some(format!("{}", day_04::day_04(part, input))),
    5 => Some(format!("{}", day_05::day_05(part, input))),
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

  #[test]
  fn day_03() {
    let input = include_str!("../inputs/year_2016/day_03_input");
    assert_eq!(day_03::day_03_v1(input), 993);
    assert_eq!(day_03::day_03_v2(input), 1849);
  }

  #[test]
  fn day_04() {
    let input = include_str!("../inputs/year_2016/day_04_input");
    assert_eq!(day_04::day_04_v1(input), 158_835);
    assert_eq!(day_04::day_04_v2(input), 993);
  }

  #[test]
  fn day_05() {
    let input = include_str!("../inputs/year_2016/day_05_input");
    assert_eq!(day_05::day_05_v1(input), "4543c154");
    assert_eq!(day_05::day_05_v2(input), "1050cbbd");
  }
}
