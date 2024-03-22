use itertools::Itertools;
use std::str;

#[derive(Debug)]
enum Instruction {
  SwapPosition(usize, usize),
  SwapLetter(char, char),
  RotateLeft(usize),
  RotateRight(usize),
  RotateBasedOnLetter(char),
  ReversePositions(usize, usize),
  MovePosition(usize, usize),
}

impl Instruction {
  fn new(input: &str) -> Self {
    let parts = input.split_whitespace().collect_vec();
    match (parts[0], parts[1]) {
      ("swap", "position") => {
        let lhs = parts[2].parse::<usize>().unwrap();
        let rhs = parts[5].parse::<usize>().unwrap();
        Instruction::SwapPosition(lhs, rhs)
      }
      ("swap", "letter") => {
        let lhs = parts[2].as_bytes()[0] as char;
        let rhs = parts[5].as_bytes()[0] as char;
        Instruction::SwapLetter(lhs, rhs)
      }
      ("rotate", "left") => {
        let steps = parts[2].parse::<usize>().unwrap();
        Instruction::RotateLeft(steps)
      }
      ("rotate", "right") => {
        let steps = parts[2].parse::<usize>().unwrap();
        Instruction::RotateRight(steps)
      }
      ("rotate", "based") => {
        let chr = parts[6].as_bytes()[0] as char;
        Instruction::RotateBasedOnLetter(chr)
      }
      ("reverse", "positions") => {
        let lhs = parts[2].parse::<usize>().unwrap();
        let rhs = parts[4].parse::<usize>().unwrap();
        Instruction::ReversePositions(lhs, rhs)
      }
      ("move", "position") => {
        let lhs = parts[2].parse::<usize>().unwrap();
        let rhs = parts[5].parse::<usize>().unwrap();
        Instruction::MovePosition(lhs, rhs)
      }
      _ => panic!("Unknown instruction: {}", input),
    }
  }

  fn execute_rev(self, mut password: Vec<char>) -> Vec<char> {
    match self {
      Instruction::RotateLeft(steps) => password.rotate_right(steps),
      Instruction::RotateRight(steps) => password.rotate_left(steps),
      Instruction::MovePosition(lhs, rhs) => {
        let extract = password.remove(rhs);
        password.insert(lhs, extract);
      }
      Instruction::RotateBasedOnLetter(chr) => {
        for rotate in 0..password.len() {
          let mut tester = password.clone();
          tester.rotate_right(rotate);
          let mut index = tester.iter().position(|c| *c == chr).unwrap();
          index += if index >= 4 { 2 } else { 1 };
          index %= password.len();
          tester.rotate_right(index);
          if tester == password {
            password.rotate_left(index);
            break;
          }
        }
      }
      _ => {
        password = self.execute(password);
      }
    };
    password
  }

  fn execute(self, mut password: Vec<char>) -> Vec<char> {
    match self {
      Instruction::SwapPosition(lhs, rhs) => password.swap(lhs, rhs),
      Instruction::SwapLetter(lhs, rhs) => {
        for character in &mut password {
          if *character == lhs {
            *character = rhs;
          } else if *character == rhs {
            *character = lhs;
          }
        }
      }
      Instruction::RotateLeft(steps) => password.rotate_left(steps),
      Instruction::RotateRight(steps) => password.rotate_right(steps),
      Instruction::RotateBasedOnLetter(chr) => {
        let mut index = password.iter().position(|c| *c == chr).unwrap();
        index += if index >= 4 { 2 } else { 1 };
        index %= password.len();
        password.rotate_right(index);
      }
      Instruction::ReversePositions(mut lhs, mut rhs) => {
        while lhs < rhs {
          password.swap(lhs, rhs);
          lhs += 1;
          rhs -= 1;
        }
      }
      Instruction::MovePosition(lhs, rhs) => {
        let extract = password.remove(lhs);
        password.insert(rhs, extract);
      }
    }
    password
  }
}

pub fn day_21_v1(input: impl Into<String>) -> String {
  let mut password = "abcdefgh".chars().collect::<Vec<_>>();
  let instructions = input.into().lines().map(Instruction::new).collect_vec();
  for instruction in instructions {
    password = instruction.execute(password);
  }

  password.iter().collect::<String>()
}

pub fn day_21_v2(input: impl Into<String>) -> String {
  let mut password = "fbgdceah".chars().collect::<Vec<_>>();
  let mut instructions = input.into().lines().map(Instruction::new).collect_vec();
  instructions.reverse();
  for instruction in instructions {
    password = instruction.execute_rev(password);
  }

  password.iter().collect::<String>()
}

solvable!(day_21, day_21_v1, day_21_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "swap position 4 with position 0\n\
    swap letter d with letter b\n\
    reverse positions 0 through 4\n\
    rotate left 1 step\n\
    move position 1 to position 4\n\
    move position 3 to position 0\n\
    rotate based on position of letter b\n\
    rotate based on position of letter d";

  #[test]
  fn can_interpret_code() {
    let mut password = "abcde".chars().collect::<Vec<_>>();
    let instructions = SAMPLE.lines().map(Instruction::new).collect_vec();
    for instruction in instructions {
      password = instruction.execute(password);
    }
    assert_eq!(password.iter().collect::<String>(), "decab");
  }
}
