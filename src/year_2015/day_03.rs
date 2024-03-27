//! Advent of Code 2015: Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! # Original exercise:
//! > ## --- Day 3: Perfectly Spherical Houses in a Vacuum ---
//! > Santa is delivering presents to an infinite two-dimensional grid of houses.
//! >
//! > He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (`^`), south (`v`), east (`>`), or west (`<`). After each move, he delivers another present to the house at his new location.
//! >
//! > However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?
//! >
//! > For example:
//! >
//! > - `>` delivers presents to `2` houses: one at the starting location, and one to the east.
//! > - `^>v<` delivers presents to `4` houses in a square, including twice to the house at his starting/ending location.
//! > - `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only `2` houses.
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//! >
//! > ## --- Part Two ---
//! > The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.
//! >
//! > Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.
//! >
//! > This year, how many houses receive at least one present?
//! >
//! > For example:
//! >
//! > - `^v` delivers presents to `3` houses, because Santa goes north, and then Robo-Santa goes south.
//! > - `^>v<` now delivers presents to `3` houses, and Santa and Robo-Santa end up back where they started.
//! > - `^v^v^v^v^v` now delivers presents to `11` houses, with Santa going one direction and Robo-Santa going the other.
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//!

use itertools::Itertools;
use std::collections::HashSet;

#[inline]
fn move_character(pos: &mut (i8, i8), direction: u8) {
  match direction {
    b'>' => pos.0 += 1,
    b'<' => pos.0 -= 1,
    b'v' => pos.1 += 1,
    b'^' => pos.1 -= 1,
    _ => panic!("Invalid direction character: {direction}"),
  }
  // pos
}

pub fn day_03_v1(input: impl Into<String>) -> u16 {
  let mut santa: (i8, i8) = (0, 0);
  let mut houses = HashSet::from([santa]);

  for direction in input.into().bytes() {
    move_character(&mut santa, direction);
    houses.insert(santa);
  }

  houses.len() as u16
}

pub fn day_03_v2(input: impl Into<String>) -> u16 {
  let mut santa: (i8, i8) = (0, 0);
  let mut robot: (i8, i8) = (0, 0);
  let mut houses = HashSet::from([santa]);

  for mut chars in &input.into().bytes().chunks(2) {
    move_character(&mut santa, chars.next().unwrap());
    move_character(&mut robot, chars.next().unwrap());
    houses.insert(santa);
    houses.insert(robot);
  }

  houses.len() as u16
}

solvable!(day_03, day_03_v1, day_03_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u16); 3] = [
      (">", 2),
      ("^>v<", 4),
      ("^v^v^v^v^v", 2),
    ];

    for (sample, result) in sample_one.iter() {
      assert_eq!(day_03_v1(*sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, u16); 3] = [
      ("^v", 3),
      ("^>v<", 3),
      ("^v^v^v^v^v", 11),
    ];

    for (sample, result) in sample_two.iter() {
      assert_eq!(day_03_v2(*sample), *result);
    }
  }
}
