type Value = u64;
type Register = u8;

#[derive(Debug, Copy, Clone)]
enum ValueOrRegister {
  Value(Value),
  Register(Register),
}

impl ValueOrRegister {
  fn parse(input: &str) -> Self {
    match input {
      "a" | "b" | "c" | "d" => Self::Register(input.as_bytes()[0] - b'a'),
      _ => Self::Value(input.parse::<Value>().unwrap()),
    }
  }
}

#[derive(Debug)]
enum Instruction {
  // cpy x y copies x (either an integer or the value of a register) into register y.
  Copy(ValueOrRegister, Register),
  // inc x increases the value of register x by one.
  Increment(Register),
  // dec x decreases the value of register x by one.
  Decrement(Register),
  // jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.
  JumpNotZero(ValueOrRegister, i8),
}

pub struct Assembunny {
  pc: usize,
  pub registers: [u64; 4],
  instructions: Vec<Instruction>,
}

impl Assembunny {
  const fn value_of(&self, item: ValueOrRegister) -> u64 {
    match item {
      ValueOrRegister::Value(value) => value,
      ValueOrRegister::Register(register_id) => self.registers[register_id as usize],
    }
  }

  pub fn set_register(&mut self, register: &str, value: u64) {
    let reg_id = Self::register(register);
    self.registers[reg_id as usize] = value;
  }

  pub fn run(&mut self) {
    while (self.pc as usize) < self.instructions.len() {
      match &self.instructions[self.pc as usize] {
        Instruction::Copy(vor, register) => {
          self.registers[*register as usize] = self.value_of(*vor);
          self.pc += 1;
        }
        Instruction::Increment(register) => {
          self.registers[*register as usize] += 1;
          self.pc += 1;
        }
        Instruction::Decrement(register) => {
          self.registers[*register as usize] -= 1;
          self.pc += 1;
        }
        Instruction::JumpNotZero(vor, offset) => {
          let value = self.value_of(*vor);
          if value != 0 {
            self.pc = ((self.pc as i8) + offset) as usize;
          } else {
            self.pc += 1;
          }
        }
      }
    }
  }

  fn register(register: &str) -> u8 {
    match register {
      "A" | "a" => 0,
      "B" | "b" => 1,
      "C" | "c" => 2,
      "D" | "d" => 3,
      _ => panic!("Unknown register: {}", register),
    }
  }

  pub fn from_input(input: &str) -> Self {
    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines() {
      let parts: Vec<&str> = line.split(" ").collect();
      match parts[0] {
        // cpy x y copies x (either an integer or the value of a register) into register y.
        "cpy" => {
          let vor = ValueOrRegister::parse(parts[1]);
          let register = Self::register(parts[2]);
          instructions.push(Instruction::Copy(vor, register));
        }
        // inc x increases the value of register x by one.
        "inc" => {
          let register = Self::register(parts[1]);
          instructions.push(Instruction::Increment(register));
        }
        // dec x decreases the value of register x by one.
        "dec" => {
          let register = Self::register(parts[1]);
          instructions.push(Instruction::Decrement(register));
        }
        // jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.
        "jnz" => {
          let vor = ValueOrRegister::parse(parts[1]);
          let offset = parts[2].parse::<i8>().unwrap();
          instructions.push(Instruction::JumpNotZero(vor, offset));
        }
        _ => panic!("Invalid instruction: {}", line),
      }
    }
    Assembunny {
      pc: 0,
      registers: [0u64; 4],
      instructions: instructions,
    }
  }
}
