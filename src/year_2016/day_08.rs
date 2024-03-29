use std::fmt;

struct DisplayScreen {
  width: usize,
  height: usize,
  lights: Vec<bool>,
}

impl DisplayScreen {
  fn new(width: usize, height: usize) -> Self {
    let lights: Vec<bool> = vec![false; width * height];
    DisplayScreen {
      width,
      height,
      lights,
    }
  }

  fn rect(&mut self, width: usize, height: usize) {
    for pos_y in 0..height {
      let idx_from = pos_y * self.width;
      let idx_to = idx_from + width;
      self
        .lights
        .splice(idx_from..idx_to, vec![true; idx_to - idx_from]);
    }
  }

  fn rotate_row(&mut self, row: usize, shift: usize) {
    let idx_from = row * self.width;
    let idx_to = (row + 1) * self.width;
    let mut extract = self.lights[idx_from..idx_to].to_vec();
    extract.rotate_right(shift);

    self.lights.splice(idx_from..idx_to, extract);
  }

  fn rotate_column(&mut self, column: usize, shift: usize) {
    let mut extract: Vec<bool> = vec![false; self.height];
    for (row, extracted) in extract.iter_mut().enumerate() {
      *extracted = self.lights[(row * self.width) + column];
    }
    extract.rotate_right(shift);
    for (row, extracted) in extract.iter().enumerate() {
      self.lights[(row * self.width) + column] = *extracted;
    }
  }

  fn interpret(&mut self, instruction: &str) {
    let parts: Vec<_> = instruction.split_whitespace().collect();
    match parts[0] {
      "rect" => {
        let pos = parts[1].split_once('x').unwrap();
        let width = pos.0.parse::<usize>().unwrap();
        let height = pos.1.parse::<usize>().unwrap();
        self.rect(width, height);
      }
      "rotate" => {
        let pos: Vec<_> = parts[2].split('=').collect();
        let shift = parts[4].parse::<usize>().unwrap();
        match (parts[1], pos[0]) {
          ("row", "y") => {
            let row = pos[1].parse::<usize>().unwrap();
            self.rotate_row(row, shift)
          }
          ("column", "x") => {
            let column = pos[1].parse::<usize>().unwrap();
            self.rotate_column(column, shift)
          }
          _ => panic!("Invalid instruction: {}", instruction),
        }
      }
      _ => panic!("Invalid instruction: {}", instruction),
    }
  }

  fn lights_on(&self) -> u16 {
    self.lights.iter().filter(|light| **light).count() as u16
  }
}

impl fmt::Display for DisplayScreen {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let displaystr: Vec<String> = (0..self.height)
      .map(|row| {
        self.lights[row * self.width..(row + 1) * self.width]
          .iter()
          .map(|light| if *light { '#' } else { '.' })
          .collect::<String>()
      })
      .collect();

    write!(f, "{}", displaystr.join("\n"))
  }
}

pub fn day_08_v1(input: impl Into<String>) -> u16 {
  let mut screen = DisplayScreen::new(50, 6);
  for line in input.into().lines() {
    screen.interpret(line);
  }

  screen.lights_on()
}

pub fn day_08_v2(input: impl Into<String>) -> String {
  let mut screen = DisplayScreen::new(50, 6);
  for line in input.into().lines() {
    screen.interpret(line);
  }

  screen.to_string()
}

/// Stub function only calling the _v1 variant (v2 cannot be tested)
pub fn day_08(_part: u8, input: impl Into<String>) -> u16 {
  day_08_v1(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let mut screen = DisplayScreen::new(7, 3);
    screen.interpret("rect 3x2");
    assert_eq!(screen.to_string(), "###....\n###....\n.......");

    screen.interpret("rotate column x=1 by 1");
    assert_eq!(screen.to_string(), "#.#....\n###....\n.#.....");

    screen.interpret("rotate row y=0 by 4");
    assert_eq!(screen.to_string(), "....#.#\n###....\n.#.....");

    screen.interpret("rotate column x=1 by 1");
    assert_eq!(screen.to_string(), ".#..#.#\n#.#....\n.#.....");
  }
}
