//! Year 2016
//!
#![doc = include_str!("../NOTES_2016.md")]

mod assembunny;

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
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

pub fn solve(day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  if part > 2 {
    return None;
  }

  match day {
    1 => Some(day_01::day_01(part, input).to_string()),
    2 => Some(day_02::day_02(part, input).to_string()),
    3 => Some(day_03::day_03(part, input).to_string()),
    4 => Some(day_04::day_04(part, input).to_string()),
    5 => Some(day_05::day_05(part, input).to_string()),
    6 => Some(day_06::day_06(part, input).to_string()),
    7 => Some(day_07::day_07(part, input).to_string()),
    8 => Some(day_08::day_08(part, input).to_string()),
    9 => Some(day_09::day_09(part, input).to_string()),
    10 => Some(day_10::day_10(part, input).to_string()),
    11 => Some(day_11::day_11(part, input).to_string()),
    12 => Some(day_12::day_12(part, input).to_string()),
    13 => Some(day_13::day_13(part, input).to_string()),
    14 => Some(day_14::day_14(part, input).to_string()),
    15 => Some(day_15::day_15(part, input).to_string()),
    16 => Some(day_16::day_16(part, input).to_string()),
    17 => Some(day_17::day_17(part, input).to_string()),
    18 => Some(day_18::day_18(part, input).to_string()),
    19 => Some(day_19::day_19(part, input).to_string()),
    20 => Some(day_20::day_20(part, input).to_string()),
    21 => Some(day_21::day_21(part, input).to_string()),
    22 => Some(day_22::day_22(part, input).to_string()),
    23 => Some(day_23::day_23(part, input).to_string()),
    24 => Some(day_24::day_24(part, input).to_string()),
    25 => Some(day_25::day_25(input).to_string()),
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
  #[ignore = "Too slow for CI"]
  fn day_05() {
    let input = include_str!("../inputs/year_2016/day_05_input");
    assert_eq!(day_05::day_05_v1(input), "4543c154");
    assert_eq!(day_05::day_05_v2(input), "1050cbbd");
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

  #[test]
  #[ignore = "Too slow for CI"]
  fn day_11() {
    let input = include_str!("../inputs/year_2016/day_11_input");
    assert_eq!(day_11::day_11_v1(input), 47);
    assert_eq!(day_11::day_11_v2(input), 71);
  }

  #[test]
  fn day_12() {
    let input = include_str!("../inputs/year_2016/day_12_input");
    assert_eq!(day_12::day_12_v1(input), 318_007);
    assert_eq!(day_12::day_12_v2(input), 9_227_661);
  }

  #[test]
  fn day_13() {
    let input = include_str!("../inputs/year_2016/day_13_input");
    assert_eq!(day_13::day_13_v1(input), 90);
    assert_eq!(day_13::day_13_v2(input), 135);
  }

  #[test]
  #[ignore = "Too slow for CI"]
  fn day_14() {
    let input = include_str!("../inputs/year_2016/day_14_input");
    assert_eq!(day_14::day_14_v1(input), 25_427);
    assert_eq!(day_14::day_14_v2(input), 22_045);
  }

  #[test]
  fn day_15() {
    let input = include_str!("../inputs/year_2016/day_15_input");
    assert_eq!(day_15::day_15_v1(input), 203_660);
    assert_eq!(day_15::day_15_v2(input), 2_408_135);
  }

  #[test]
  fn day_16() {
    let input = include_str!("../inputs/year_2016/day_16_input");
    assert_eq!(day_16::day_16_v1(input), "10100011010101011");
    assert_eq!(day_16::day_16_v2(input), "01010001101011001");
  }

  #[test]
  fn day_17() {
    let input = include_str!("../inputs/year_2016/day_17_input");
    assert_eq!(day_17::day_17_v1(input), "RLDUDRDDRR");
    assert_eq!(day_17::day_17_v2(input), "590");
  }

  #[test]
  fn day_18() {
    let input = include_str!("../inputs/year_2016/day_18_input");
    assert_eq!(day_18::day_18_v1(input), 1926);
    assert_eq!(day_18::day_18_v2(input), 19_986_699);
  }

  #[test]
  fn day_19() {
    let input = include_str!("../inputs/year_2016/day_19_input");
    assert_eq!(day_19::day_19_v1(input), 1_830_117);
    assert_eq!(day_19::day_19_v2(input), 1_417_887);
  }

  #[test]
  fn day_20() {
    let input = include_str!("../inputs/year_2016/day_20_input");
    assert_eq!(day_20::day_20_v1(input), 4_793_564);
    assert_eq!(day_20::day_20_v2(input), 146);
  }

  #[test]
  fn day_21() {
    let input = include_str!("../inputs/year_2016/day_21_input");
    assert_eq!(day_21::day_21_v1(input), "gfdhebac");
    assert_eq!(day_21::day_21_v2(input), "dhaegfbc");
  }

  #[test]
  fn day_22() {
    let input = include_str!("../inputs/year_2016/day_22_input");
    assert_eq!(day_22::day_22_v1(input), 955);
    assert_eq!(day_22::day_22_v2(input), 246);
  }

  #[test]
  #[ignore = "Too slow for CI"]
  fn day_23() {
    let input = include_str!("../inputs/year_2016/day_23_input");
    assert_eq!(day_23::day_23_v1(input), 13_776);
    assert_eq!(day_23::day_23_v2(input), 479_010_336);
  }

  #[test]
  fn day_24() {
    let input = include_str!("../inputs/year_2016/day_24_input");
    assert_eq!(day_24::day_24_v1(input), 430);
    assert_eq!(day_24::day_24_v2(input), 700);
  }

  #[test]
  fn day_25() {
    let input = include_str!("../inputs/year_2016/day_25_input");
    assert_eq!(day_25::day_25(input), 192);
  }
}
