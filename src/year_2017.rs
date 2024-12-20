//! Year 2017
//!
#![doc = include_str!("../NOTES_2017.md")]

mod knot_hash;

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

  #[test]
  fn day_06() {
    let input = include_str!("../inputs/year_2017/day_06_input");
    assert_eq!(day_06::day_06_v1(input), 11_137);
    assert_eq!(day_06::day_06_v2(input), 1_037);
  }

  #[test]
  fn day_07() {
    let input = include_str!("../inputs/year_2017/day_07_input");
    assert_eq!(day_07::day_07_v1(input), "ykpsek");
    assert_eq!(day_07::day_07_v2(input), "1060");
  }

  #[test]
  fn day_08() {
    let input = include_str!("../inputs/year_2017/day_08_input");
    assert_eq!(day_08::day_08_v1(input), 4_163);
    assert_eq!(day_08::day_08_v2(input), 5_347);
  }

  #[test]
  fn day_09() {
    let input = include_str!("../inputs/year_2017/day_09_input");
    assert_eq!(day_09::day_09_v1(input), 17_537);
    assert_eq!(day_09::day_09_v2(input), 7_539);
  }

  #[test]
  fn day_10() {
    let input = include_str!("../inputs/year_2017/day_10_input");
    assert_eq!(day_10::day_10_v1(input), "6909");
    assert_eq!(day_10::day_10_v2(input), "9d5f4561367d379cfbf04f8c471c0095");
  }

  #[test]
  fn day_11() {
    let input = include_str!("../inputs/year_2017/day_11_input");
    assert_eq!(day_11::day_11_v1(input), 682);
    assert_eq!(day_11::day_11_v2(input), 1_406);
  }

  #[test]
  fn day_12() {
    let input = include_str!("../inputs/year_2017/day_12_input");
    assert_eq!(day_12::day_12_v1(input), 130);
    assert_eq!(day_12::day_12_v2(input), 189);
  }

  #[test]
  fn day_13() {
    let input = include_str!("../inputs/year_2017/day_13_input");
    assert_eq!(day_13::day_13_v1(input), 2_264);
    assert_eq!(day_13::day_13_v2(input), 3_875_838);
  }

  #[test]
  fn day_14() {
    let input = include_str!("../inputs/year_2017/day_14_input");
    assert_eq!(day_14::day_14_v1(input), 8_230);
    assert_eq!(day_14::day_14_v2(input), 1_103);
  }

  #[test]
  fn day_15() {
    let input = include_str!("../inputs/year_2017/day_15_input");
    assert_eq!(day_15::day_15_v1(input), 567);
    assert_eq!(day_15::day_15_v2(input), 323);
  }

  #[test]
  fn day_16() {
    let input = include_str!("../inputs/year_2017/day_16_input");
    assert_eq!(day_16::day_16_v1(input), "lgpkniodmjacfbeh");
    assert_eq!(day_16::day_16_v2(input), "hklecbpnjigoafmd");
  }

  #[test]
  fn day_17() {
    let input = include_str!("../inputs/year_2017/day_17_input");
    assert_eq!(day_17::day_17_v1(input), 1_173);
    assert_eq!(day_17::day_17_v2(input), 1_930_815);
  }

  #[test]
  fn day_18() {
    let input = include_str!("../inputs/year_2017/day_18_input");
    assert_eq!(day_18::day_18_v1(input), 1_187);
    assert_eq!(day_18::day_18_v2(input), 5_969);
  }

  #[test]
  fn day_19() {
    let input = include_str!("../inputs/year_2017/day_19_input");
    assert_eq!(day_19::day_19_v1(input), "RYLONKEWB");
    assert_eq!(day_19::day_19_v2(input), "16016");
  }

  #[test]
  fn day_20() {
    let input = include_str!("../inputs/year_2017/day_20_input");
    assert_eq!(day_20::day_20_v1(input), 157);
    assert_eq!(day_20::day_20_v2(input), 499);
  }

  #[test]
  fn day_21() {
    let input = include_str!("../inputs/year_2017/day_21_input");
    assert_eq!(day_21::day_21_v1(input), 179);
    assert_eq!(day_21::day_21_v2(input), 2_766_750);
  }

  #[test]
  fn day_22() {
    let input = include_str!("../inputs/year_2017/day_22_input");
    assert_eq!(day_22::day_22_v1(input), 5_246);
    assert_eq!(day_22::day_22_v2(input), 2_512_059);
  }

  #[test]
  fn day_23() {
    let input = include_str!("../inputs/year_2017/day_23_input");
    assert_eq!(day_23::day_23_v1(input), 6_724);
    assert_eq!(day_23::day_23_v2(input), 903);
  }

  #[test]
  fn day_24() {
    let input = include_str!("../inputs/year_2017/day_24_input");
    assert_eq!(day_24::day_24_v1(input), 1_906);
    assert_eq!(day_24::day_24_v2(input), 1_824);
  }

  #[test]
  fn day_25() {
    let input = include_str!("../inputs/year_2017/day_25_input");
    assert_eq!(day_25::day_25(input), 4_387);
  }
}
