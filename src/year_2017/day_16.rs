use std::collections::HashMap;

enum DanceMove {
  Spin(usize),
  Exchange(usize, usize),
  Partner(char, char),
}

struct DanceFloor {
  input: Vec<char>,
  instructions: Vec<DanceMove>,
}

impl DanceFloor {
  fn dance(&mut self) {
    self
      .instructions
      .iter()
      .for_each(|instruction| match instruction {
        DanceMove::Spin(rotate) => self.input.rotate_right(*rotate),
        DanceMove::Exchange(lhs, rhs) => self.input.swap(*lhs, *rhs),
        DanceMove::Partner(lhs, rhs) => {
          let lhs_id = self.input.iter().position(|c| c == lhs).unwrap();
          let rhs_id = self.input.iter().position(|c| c == rhs).unwrap();
          self.input.swap(lhs_id, rhs_id)
        }
      })
  }

  pub fn new(input: &str, instructions: &str) -> Self {
    let input: Vec<char> = input.chars().collect();
    let instructions = instructions
      .split(',')
      .map(|instruction| match &instruction[..1] {
        "s" => DanceMove::Spin(instruction[1..].parse::<usize>().unwrap()),
        "x" => {
          let indexes: Vec<usize> = instruction[1..]
            .split('/')
            .map(|number| number.parse::<usize>().unwrap())
            .collect();
          assert_eq!(indexes.len(), 2);
          DanceMove::Exchange(indexes[0], indexes[1])
        }
        "p" => {
          let indexes: Vec<char> = instruction[1..]
            .split('/')
            .map(|chr| chr.chars().next().unwrap())
            .collect();
          assert_eq!(indexes.len(), 2);
          DanceMove::Partner(indexes[0], indexes[1])
        }
        _ => panic!("Invalid instruction: _{}_", instruction),
      })
      .collect();
    DanceFloor {
      input,
      instructions,
    }
  }
}

impl std::fmt::Display for DanceFloor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let string = self.input.iter().collect::<String>();
    write!(f, "{}", string)
  }
}

pub fn day_16_v1(input: impl Into<String>) -> String {
  let input = input.into();
  let input = input.lines().next().unwrap();
  let mut dancefloor = DanceFloor::new("abcdefghijklmnop", input);
  dancefloor.dance();

  dancefloor.to_string()
}

pub fn day_16_v2(input: impl Into<String>) -> String {
  let input = input.into();
  let input = input.lines().next().unwrap();
  let mut dancefloor = DanceFloor::new("abcdefghijklmnop", input);
  let mut positions: HashMap<String, i64> = HashMap::new();
  let mut idx = 0i64;
  while positions.insert(dancefloor.to_string(), idx).is_none() {
    dancefloor.dance();
    idx += 1;
  }
  let pos = 1_000_000_000i64 % positions.len() as i64;
  positions
    .iter()
    .find(|(_, value)| **value == pos)
    .unwrap()
    .0
    .clone()
}
solvable!(day_16, day_16_v1, day_16_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "s1,x3/4,pe/b";

  #[test]
  fn works_with_samples_v1() {
    let mut dancefloor = DanceFloor::new("abcde", SAMPLE);
    dancefloor.dance();
    assert_eq!(dancefloor.to_string(), "baedc");
  }

  #[test]
  fn works_with_samples_v2() {
    let mut dancefloor = DanceFloor::new("abcde", SAMPLE);
    (0..2).for_each(|_| dancefloor.dance());
    assert_eq!(dancefloor.to_string(), "ceadb");
  }
}
