#![doc = include_str!("../README.md")]

macro_rules! solvable {
  ($day_xx:tt, $day_v1:tt, $day_v2:tt, $retype:ty) => {
    /// Stub function calling the _v1 or _v2 variant
    pub fn $day_xx(part: u8, input: impl Into<String>) -> $retype {
      match part {
        1 => $day_v1(input),
        2 => $day_v2(input),
        _ => panic!("Invalid part: {}", part),
      }
    }
  };
}

mod bfs;
pub mod year_2015;
pub mod year_2016;
pub mod year_2017;

/// Returns the solution for a specified exercise and input.
///
/// # Arguments
/// * `year` - The year of the exercise, from 2015.
/// * `day` - The day of the exercise, from 1 to 25.
/// * `part` - The part of the exercise, 1 or 2.
/// * `input` - The input to the exercise.
///
/// # Examples
/// Missing year/day/part exercise
/// ```
/// use advent_rs::solve;
/// assert_eq!(solve(2014, 1, 1, ""), None);
/// assert_eq!(solve(2015, 1, 3, ""), None);
/// assert_eq!(solve(2015, 26, 1, ""), None);
/// ```
///
/// Available year/day/part exercise returns a String
/// ```
/// use advent_rs::solve;
/// let solution = solve(2015, 1, 1, "(())");
/// assert_eq!(solution, Some("0".to_string()));
/// ```
pub fn solve(year: u16, day: u8, part: u8, input: impl Into<String>) -> Option<String> {
  match year {
    2015 => year_2015::solve(day, part, input),
    2016 => year_2016::solve(day, part, input),
    2017 => year_2017::solve(day, part, input),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn solve_for_invalid_year() {
    assert_eq!(solve(2014, 1, 1, "".to_string()), None);
  }

  #[test]
  fn solve_for_year_2015_day_01v1() {
    assert_eq!(solve(2015, 1, 1, "(())".to_string()), Some("0".to_string()));
  }
}
