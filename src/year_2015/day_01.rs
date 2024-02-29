//! Advent of Code 2015: Day 1: Not Quite Lisp
//!
//! # Original exercise:
//! > ## --- Day 1: Not Quite Lisp ---
//! > Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.
//! >
//! > Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//! >
//! > Here's an easy puzzle to warm you up.
//! >
//! > Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.
//! >
//! > An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.
//! >
//! > The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
//! >
//! > For example:
//! >
//! > - `(())` and `()()` both result in floor `0`.
//! > - `(((` and `(()(()(` both result in floor `3`.
//! > - `))(((((` also results in floor `3`.
//! > - `())` and `))(` both result in floor `-1` (the first basement level).
//! > - `)))` and `)())())` both result in floor `-3`.
//! >
//! > To what floor do the instructions take Santa?
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//! >
//! > ## --- Part Two ---
//! > Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.
//! >
//! > For example:
//! >
//! > - `)` causes him to enter the basement at character position `1`.
//! > - `()())` causes him to enter the basement at character position `5`.
//! >
//! > What is the position of the character that causes Santa to first enter the basement?
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.

/// Solve exercise for year 2015, day 1 (part 1).
///
/// This exercise requires counting all `(` as Santa moving up one level and all
/// `)` as Santa moving down one level, then return Santa's final level.
///
/// # Naive implementation:
/// - Match all instances of `(` and count them as `up`.
/// - Match all instances of `)` and count them as `down`.
/// - Subtract `down` from `up` to get `level`.
/// - Return `level`.
///
/// Drawback: The code is iterating the string twice, and the matcher may not be
/// optimized properly for our input.
///
/// # Regular implementation:
/// - Start with a mutable `level` equal to `0`.
/// - Iterate over each character of the string.
/// - If the char is a `(`, add `1` to `level`.
/// - If the char is a `)`, substract `1` from `level`.
/// - Return `level`.
///
/// Advantage: The code is iterating over the string only once.
///
/// # Fast and furious implementation:
/// - Start with a mutable `level` equal to `0`.
/// - Treat the string as a `Vec<u16>`. Iterating over it now gives a compounded
///   value of two characters.
/// - Increase/decrease `level` based on the value of the compounded characters.
/// - If we get any unknown value (most likely an empty char), we return early.
/// - Return `level`.
///
/// Note: Further optimization CAN be done by treating the string as `Vec<u32>`,
/// but the matcher would become too complicated to maintain.
///
/// # Examples
/// ```
/// use advent_rs::year_2015::day_01;
///
/// assert_eq!(day_01::day_01_v1("("), 1);
/// assert_eq!(day_01::day_01_v1(")"), -1);
/// assert_eq!(day_01::day_01_v1("(("), 2);
/// assert_eq!(day_01::day_01_v1("))"), -2);
/// assert_eq!(day_01::day_01_v1("()"), 0);
/// assert_eq!(day_01::day_01_v1(")("), 0);
///
/// let input_a = "))(((((";
/// let solution_a = day_01::day_01_v1(input_a);
/// assert_eq!(solution_a, 3);
///
/// let input_b = ")())())";
/// let solution_b = day_01::day_01_v1(input_b);
/// assert_eq!(solution_b, -3);
/// ```
pub fn day_01_v1(input: impl Into<String>) -> i16 {
  let mut lvl: i16 = 0;
  for chr in input.into().chars() {
    match chr {
      '(' => lvl += 1,
      ')' => lvl -= 1,
      _ => panic!("Invalid character: {}", chr),
    }
  }

  lvl
}

/// Solve exercise for year 2015, day 1 (part 2).
///
/// This exercise uses the same rules for moving up and down levels, but we have
/// to return the moment where Santa will reach the "basement".
///
/// # Regular implementation:
/// - Start with a mutable `level` equal to `0`.
/// - Iterate over each character of the string.
/// - If the char is a `(`, add `1` to `level`.
/// - If the char is a `)`, substract `1` from `level`.
/// - If `level` equals `-1`, return `index` plus `1` (`index` started at `0`).
///
/// # Fast and furious implementation:
/// - Start with a mutable `level` equal to `0`.
/// - Treat the string as a `Vec<u16>`. Iterating over it now gives a compounded
///   value of two characters.
/// - Enumerate `(index, chrs)` over the string.
/// - Increase/decrease `level` based on the value of the compounded characters.
/// - If we get any unknown value (most likely an empty char), we return early.
/// - If `level` reaches `-1`, return current `index` multipled by `2` (compound
///   size) and add `1` (because `index` started at `0`).
///
/// Advantage: Impossible to use the "Naive" implementation, and "Normal" is not
/// a huge time loss either.
///
/// # Examples:
/// ```
/// use advent_rs::year_2015::day_01;
///
/// assert_eq!(day_01::day_01_v2("("), 0);
/// assert_eq!(day_01::day_01_v2(")"), 1);
/// assert_eq!(day_01::day_01_v2("(("), 0);
/// assert_eq!(day_01::day_01_v2("))"), 1);
/// assert_eq!(day_01::day_01_v2("()"), 0);
/// assert_eq!(day_01::day_01_v2(")("), 1);
///
/// let input = "()())";
/// let solution = day_01::day_01_v2(input);
/// assert_eq!(solution, 5);
/// ```
pub fn day_01_v2(input: impl Into<String>) -> i16 {
  let mut lvl: i16 = 0;
  for (idx, chr) in input.into().chars().enumerate() {
    match chr {
      '(' => lvl += 1,
      ')' => lvl -= 1,
      _ => panic!("Invalid character: {}", chr),
    }
    if lvl < 0 {
      return idx as i16 + 1;
    }
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, i16); 9] = [
      ("(())", 0),
      ("()()", 0),
      ("(((", 3),
      ("(()(()(", 3),
      ("))(((((", 3),
      ("())", -1),
      ("))(", -1),
      (")))", -3),
      (")())())", -3),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_01_v1(*sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, i16); 2] = [
      (")", 1),
      ("()())", 5),
    ];
    for (sample, result) in sample_two.iter() {
      assert_eq!(day_01_v2(*sample), *result);
    }
  }
}
