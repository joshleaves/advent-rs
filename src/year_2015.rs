//! Year 2015
//!
//! ```text
//!                         |                               
//!                        \|/                              
//!                       --*--                        25 **
//!                        >o<                         24 **
//!                       >>@<<                        23 **
//!                      >>o>*<<                       22 **
//!                     >@<<<o>@<                      21 **
//!                    >@<<<*<<<O<                     20 **
//!                   >>o<<<O<<o<<<                    19 **
//!                  >o<@<@<<o<@>>@<                   18 **
//!                 >>O>>o>>O>*>>@<<<                  17 **
//!                >O>*>o>>O<O<O>>o<<<                 16 **
//!               >>o<<<@>*<*<<<O>*<@<<                15 **
//!              >>o>O<<*>*>>>o>*<<<*<<<               14 **
//!             >o<O<<O<<<o>>O<<<*<<<o<<<              13 **
//!            >*>O>*>@>o<<*<@>>>O<<<@<<<<             12 **
//!           >>O<@<<O>>>o>>>@<<<O>@>o>>>O<            11 **
//!          >*>>>*<<O>>>o>>@<<@<<@<@<O>>@<<           10 **
//!         >@>>>@<@>>>o<@<<<@>@>@<o>>>O>>@<<           9 **
//!        >>@<<<O<o>o<<<@<<O<<<@>o<<<O>>*<<*<          8 **
//!       >O<<*>>>O<o<<<@>>>@<<o<O>O<<o>*<<<*<<         7 **
//!      >>@>>>o>>>o<*<O<<<*<<<O<<@>O<<@<O>>>@<<        6 **
//!     >>*<O<O>>O<<<@<@>>>*>>o<@>*<<O>>O>O<<o>o<       5 **
//!    >>O>>*<<@<<<O>>>*>>O>>o>*>@>>>o<<<O<o<<<@<<      4 **
//!   >*<<<o>>@<<o<<<*<<*>*<<<o<o<<<*>>@<<<*>>>*<<<     3 **
//!  >>O<<O>>>O>>>@>@<<o<@<<<O>>o>>>o>@>>o<<<@>>O<O<    2 **
//! >@>>>@>>o>>@>@>>>O>o>o>>>*<<<@>@<<<*>@>o<<<o>>O<<   1 **
//!                       |   |                             
//!                       |   |                             
//!            _  _ __ ___|___|___ __ _  _                  
//! ```
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

