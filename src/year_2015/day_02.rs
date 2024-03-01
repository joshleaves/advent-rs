use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct PresentBox {
  height: u8,
  width: u8,
  length: u8,
}

impl PresentBox {
  fn area_small(&self) -> u16 {
    self.height as u16 * self.width as u16
  }

  fn area_med(&self) -> u16 {
    self.width as u16 * self.length as u16
  }

  fn area_large(&self) -> u16 {
    self.length as u16 * self.height as u16
  }

  fn wrapper(&self) -> u16 {
    (3 * self.area_small()) + (2 * self.area_med()) + (2 * self.area_large())
  }

  fn ribbon(&self) -> u16 {
    (self.height as u16 * 2) + (self.width as u16 * 2) + (self.area_small() * self.length as u16)
  }
}

impl FromStr for PresentBox {
  type Err = std::num::ParseIntError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let num_str: Vec<&str> = input.split("x").collect();
    let mut num_int: [u8; 3] = [
      num_str[0].parse::<u8>().unwrap(),
      num_str[1].parse::<u8>().unwrap(),
      num_str[2].parse::<u8>().unwrap(),
    ];
    num_int.sort();

    Ok(PresentBox {
      height: num_int[0],
      width: num_int[1],
      length: num_int[2],
    })
  }
}

pub fn day_02_v1(input: impl Into<String>) -> u32 {
  let mut total: u32 = 0;
  for line in input.into().lines() {
    match PresentBox::from_str(line) {
      Ok(present) => total += present.wrapper() as u32,
      Err(_) => {}
    }
  }
  total
}

pub fn day_02_v2(input: impl Into<String>) -> u32 {
  let mut total: u32 = 0;
  for line in input.into().lines() {
    match PresentBox::from_str(line) {
      Ok(present) => total += present.ribbon() as u32,
      Err(_) => {}
    }
  }
  total
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
