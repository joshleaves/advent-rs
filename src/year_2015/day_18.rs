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
    self
      .data
      .iter()
      .map(|row| row.iter().filter(|c| **c == LIGHT_ON).count())
      .sum()
  }

  #[inline]
  fn cell_is_alive(&self, row: usize, line: usize) -> bool {
    self.data[row][line]
  }

  fn count_cells(&self, row: usize, line: usize) -> u8 {
    let row_from = if row == 0 { 0 } else { row - 1 };
    let row_to = if row + 1 == self.size { row } else { row + 1 };
    let line_from = if line == 0 { 0 } else { line - 1 };
    let line_to = if line + 1 == self.size {
      line
    } else {
      line + 1
    };
    let mut alive = 0_u8;
    for line_id in line_from..=line_to {
      for row_id in row_from..=row_to {
        if line == line_id && row == row_id {
          continue;
        }
        if self.cell_is_alive(row_id, line_id) {
          alive += 1;
        }
      }
    }
    alive
  }

  fn cell_will_be_alive(&self, row: usize, line: usize) -> bool {
    let cell_status = self.data[row][line];
    let neighbors = self.count_cells(row, line);
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

  fn mutate(self) -> Self {
    GameOfLifeGrid::from_grid(self)
  }

  fn new(input: &str, alive_corner: bool) -> Self {
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
  let mut grid = GameOfLifeGrid::new(&input.into(), false);
  for _i in 0..100 {
    grid = grid.mutate();
  }

  grid.alive_count()
}

pub fn day_18_v2(input: impl Into<String>) -> usize {
  let mut grid = GameOfLifeGrid::new(&input.into(), true);

  for _i in 0..100 {
    grid = grid.mutate();
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
    let mut grid = GameOfLifeGrid::new(sample, false);
    grid = grid.mutate();
    assert_eq!(grid.alive_count(), 11);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample = "##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#";
    let mut grid = GameOfLifeGrid::new(sample, true);

    grid = grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####\n"
    );

    grid = grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#..#.#\n#....#\n.#.##.\n...##.\n.#..##\n##.###\n"
    );

    grid = grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#...##\n####.#\n..##.#\n......\n##....\n####.#\n"
    );

    grid = grid.mutate();
    assert_eq!(
      grid.to_string(),
      "#.####\n#....#\n...#..\n.##...\n#.....\n#.#..#\n"
    );

    grid = grid.mutate();
    assert_eq!(
      grid.to_string(),
      "##.###\n.##..#\n.##...\n.##...\n#.#...\n##...#\n"
    );
  }
}
