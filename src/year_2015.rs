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
  match (day, part) {
    (1, 1) => return Some(format!("{}", day_01::day_01_v1(input))),
    (1, 2) => return Some(format!("{}", day_01::day_01_v2(input))),
    (2, 1) => return Some(format!("{}", day_02::day_02_v2(input))),
    (2, 2) => return Some(format!("{}", day_02::day_02_v2(input))),
    (3, 1) => return Some(format!("{}", day_03::day_03_v2(input))),
    (3, 2) => return Some(format!("{}", day_03::day_03_v2(input))),
    (4, 1) => return Some(format!("{}", day_04::day_04_v2(input))),
    (4, 2) => return Some(format!("{}", day_04::day_04_v2(input))),
    (5, 1) => return Some(format!("{}", day_05::day_05_v2(input))),
    (5, 2) => return Some(format!("{}", day_05::day_05_v2(input))),
    (6, 1) => return Some(format!("{}", day_06::day_06_v2(input))),
    (6, 2) => return Some(format!("{}", day_06::day_06_v2(input))),
    (7, 1) => return Some(format!("{}", day_07::day_07_v2(input))),
    (7, 2) => return Some(format!("{}", day_07::day_07_v2(input))),
    (8, 1) => return Some(format!("{}", day_08::day_08_v1(input))),
    (8, 2) => return Some(format!("{}", day_08::day_08_v2(input))),
    (9, 1) => return Some(format!("{}", day_09::day_09_v1(input))),
    (9, 2) => return Some(format!("{}", day_09::day_09_v2(input))),
    (10, 1) => return Some(format!("{}", day_10::day_10_v1(input))),
    (10, 2) => return Some(format!("{}", day_10::day_10_v2(input))),
    (11, 1) => return Some(format!("{}", day_11::day_11_v1(input))),
    (11, 2) => return Some(format!("{}", day_11::day_11_v2(input))),
    (12, 1) => return Some(format!("{}", day_12::day_12_v1(input))),
    (12, 2) => return Some(format!("{}", day_12::day_12_v2(input))),
    (13, 1) => return Some(format!("{}", day_13::day_13_v1(input))),
    (13, 2) => return Some(format!("{}", day_13::day_13_v2(input))),
    (14, 1) => return Some(format!("{}", day_14::day_14_v1(input))),
    (14, 2) => return Some(format!("{}", day_14::day_14_v2(input))),
    _ => return None,
  }
}
