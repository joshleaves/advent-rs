//! Advent of Code 2015: Day 18: Like a GIF For Your Yard

type Light = bool;
const LIGHT_ON: Light = true;
const LIGHT_OFF: Light = false;

struct GameOfLifeGrid {
  data: Vec<Vec<Light>>,
  size: usize,
  alive_corner: bool,
}

impl GameOfLifeGrid {
  fn alive_count(&self) -> usize {
    let mut alive_count: usize = 0;
    for row in 0..self.size {
      alive_count += self.data[row].iter().filter(|c| **c == LIGHT_ON).count();
    }

    alive_count
  }

  fn cell_is_alive(&self, row: usize, line: usize) -> bool {
    if self.alive_corner
      && ((row, line) == (0, 0)
        || (row, line) == (self.size - 1, 0)
        || (row, line) == (0, self.size - 1)
        || (row, line) == (self.size - 1, self.size - 1))
    {
      return true;
    }
    self.data[row][line]
  }

  fn cell_will_be_alive(&self, row: usize, line: usize) -> bool {
    if self.alive_corner
      && ((row, line) == (0, 0)
        || (row, line) == (self.size - 1, 0)
        || (row, line) == (0, self.size - 1)
        || (row, line) == (self.size - 1, self.size - 1))
    {
      return true;
    }
    let mut around_positions: Vec<(usize, usize)> = vec![];
    if row > 0 {
      if line > 0 {
        around_positions.push((row - 1, line - 1));
      }
      around_positions.push((row - 1, line));
      if line < self.size - 1 {
        around_positions.push((row - 1, line + 1));
      }
    }
    if line > 0 {
      around_positions.push((row, line - 1));
    }
    if line < self.size - 1 {
      around_positions.push((row, line + 1));
    }
    if row < self.size - 1 {
      if line > 0 {
        around_positions.push((row + 1, line - 1));
      }
      around_positions.push((row + 1, line));
      if line < self.size - 1 {
        around_positions.push((row + 1, line + 1));
      }
    }

    let cell_status = self.data[row][line];
    let neighbors = around_positions
      .iter()
      .filter(|(row, line)| self.cell_is_alive(*row, *line))
      .count();
    matches!((cell_status, neighbors), (LIGHT_ON, 2) | (_, 3))
  }

  fn from_grid(old_grid: GameOfLifeGrid) -> Self {
    let mut data: Vec<Vec<Light>> = vec![];
    for (x, row) in old_grid.data.iter().enumerate() {
      let mut data_line: Vec<Light> = vec![];
      for (y, _cell) in row.iter().enumerate() {
        match old_grid.cell_will_be_alive(x, y) {
          true => data_line.push(LIGHT_ON),
          false => data_line.push(LIGHT_OFF),
        }
      }
      data.push(data_line);
    }
    let size: usize = old_grid.size;
    let alive_corner: bool = old_grid.alive_corner;
    GameOfLifeGrid {
      data,
      size,
      alive_corner,
    }
  }

  fn from_str(input: &str, alive_corner: bool) -> Self {
    let mut data: Vec<Vec<Light>> = vec![];
    let mut size: usize = 0;
    for line in input.lines() {
      let mut data_line: Vec<Light> = vec![];
      for chr in line.chars() {
        match chr {
          '#' => data_line.push(LIGHT_ON),
          _ => data_line.push(LIGHT_OFF),
        }
      }
      size = data_line.len();
      data.push(data_line);
    }
    if alive_corner {
      data[0][0] = LIGHT_ON;
      data[0][size - 1] = LIGHT_ON;
      data[size - 1][0] = LIGHT_ON;
      data[size - 1][size - 1] = LIGHT_ON;
    }

    GameOfLifeGrid {
      data,
      size,
      alive_corner,
    }
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

pub fn day_18_v1(input: impl Into<String>) -> usize {
  let mut grid = GameOfLifeGrid::from_str(&input.into(), false);
  for _i in 0..100 {
    grid = GameOfLifeGrid::from_grid(grid);
  }

  grid.alive_count()
}

pub fn day_18_v2(input: impl Into<String>) -> usize {
  let mut grid = GameOfLifeGrid::from_str(&input.into(), true);

  for _i in 0..100 {
    grid = GameOfLifeGrid::from_grid(grid);
  }

  grid.alive_count()
}

solvable!(day_18, day_18_v1, day_18_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
    let mut grid = GameOfLifeGrid::from_str(sample, false);
    grid = GameOfLifeGrid::from_grid(grid);
    assert_eq!(grid.alive_count(), 11);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample = "##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#";
    let mut grid = GameOfLifeGrid::from_str(sample, true);

    grid = GameOfLifeGrid::from_grid(grid);
    assert_eq!(
      grid.to_string(),
      "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####\n"
    );

    grid = GameOfLifeGrid::from_grid(grid);
    assert_eq!(
      grid.to_string(),
      "#..#.#\n#....#\n.#.##.\n...##.\n.#..##\n##.###\n"
    );

    grid = GameOfLifeGrid::from_grid(grid);
    assert_eq!(
      grid.to_string(),
      "#...##\n####.#\n..##.#\n......\n##....\n####.#\n"
    );

    grid = GameOfLifeGrid::from_grid(grid);
    assert_eq!(
      grid.to_string(),
      "#.####\n#....#\n...#..\n.##...\n#.....\n#.#..#\n"
    );

    grid = GameOfLifeGrid::from_grid(grid);
    assert_eq!(
      grid.to_string(),
      "##.###\n.##..#\n.##...\n.##...\n#.#...\n##...#\n"
    );
  }
}
