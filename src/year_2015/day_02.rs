//! Advent of Code 2015: Day 2: I Was Told There Would Be No Math
//!
//! # Original exercise:
//! > ## --- Day 2: I Was Told There Would Be No Math ---
//! > The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.
//! >
//! > Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is `2*l*w + 2*w*h + 2*h*l`. The elves also need a little extra paper for each present: the area of the smallest side.
//! >
//! > For example:
//! >
//! > - A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square feet of wrapping paper plus `6` square feet of slack, for a total of `58` square feet.
//! > - A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square feet of wrapping paper plus `1` square foot of slack, for a total of `43` square feet.
//! >
//! > All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//! >
//! > ## --- Part Two ---
//! > The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.
//! >
//! > The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.
//! >
//! > For example:
//! >
//! > - A present with dimensions `2x3x4` requires `2+2+3+3 = 10` feet of ribbon to wrap the present plus `2*3*4 = 24` feet of ribbon for the bow, for a total of `34` feet.
//! > - A present with dimensions `1x1x10` requires `1+1+1+1 = 4` feet of ribbon to wrap the present plus `1*1*10 = 10` feet of ribbon for the bow, for a total of `14` feet.
//! >
//! > How many total feet of ribbon should they order?
//! >
//! > Your puzzle answer was ~~`REDACTED`~~.
//!

#[derive(Debug, PartialEq)]
struct PresentBox {
  height: u16,
  width: u16,
  length: u16,
}

impl PresentBox {
  fn new(input: &str) -> Self {
    let mut nums: Vec<u16> = input
      .split('x')
      .map(|num| num.parse::<u16>().unwrap())
      .collect();
    nums.sort();

    PresentBox {
      height: nums[0],
      width: nums[1],
      length: nums[2],
    }
  }

  fn area_small(&self) -> u16 {
    self.height * self.width
  }

  fn area_med(&self) -> u16 {
    self.width * self.length
  }

  fn area_large(&self) -> u16 {
    self.length * self.height
  }

  fn wrapper(&self) -> u16 {
    (3 * self.area_small()) + (2 * self.area_med()) + (2 * self.area_large())
  }

  fn ribbon(&self) -> u16 {
    (self.height * 2) + (self.width * 2) + (self.area_small() * self.length)
  }
}

pub fn day_02_v1(input: impl Into<String>) -> u32 {
  input
    .into()
    .lines()
    .map(|line| PresentBox::new(line).wrapper() as u32)
    .sum()
}

pub fn day_02_v2(input: impl Into<String>) -> u32 {
  input
    .into()
    .lines()
    .map(|line| PresentBox::new(line).ribbon() as u32)
    .sum()
}

solvable!(day_02, day_02_v1, day_02_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 2] = [
      ("2x3x4", 58),
      ("1x1x10", 43),
    ];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_02_v1(*sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, u32); 2] = [
      ("2x3x4", 34),
      ("1x1x10", 14),
    ];

    for (sample, result) in sample_two.iter() {
      assert_eq!(day_02_v2(*sample), *result);
    }
  }
}