/// Returns the solution for a specified exercise and input.
///
/// # Arguments
/// * `day` - The day of the exercise, from 1 to 25.
/// * `part` - The part of the exercise, 1 or 2.
/// * `input` - The input to the exercise.
///
/// # Examples
/// Missing day/part exercise
/// ```
/// use advent_rs::year_2015::solve;
///
/// assert_eq!(solve(1, 3, ""), None);
/// assert_eq!(solve(26, 1, ""), None);
/// ```
///
/// Available day/part exercise returns a String
/// ```
/// use advent_rs::year_2015::solve;
///
/// let solution = solve(1, 1, "(())");
/// assert_eq!(solution, Some("0".to_string()));
/// ```
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
    10 => Some(format!("{}", day_10::day_10(part, input))),
    11 => Some(format!("{}", day_11::day_11(part, input))),
    12 => Some(format!("{}", day_12::day_12(part, input))),
    13 => Some(format!("{}", day_13::day_13(part, input))),
    14 => Some(format!("{}", day_14::day_14(part, input))),
    15 => Some(format!("{}", day_15::day_15(part, input))),
    16 => Some(format!("{}", day_16::day_16(part, input))),
    17 => Some(format!("{}", day_17::day_17(part, input))),
    18 => Some(format!("{}", day_18::day_18(part, input))),
    19 => Some(format!("{}", day_19::day_19(part, input))),
    20 => Some(format!("{}", day_20::day_20(part, input))),
    21 => Some(format!("{}", day_21::day_21(part, input))),
    22 => Some(format!("{}", day_22::day_22(part, input))),
    23 => Some(format!("{}", day_23::day_23(part, input))),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_01() {
    let input = include_str!("../inputs/year_2015_day_01_input");
    assert_eq!(day_01::day_01_v1(input), 138);
    assert_eq!(day_01::day_01_v2(input), 1771);
  }

  #[test]
  fn day_02() {
    let input = include_str!("../inputs/year_2015_day_02_input");
    assert_eq!(day_02::day_02_v1(input), 1_588_178);
    assert_eq!(day_02::day_02_v2(input), 3_783_758);
  }

  #[test]
  fn day_03() {
    let input = include_str!("../inputs/year_2015_day_03_input");
    assert_eq!(day_03::day_03_v1(input), 2081);
    assert_eq!(day_03::day_03_v2(input), 2341);
  }

  #[test]
  fn day_04() {
    let input = include_str!("../inputs/year_2015_day_04_input");
    assert_eq!(day_04::day_04_v1(input), 346_386);
    assert_eq!(day_04::day_04_v2(input), 9_958_218);
  }

  #[test]
  fn day_05() {
    let input = include_str!("../inputs/year_2015_day_05_input");
    assert_eq!(day_05::day_05_v1(input), 238);
    assert_eq!(day_05::day_05_v2(input), 69);
  }

  #[test]
  fn day_06() {
    let input = include_str!("../inputs/year_2015_day_06_input");
    assert_eq!(day_06::day_06_v1(input), 400_410);
    assert_eq!(day_06::day_06_v2(input), 15_343_601);
  }

  #[test]
  fn day_07() {
    let input = include_str!("../inputs/year_2015_day_07_input");
    assert_eq!(day_07::day_07_v1(input), 46_065);
    assert_eq!(day_07::day_07_v2(input), 14_134);
  }

  #[test]
  fn day_08() {
    let input = include_str!("../inputs/year_2015_day_08_input");
    assert_eq!(day_08::day_08_v1(input), 1_333);
    assert_eq!(day_08::day_08_v2(input), 2_046);
  }

  #[test]
  fn day_09() {
    let input = include_str!("../inputs/year_2015_day_09_input");
    assert_eq!(day_09::day_09_v1(input), 117);
    assert_eq!(day_09::day_09_v2(input), 909);
  }
  #[test]
  fn day_10() {
    let input = include_str!("../inputs/year_2015_day_10_input");
    assert_eq!(day_10::day_10_v1(input), 252_594);
    assert_eq!(day_10::day_10_v2(input), 3_579_328);
  }

  #[test]
  fn day_11() {
    let input = include_str!("../inputs/year_2015_day_11_input");
    assert_eq!(day_11::day_11_v1(input), "vzbxxyzz");
    assert_eq!(day_11::day_11_v2(input), "vzcaabcc");
  }

  #[test]
  fn day_12() {
    let input = include_str!("../inputs/year_2015_day_12_input");
    assert_eq!(day_12::day_12_v1(input), 111_754);
    assert_eq!(day_12::day_12_v2(input), 65_402);
  }

  #[test]
  fn day_13() {
    let input = include_str!("../inputs/year_2015_day_13_input");
    assert_eq!(day_13::day_13_v1(input), 709);
    assert_eq!(day_13::day_13_v2(input), 668);
  }

  #[test]
  fn day_14() {
    let input = include_str!("../inputs/year_2015_day_14_input");
    assert_eq!(day_14::day_14_v1(input), 2655);
    assert_eq!(day_14::day_14_v2(input), 1059);
  }

  #[test]
  fn day_15() {
    let input = include_str!("../inputs/year_2015_day_15_input");
    assert_eq!(day_15::day_15_v1(input), 222_870);
    assert_eq!(day_15::day_15_v2(input), 117_936);
  }

  #[test]
  fn day_16() {
    let input = include_str!("../inputs/year_2015_day_16_input");
    assert_eq!(day_16::day_16_v1(input), 373);
    assert_eq!(day_16::day_16_v2(input), 260);
  }

  #[test]
  fn day_17() {
    let input = include_str!("../inputs/year_2015_day_17_input");
    assert_eq!(day_17::day_17_v1(input), 1638);
    assert_eq!(day_17::day_17_v2(input), 17);
  }

  #[test]
  fn day_18() {
    let input = include_str!("../inputs/year_2015_day_18_input");
    assert_eq!(day_18::day_18_v1(input), 821);
    assert_eq!(day_18::day_18_v2(input), 886);
  }

  #[test]
  fn day_19() {
    let input = include_str!("../inputs/year_2015_day_19_input");
    assert_eq!(day_19::day_19_v1(input), 576);
    assert_eq!(day_19::day_19_v2(input), 207);
  }

  #[test]
  fn day_20() {
    let input = include_str!("../inputs/year_2015_day_20_input");
    assert_eq!(day_20::day_20_v1(input), 831_600);
    assert_eq!(day_20::day_20_v2(input), 884_520);
  }

  #[test]
  fn day_21() {
    let input = include_str!("../inputs/year_2015_day_21_input");
    assert_eq!(day_21::day_21_v1(input), 91);
    assert_eq!(day_21::day_21_v2(input), 158);
  }

  #[test]
  fn day_22() {
    let input = include_str!("../inputs/year_2015_day_22_input");
    assert_eq!(day_22::day_22_v1(input), 953);
    assert_eq!(day_22::day_22_v2(input), 1289);
  }

  #[test]
  fn day_23() {
    let input = include_str!("../inputs/year_2015_day_23_input");
    assert_eq!(day_23::day_23_v1(input), 307);
    assert_eq!(day_23::day_23_v2(input), 160);
  }
}
