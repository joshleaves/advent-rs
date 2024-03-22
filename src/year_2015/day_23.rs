//! Advent of Code 2015: Day 23: Opening the Turing Lock

#[derive(Debug)]
enum Instruction {
  Half(usize),
  Triple(usize),
  Increment(usize),
  Jump(i8),
  JumpIfEven(usize, i8),
  JumpIfOne(usize, i8),
}

struct Program {
  pc: i8,
  registers: [i32; 2],
  instructions: Vec<Instruction>,
}

impl Program {
  fn run(&mut self) {
    // while self.pc < (self.instructions.len() as i8) {
    while (self.pc as usize) < self.instructions.len() {
      match self.instructions[self.pc as usize] {
        Instruction::Half(register) => {
          self.registers[register] /= 2;
          self.pc += 1;
        }
        Instruction::Triple(register) => {
          self.registers[register] *= 3;
          self.pc += 1;
        }
        Instruction::Increment(register) => {
          self.registers[register] += 1;
          self.pc += 1;
        }
        Instruction::Jump(offset) => {
          self.pc += offset;
        }
        Instruction::JumpIfEven(register, offset) => {
          if self.registers[register] % 2 == 0 {
            self.pc += offset;
          } else {
            self.pc += 1;
          }
        }
        Instruction::JumpIfOne(register, offset) => {
          if self.registers[register] == 1 {
            self.pc += offset;
          } else {
            self.pc += 1;
          }
        }
      }
    }
  }

  fn from_input(input: &str) -> Self {
    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines() {
      let parts: Vec<&str> = line.split(' ').collect();
      match parts[0] {
        "hlf" => {
          let register = if parts[1] == "a" { 0 } else { 1 };
          instructions.push(Instruction::Half(register))
        }
        "tpl" => {
          let register = if parts[1] == "a" { 0 } else { 1 };
          instructions.push(Instruction::Triple(register))
        }
        "inc" => {
          let register = if parts[1] == "a" { 0 } else { 1 };
          instructions.push(Instruction::Increment(register))
        }
        "jmp" => {
          let Ok(offset) = parts[1].parse::<i8>() else {
            panic!("Invalid instruction: {}", line);
          };
          instructions.push(Instruction::Jump(offset))
        }
        "jie" => {
          let register = if parts[1] == "a," { 0 } else { 1 };
          let Ok(offset) = parts[2].parse::<i8>() else {
            panic!("Invalid instruction: {}", line);
          };
          instructions.push(Instruction::JumpIfEven(register, offset))
        }
        "jio" => {
          let register = if parts[1] == "a," { 0 } else { 1 };
          let Ok(offset) = parts[2].parse::<i8>() else {
            panic!("Invalid instruction: {}", line);
          };
          instructions.push(Instruction::JumpIfOne(register, offset))
        }
        _ => {
          panic!("Invalid instruction: {}", line)
        }
      }
    }

    Program {
      pc: 0,
      registers: [0i32; 2],
      instructions,
    }
  }
}

pub fn day_23_v1(input: impl Into<String>) -> i32 {
  let mut program = Program::from_input(&input.into());
  program.run();

  program.registers[1]
}

pub fn day_23_v2(input: impl Into<String>) -> i32 {
  let mut program = Program::from_input(&input.into());
  program.registers[0] = 1;
  program.run();

  program.registers[1]
}

solvable!(day_23, day_23_v1, day_23_v2, i32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn iterates_over_program() {
    let sample_one = "inc a\njio a, +2\ntpl a\ninc a";
    let mut program = Program::from_input(sample_one);
    program.run();
    assert_eq!(program.registers[0], 2);
  }
}
