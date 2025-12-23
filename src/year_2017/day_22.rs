use std::collections::HashMap;

use itertools::Itertools;

type Position = (i32, i32);

enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[derive(Default)]
enum State {
  #[default]
  Clean,
  Weakened,
  Infected,
  Flagged,
}

struct Grid {
  infected: HashMap<Position, State>,
  position: Position,
  direction: Direction,
  infections: u32,
}

impl Grid {
  #[inline]
  fn turn_left(&mut self) {
    self.direction = match self.direction {
      Direction::Up => Direction::Left,
      Direction::Left => Direction::Down,
      Direction::Down => Direction::Right,
      Direction::Right => Direction::Up,
    };
  }

  #[inline]
  fn turn_right(&mut self) {
    self.direction = match self.direction {
      Direction::Up => Direction::Right,
      Direction::Left => Direction::Up,
      Direction::Down => Direction::Left,
      Direction::Right => Direction::Down,
    };
  }

  #[inline]
  fn reverse(&mut self) {
    self.direction = match self.direction {
      Direction::Up => Direction::Down,
      Direction::Left => Direction::Right,
      Direction::Down => Direction::Up,
      Direction::Right => Direction::Left,
    };
  }

  #[inline]
  fn move_forward(&mut self) {
    match self.direction {
      Direction::Up => self.position.1 -= 1,
      Direction::Left => self.position.0 -= 1,
      Direction::Down => self.position.1 += 1,
      Direction::Right => self.position.0 += 1,
    };
  }

  pub fn new(input: &str) -> Self {
    let mut size = 0;
    let infected = input
      .lines()
      .enumerate()
      .flat_map(|(y, row)| {
        row
          .chars()
          .enumerate()
          .filter_map(|(x, chr)| {
            size = std::cmp::max(size, x.div_ceil(2));
            if chr == '#' {
              Some((x as i32, y as i32))
            } else {
              None
            }
          })
          .collect_vec()
      })
      .collect_vec();
    let infected = HashMap::from_iter(
      infected
        .iter()
        .map(|p| ((p.0 - size as i32, p.1 - size as i32), State::Infected)),
    );

    Grid {
      infected,
      position: (0, 0),
      direction: Direction::Up,
      infections: 0,
    }
  }
}

impl Grid {
  #[inline]
  fn next_v1(&mut self) {
    let entry = self.infected.entry(self.position).or_insert(State::Clean);
    match *entry {
      State::Clean => {
        *entry = State::Infected;
        self.infections += 1;
        self.turn_left();
      }
      State::Infected => {
        *entry = State::Clean;
        self.turn_right();
      }
      State::Weakened | State::Flagged => (),
    }
    self.move_forward();
  }

  #[inline]
  fn next_v2(&mut self) {
    let entry = self.infected.entry(self.position).or_insert(State::Clean);
    match *entry {
      State::Clean => {
        *entry = State::Weakened;
        self.turn_left();
      }
      State::Weakened => {
        *entry = State::Infected;
        self.infections += 1;
      }
      State::Infected => {
        *entry = State::Flagged;
        self.turn_right();
      }
      State::Flagged => {
        *entry = State::Clean;
        self.reverse();
      }
    }
    self.move_forward();
  }
}

pub fn day_22_v1(input: impl Into<String>) -> u32 {
  let mut grid = Grid::new(&input.into());
  for _ in 0..10_000 {
    grid.next_v1();
  }
  grid.infections
}

pub fn day_22_v2(input: impl Into<String>) -> u32 {
  let mut grid = Grid::new(&input.into());
  for _ in 0..10_000_000 {
    grid.next_v2();
  }
  grid.infections
}

solvable!(day_22, day_22_v1, day_22_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "..#\n\
    #..\n\
    ...";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_22_v1(SAMPLE), 5_587);
  }

  #[test]
  #[ignore = "Too slow for CI"]
  fn works_with_samples_v2() {
    assert_eq!(day_22_v2(SAMPLE), 2_511_944);
  }
}
