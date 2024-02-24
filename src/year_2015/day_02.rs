use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct PresentBox {
  height: u32,
  width: u32,
  length: u32,
}

impl PresentBox {
  fn area_small(&self) -> u32 {
    self.height * self.width
  }

  fn area_med(&self) -> u32 {
    self.width * self.length
  }

  fn area_large(&self) -> u32 {
    self.length * self.height
  }

  fn wrapper(&self) -> u32 {
    (3 * self.area_small()) + (2 * self.area_med()) + (2 * self.area_large())
  }

  fn ribbon(&self) -> u32 {
    (self.height * 2) + (self.width * 2) + (self.height * self.width * self.length)
  }
}

impl FromStr for PresentBox {
  type Err = std::num::ParseIntError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let num_str: Vec<&str> = input.split("x").collect();
    let mut num_int: [u32; 3] = [
      num_str[0].parse::<u32>().unwrap(),
      num_str[1].parse::<u32>().unwrap(),
      num_str[2].parse::<u32>().unwrap(),
    ];
    num_int.sort();

    Ok(PresentBox {
      height: num_int[0],
      width: num_int[1],
      length: num_int[2],
    })
  }
}

pub fn day_02_v1(input: &str) -> u32 {
  let mut total = 0;
  for line in input.lines() {
    match PresentBox::from_str(line) {
      Ok(present) => total += present.wrapper(),
      Err(_) => {}
    }
  }
  total
}

pub fn day_02_v2(input: &str) -> u32 {
  let mut total = 0;
  for line in input.lines() {
    match PresentBox::from_str(line) {
      Ok(present) => total += present.ribbon(),
      Err(_) => {}
    }
  }
  total
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 2] = [("2x3x4", 58), ("1x1x10", 43)];
    for (sample, result) in sample_one.iter() {
      assert_eq!(day_02_v1(sample), *result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, u32); 2] = [("2x3x4", 34), ("1x1x10", 14)];

    for (sample, result) in sample_two.iter() {
      assert_eq!(day_02_v2(sample), *result);
    }
  }
}
