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
    1 => return Some(format!("{}", day_01::day_01(part, input))),
    2 => return Some(format!("{}", day_02::day_02(part, input))),
    3 => return Some(format!("{}", day_03::day_03(part, input))),
    4 => return Some(format!("{}", day_04::day_04(part, input))),
    5 => return Some(format!("{}", day_05::day_05(part, input))),
    6 => return Some(format!("{}", day_06::day_06(part, input))),
    7 => return Some(format!("{}", day_07::day_07(part, input))),
    8 => return Some(format!("{}", day_08::day_08(part, input))),
    9 => return Some(format!("{}", day_09::day_09(part, input))),
    10 => return Some(format!("{}", day_10::day_10(part, input))),
    11 => return Some(format!("{}", day_11::day_11(part, input))),
    12 => return Some(format!("{}", day_12::day_12(part, input))),
    13 => return Some(format!("{}", day_13::day_13(part, input))),
    14 => return Some(format!("{}", day_14::day_14(part, input))),
    15 => return Some(format!("{}", day_15::day_15(part, input))),
    16 => return Some(format!("{}", day_16::day_16(part, input))),
    _ => return None,
  }
}
