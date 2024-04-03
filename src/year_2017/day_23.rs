use itertools::Itertools;

#[derive(Clone, Debug)]
enum VoR {
  Value(i32),
  Register(usize),
}

impl VoR {
  fn new(input: &str) -> Self {
    let bytes = input.as_bytes();
    match bytes[0] {
      b'a'..=b'h' => VoR::Register((bytes[0] - b'a') as usize),
      b'0'..=b'9' | b'-' => VoR::Value(input.parse::<i32>().unwrap()),
      _ => panic!("Invalid value: _{}_", input),
    }
  }
}

#[derive(Clone, Debug)]
enum Instruction {
  Set(usize, VoR),
  Sub(usize, VoR),
  Mul(usize, VoR),
  Jump(VoR, VoR),
}

impl Instruction {
  fn new_set(reg: &str, vor: &str) -> Self {
    Instruction::Set((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_sub(reg: &str, vor: &str) -> Self {
    Instruction::Sub((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_mul(reg: &str, vor: &str) -> Self {
    Instruction::Mul((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_jnz(vor: &str, jmp: &str) -> Self {
    Instruction::Jump(VoR::new(vor), VoR::new(jmp))
  }

  fn new(input: &str) -> Self {
    let mut parts = input.split_whitespace();
    match parts.next().unwrap() {
      "set" => Self::new_set(parts.next().unwrap(), parts.next().unwrap()),
      "sub" => Self::new_sub(parts.next().unwrap(), parts.next().unwrap()),
      "mul" => Self::new_mul(parts.next().unwrap(), parts.next().unwrap()),
      "jnz" => Self::new_jnz(parts.next().unwrap(), parts.next().unwrap()),
      _ => panic!("Invalid instruction: {}", input),
    }
  }
}

struct Coprocessor {
  registers: [i32; 8],
  instructions: Vec<Instruction>,
  pc: i16,
  count_mul: i32,
}

impl Coprocessor {
  fn new(input: &str) -> Self {
    let registers = [0; 8];
    let instructions = input.lines().map(Instruction::new).collect_vec();

    Coprocessor {
      registers,
      instructions,
      pc: 0,
      count_mul: 0,
    }
  }

  fn value_of(&self, vor: &VoR) -> i32 {
    match vor {
      VoR::Value(value) => *value,
      VoR::Register(value) => self.registers[*value],
    }
  }

  fn execute_once(&mut self) -> bool {
    if self.pc < 0 || self.pc >= self.instructions.len() as i16 {
      return false;
    }
    match &self.instructions[self.pc as usize] {
      Instruction::Set(reg, vor) => self.registers[*reg] = self.value_of(vor),
      Instruction::Sub(reg, vor) => self.registers[*reg] -= self.value_of(vor),
      Instruction::Mul(reg, vor) => {
        self.registers[*reg] *= self.value_of(vor);
        self.count_mul += 1;
      }
      Instruction::Jump(vor, jmp) => {
        if self.value_of(vor) != 0 {
          self.pc = self.pc + self.value_of(jmp) as i16 - 1;
        }
      }
    }
    self.pc += 1;
    true
  }
}

pub fn day_23_v1(input: impl Into<String>) -> i32 {
  let mut proco = Coprocessor::new(&input.into());
  while proco.execute_once() {}
  proco.count_mul
}

pub fn day_23_v2(input: impl Into<String>) -> i32 {
  let mut proco = Coprocessor::new(&input.into());
  proco.registers[0] = 1;
  for _ in 0..10 {
    proco.execute_once();
  }
  let start = proco.registers[1];
  let end = proco.registers[2];

  (start..(end + 1))
    .step_by(17)
    .filter(|&n| {
      for i in 2..n {
        if n % i == 0 {
          return true;
        }
      }
      false
    })
    .count() as i32
}

solvable!(day_23, day_23_v1, day_23_v2, i32);
