#![doc = include_str!("../NOTES_2016.md")]

//! Year 2016
//!
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;

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
    6 => Some(format!("{}", day_06::day_06(part, input))),
    7 => Some(format!("{}", day_07::day_07(part, input))),
    8 => Some(format!("{}", day_08::day_08(part, input))),
    9 => Some(format!("{}", day_09::day_09(part, input))),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

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
    if env::var("CI").is_err() {
      assert_eq!(day_05::day_05_v2(input), "1050cbbd");
    }
  }

  #[test]
  fn day_06() {
    let input = include_str!("../inputs/year_2016/day_06_input");
    assert_eq!(day_06::day_06_v1(input), "zcreqgiv");
    assert_eq!(day_06::day_06_v2(input), "pljvorrk");
  }

  #[test]
  fn day_07() {
    let input = include_str!("../inputs/year_2016/day_07_input");
    assert_eq!(day_07::day_07_v1(input), 118);
    assert_eq!(day_07::day_07_v2(input), 260);
  }

  #[test]
  fn day_08() {
    let input = include_str!("../inputs/year_2016/day_08_input");
    assert_eq!(day_08::day_08_v1(input), 106);
    const CFLELOYFCS: &str = ".##..####.#....####.#.....##..#...#####..##...###.\n\
      #..#.#....#....#....#....#..#.#...##....#..#.#....\n\
      #....###..#....###..#....#..#..#.#.###..#....#....\n\
      #....#....#....#....#....#..#...#..#....#.....##..\n\
      #..#.#....#....#....#....#..#...#..#....#..#....#.\n\
      .##..#....####.####.####..##....#..#.....##..###..";
    assert_eq!(day_08::day_08_v2(input), CFLELOYFCS);
  }

  #[test]
  fn day_09() {
    let input = include_str!("../inputs/year_2016/day_09_input");
    assert_eq!(day_09::day_09_v1(input), 183_269);
    assert_eq!(day_09::day_09_v2(input), 11_317_278_863);
  }

  #[test]
  fn day_10() {
    let input = include_str!("../inputs/year_2016/day_10_input");
    assert_eq!(day_10::day_10_v1(input), 157);
    assert_eq!(day_10::day_10_v2(input), 1085);
  }
}
