use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pattern {
  size: u16,
  data: Vec<Vec<bool>>,
}

impl Pattern {
  fn _split_2(&self) -> Vec<Self> {
    let mut bools = self.data.clone();
    let mut results = vec![];
    while !bools.is_empty() {
      let mut rows: Vec<_> = bools.drain(0..2).collect();
      while !rows[0].is_empty() {
        let result: Vec<Vec<bool>> = vec![
          rows[0].drain(0..2).collect(),
          rows[1].drain(0..2).collect(),
        ];
        results.push(result);
      }
    }
    results
      .iter()
      .map(|result| Pattern {
        size: 2,
        data: result.clone(),
      })
      .collect()
  }

  fn _split_3(&self) -> Vec<Self> {
    let mut bools = self.data.clone();
    let mut results = vec![];
    while !bools.is_empty() {
      let mut rows: Vec<_> = bools.drain(0..3).collect();
      while !rows[0].is_empty() {
        let result: Vec<Vec<bool>> = vec![
          rows[0].drain(0..3).collect(),
          rows[1].drain(0..3).collect(),
          rows[2].drain(0..3).collect(),
        ];
        results.push(result);
      }
    }
    results
      .iter()
      .map(|result| Pattern {
        size: 3,
        data: result.clone(),
      })
      .collect()
  }

  pub fn split(self) -> (u16, Vec<Self>) {
    if self.size.is_multiple_of(2) {
      (2, self._split_2())
    } else if self.size.is_multiple_of(3) {
      (3, self._split_3())
    } else {
      panic!("Cannot split a pattern of size {}", self.size)
    }
  }

  pub fn flipped(&self) -> Self {
    let size = self.size as usize;
    let data = self
      .data
      .iter()
      .map(|row| row.iter().cloned().rev().collect())
      .collect();

    Pattern {
      size: size as u16,
      data,
    }
  }

  pub fn rotate(&mut self) {
    let mut data = self.data.clone();
    let size = self.size as usize;
    self.data.iter().enumerate().for_each(|(row_idx, row)| {
      row.iter().enumerate().for_each(|(chr_idx, chr)| {
        data[chr_idx][size - 1 - row_idx] = *chr;
      })
    });

    self.data = data;
  }

  pub fn new(input: &str) -> Self {
    let data: Vec<Vec<bool>> = input
      .split('/')
      .map(|line| line.chars().map(|chr| chr == '#').collect())
      .collect();
    let size = data.len() as u16;
    assert!(data.iter().all(|row| row.len() == size as usize));

    Pattern { size, data }
  }
}

impl Default for Pattern {
  fn default() -> Self {
    let size = 3;
    let data = vec![
      vec![false, true, false],
      vec![false, false, true],
      vec![true, true, true],
    ];
    Pattern { size, data }
  }
}

impl fmt::Display for Pattern {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.data.iter() {
      for chr in row.iter() {
        match chr {
          true => write!(f, "#")?,
          false => write!(f, ".")?,
        }
      }
      writeln!(f)?
    }
    Ok(())
  }
}

#[derive(Default)]
struct Image {
  data: Pattern,
  rules: HashMap<Pattern, Pattern>,
}

impl Image {
  fn _add_rules(&mut self, input: &str) {
    input.lines().for_each(|line| {
      let (from, to) = line.split_once(" => ").unwrap();
      let mut from = Pattern::new(from);
      let to = Pattern::new(to);
      for _ in 0..4 {
        self.rules.insert(from.clone(), to.clone());
        self.rules.insert(from.flipped(), to.clone());
        from.rotate();
      }
    });
  }

  pub fn new(rules: &str) -> Self {
    let mut pattern = Image::default();
    pattern._add_rules(rules);
    pattern
  }

  pub fn pixels_on(&self) -> u32 {
    self
      .data
      .data
      .iter()
      .map(|line| line.iter().filter(|cell| **cell).count() as u32)
      .sum::<u32>()
  }

  pub fn mutate(&mut self) {
    if let Some(output) = self.rules.get(&self.data) {
      self.data = output.clone();
    } else {
      let (psize, subps) = self.data.clone().split();
      let mut outputs: Vec<Vec<Vec<bool>>> = subps
        .iter()
        .map(|subp| self.rules.get(subp).unwrap().data.clone())
        .collect();
      let psize = (self.data.size / psize) as usize;

      let mut results = vec![];
      while !outputs.is_empty() {
        while !outputs[0].is_empty() {
          let mut result = vec![];
          (0..psize).for_each(|idx| {
            result.append(&mut outputs[idx][0].drain(..).collect_vec());
            outputs[idx].drain(..1);
          });
          results.push(result);
        }
        outputs.drain(0usize..psize);
      }

      self.data = Pattern {
        size: results[0].len() as u16,
        data: results,
      }
    }
  }
}

pub fn day_21_v1(input: impl Into<String>) -> u32 {
  let mut image = Image::new(&input.into());
  for _ in 0..5 {
    image.mutate();
  }
  image.pixels_on()
}

pub fn day_21_v2(input: impl Into<String>) -> u32 {
  let mut image = Image::new(&input.into());
  for _ in 0..18 {
    image.mutate();
  }
  image.pixels_on()
}

solvable!(day_21, day_21_v1, day_21_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "../.# => ##./#../...\n\
      .#./..#/### => #..#/..../..../#..#";
    let mut image = Image::new(sample_one);
    image.mutate();
    image.mutate();
    assert_eq!(image.pixels_on(), 12);
  }
}
