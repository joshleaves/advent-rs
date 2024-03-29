//! Advent of Code 2015: Day 6: Probably a Fire Hazard

use itertools::Itertools;

type Position = (u16, u16);

#[inline]
fn parse_coordinates(coordinates: &str) -> Position {
  coordinates
    .split(',')
    .map(|coord| coord.parse::<u16>().unwrap())
    .collect_tuple()
    .unwrap()
}

enum Instruction {
  TurnOn(Position, Position),
  TurnOff(Position, Position),
  Toggle(Position, Position),
}

impl Instruction {
  fn new(mut input: &str) -> Self {
    input = input.trim();
    if let Some(stripped_input) = input.strip_prefix("turn") {
      input = stripped_input;
    }
    let parts: Vec<_> = input.split_whitespace().collect();
    let from = parse_coordinates(parts[1]);
    let to = parse_coordinates(parts[3]);
    match parts[0] {
      "on" => Instruction::TurnOn(from, to),
      "off" => Instruction::TurnOff(from, to),
      "toggle" => Instruction::Toggle(from, to),
      _ => panic!("Invalid instruction: {}", input),
    }
  }

  fn execute_v1(&self, light_grid: &mut Vec<bool>) {
    match self {
      Instruction::TurnOn(from, to) => {
        modify_grid(light_grid, *from, *to, |_lights: &[bool], length: usize| {
          vec![true; length]
        })
      }
      Instruction::TurnOff(from, to) => {
        modify_grid(light_grid, *from, *to, |_lights: &[bool], length: usize| {
          vec![false; length]
        })
      }
      Instruction::Toggle(from, to) => {
        modify_grid(light_grid, *from, *to, |lights: &[bool], _length: usize| {
          lights.iter().map(|c| !*c).collect()
        })
      }
    }
  }

  fn execute_v2(&self, light_grid: &mut Vec<u8>) {
    match self {
      Instruction::TurnOn(from, to) => {
        modify_grid(light_grid, *from, *to, |lights: &[u8], _length: usize| {
          lights.iter().map(|c| *c + 1).collect()
        })
      }
      Instruction::TurnOff(from, to) => {
        modify_grid(light_grid, *from, *to, |lights: &[u8], _length: usize| {
          lights
            .iter()
            .map(|c| std::cmp::max(0, *c as i8 - 1) as u8)
            .collect()
        })
      }
      Instruction::Toggle(from, to) => {
        modify_grid(light_grid, *from, *to, |lights: &[u8], _length: usize| {
          lights.iter().map(|c| *c + 2).collect()
        })
      }
    }
  }
}

fn modify_grid<F, T>(light_grid: &mut Vec<T>, from: Position, to: Position, modifier: F)
where
  F: Fn(&[T], usize) -> Vec<T>,
  T: Clone,
{
  let (from_y, from_x) = from;
  let (to_y, to_x) = to;
  for y in from_y..=to_y {
    let idx_from = y as usize * 1000 + from_x as usize;
    let idx_to = y as usize * 1000 + to_x as usize;
    let oldval = &light_grid[idx_from..=idx_to];
    let newval = modifier(oldval, oldval.len());

    light_grid.splice(idx_from..=idx_to, newval.iter().cloned());
  }
}

pub fn day_06_v1(input: impl Into<String>) -> u32 {
  let mut light_grid: Vec<bool> = vec![false; 1_000_000];
  for line in input.into().lines() {
    let instruction = Instruction::new(line);
    instruction.execute_v1(&mut light_grid);
  }

  light_grid.iter().filter(|&light| *light).count() as u32
}

pub fn day_06_v2(input: impl Into<String>) -> u32 {
  let mut light_grid: Vec<u8> = vec![0_u8; 1_000_000];
  for line in input.into().lines() {
    let instruction = Instruction::new(line);
    instruction.execute_v2(&mut light_grid)
  }

  light_grid.iter().map(|&i| i as u32).sum()
}

solvable!(day_06, day_06_v1, day_06_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "turn on 0,0 through 999,999\n\
      toggle 0,0 through 999,0\n\
      turn off 499,499 through 500,500";
    assert_eq!(day_06_v1(sample_one), 998_996);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "turn on 0,0 through 0,0\n\
      toggle 0,0 through 999,999";
    assert_eq!(day_06_v2(sample_two), 2_000_001);
  }
}
