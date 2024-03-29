//! Advent of Code 2015: Day 18: Like a GIF For Your Yard

type Light = bool;
const LIGHT_ON: Light = true;
const LIGHT_OFF: Light = false;

/// Structure to old Game Of Life state.
///
/// DO. NOT. EVER. TURN. IT. TO. A. SINGLE. DIMENSION. ARRAY.
///
/// The performance loss is kinda as follows:
///
/// ```text
/// year_2015::day_18/year_2015::day_18_v1
///         time:   [10.337 ms 10.350 ms 10.364 ms]
///         change: [+32.681% +32.909% +33.125%] (p = 0.00 < 0.05)
///         Performance has regressed.
/// year_2015::day_18/year_2015::day_18_v2
///         time:   [10.357 ms 10.385 ms 10.414 ms]
///         change: [+33.193% +33.534% +33.954%] (p = 0.00 < 0.05)
///         Performance has regressed.
/// ```
///
struct GameOfLifeGrid {
  data: Vec<Vec<Light>>,
  size: usize,
  alive_corners: bool,
}

impl GameOfLifeGrid {
  fn alive_count(&self) -> u16 {
    self
      .data
      .iter()
      .map(|row| row.iter().filter(|c| **c == LIGHT_ON).count() as u16)
      .sum::<u16>()
  }

  #[inline]
  fn get(&self, row: usize, line: usize) -> Light {
    self.data[row][line]
  }
  #[inline]
  fn set(&mut self, row: usize, line: usize, value: Light) {
    self.data[row][line] = value
  }

  /// Optimization trick here: we loop on range of (-1..+1) on both axis, except
  /// we have to skip our current position, which is (at worst) nine `if` checks
  /// where only one will be really useful.
  ///
  /// To get the optimization, we just grab the value as we iterate, and account
  /// for it when matching status/neibours next.
  fn cell_will_be_alive(&self, row: usize, line: usize) -> bool {
    let row_from = std::cmp::max(0, row as i32 - 1) as usize;
    let line_from = std::cmp::max(0, line as i32 - 1) as usize;
    let row_to = std::cmp::min(self.size - 1, row + 1) as usize;
    let line_to = std::cmp::min(self.size - 1, line + 1) as usize;
    let neighbors = (line_from..=line_to)
      .map(|line_id| {
        (row_from..=row_to)
          .filter(|&row_id| self.get(row_id, line_id))
          .count() as u8
      })
      .sum::<u8>();
    let cell_status = self.get(row, line);
    match (cell_status, neighbors) {
      (LIGHT_ON, 3) => LIGHT_ON,
      (LIGHT_ON, 4) => LIGHT_ON,
      (LIGHT_OFF, 3) => LIGHT_ON,
      _ => LIGHT_OFF,
    }
  }

  fn revive_corners(&mut self) {
    if !self.alive_corners {
      return;
    }
    let max = self.size - 1;
    for (row, line) in [
      (0, 0),
      (0, max),
      (max, 0),
      (max, max),
    ] {
      self.set(row, line, LIGHT_ON)
    }
  }

  fn mutate(&mut self) {
    let data: Vec<Vec<bool>> = self
      .data
      .iter()
      .enumerate()
      .map(|(x, row)| {
        row
          .iter()
          .enumerate()
          .map(|(y, _cell)| self.cell_will_be_alive(x, y))
          .collect()
      })
      .collect();
    self.data = data;
    self.revive_corners();
  }

  fn new(input: &str, alive_corners: bool) -> Self {
    let mut data: Vec<Vec<Light>> = vec![];
    for line in input.lines() {
      let data_line: Vec<_> = line
        .chars()
        .map(|chr| match chr {
          '#' => LIGHT_ON,
          _ => LIGHT_OFF,
        })
        .collect();
      data.push(data_line);
    }
    let size = data[0].len();
    let mut grid = GameOfLifeGrid {
      data,
      size,
      alive_corners,
    };
    grid.revive_corners();
    grid
  }
}

impl ToString for GameOfLifeGrid {
  fn to_string(&self) -> String {
    let mut data: String = String::new();
    for row in 0..self.size {
      let row_str = self.data[row]
        .iter()
        .map(|c| if *c == LIGHT_ON { "#" } else { "." })
        .collect::<Vec<&str>>()
        .join("");
      data.push_str(&row_str);
      data.push('\n');
    }

    data
  }
}

pub fn day_18_v1(input: impl Into<String>) -> u16 {
  let mut grid = GameOfLifeGrid::new(&input.into(), false);
  for _i in 0..100 {
    grid.mutate();
  }

  grid.alive_count()
}

pub fn day_18_v2(input: impl Into<String>) -> u16 {
  let mut grid = GameOfLifeGrid::new(&input.into(), true);
  for _i in 0..100 {
    grid.mutate();
  }

  grid.alive_count()
}

solvable!(day_18, day_18_v1, day_18_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
    let mut grid = GameOfLifeGrid::new(sample, false);
    grid.mutate();
    assert_eq!(grid.alive_count(), 11);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample = "##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#";
    let mut grid = GameOfLifeGrid::new(sample, true);

    grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####\n"
    );

    grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#..#.#\n#....#\n.#.##.\n...##.\n.#..##\n##.###\n"
    );

    grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#...##\n####.#\n..##.#\n......\n##....\n####.#\n"
    );

    grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#.####\n#....#\n...#..\n.##...\n#.....\n#.#..#\n"
    );

    grid.mutate();
    assert_eq!(
      grid.to_string(),
      "##.###\n.##..#\n.##...\n.##...\n#.#...\n##...#\n"
    );
  }
}
