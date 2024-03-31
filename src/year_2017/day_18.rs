use itertools::Itertools;

#[derive(Clone, Debug)]
enum VoR {
  Value(i64),
  Register(usize),
}

impl VoR {
  fn new(input: &str) -> Self {
    let bytes = input.as_bytes();
    match bytes[0] {
      b'a'..=b'z' => VoR::Register((bytes[0] - b'a') as usize),
      b'0'..=b'9' | b'-' => VoR::Value(input.parse::<i64>().unwrap()),
      _ => panic!("Invalid value: _{}_", input),
    }
  }
}

#[derive(Clone, Debug)]
enum Instruction {
  Sound(VoR),
  Set(usize, VoR),
  Add(usize, VoR),
  Mul(usize, VoR),
  Mod(usize, VoR),
  Recover(usize),
  Jump(VoR, VoR),
}

impl Instruction {
  fn new_snd(vor: &str) -> Self {
    Instruction::Sound(VoR::new(vor))
  }

  fn new_set(reg: &str, vor: &str) -> Self {
    Instruction::Set((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_add(reg: &str, vor: &str) -> Self {
    Instruction::Add((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_mul(reg: &str, vor: &str) -> Self {
    Instruction::Mul((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_mod(reg: &str, vor: &str) -> Self {
    Instruction::Mod((reg.as_bytes()[0] - b'a') as usize, VoR::new(vor))
  }

  fn new_rcv(reg: &str) -> Self {
    Instruction::Recover((reg.as_bytes()[0] - b'a') as usize)
  }

  fn new_jgz(vor: &str, jmp: &str) -> Self {
    Instruction::Jump(VoR::new(vor), VoR::new(jmp))
  }

  fn new(input: &str) -> Self {
    let mut parts = input.split_whitespace();
    match parts.next().unwrap() {
      "snd" => Self::new_snd(parts.next().unwrap()),
      "set" => Self::new_set(parts.next().unwrap(), parts.next().unwrap()),
      "add" => Self::new_add(parts.next().unwrap(), parts.next().unwrap()),
      "mul" => Self::new_mul(parts.next().unwrap(), parts.next().unwrap()),
      "mod" => Self::new_mod(parts.next().unwrap(), parts.next().unwrap()),
      "rcv" => Self::new_rcv(parts.next().unwrap()),
      "jgz" => Self::new_jgz(parts.next().unwrap(), parts.next().unwrap()),
      _ => panic!("Invalid instruction: {}", input),
    }
  }
}

#[derive(Clone)]
struct SoundCard {
  program_id: i64,
  registers: Vec<i64>,
  instructions: Vec<Instruction>,
  pc: i64,
  outputs: Vec<i64>,
  inputs: Vec<i64>,
}

impl SoundCard {
  pub fn new(input: &str, program_id: i64) -> Self {
    let registers = vec![0; 26];
    let instructions = input.lines().map(Instruction::new).collect_vec();

    SoundCard {
      program_id,
      registers,
      instructions,
      pc: 0,
      outputs: vec![],
      inputs: vec![],
    }
  }

  fn value_of(&self, vor: &VoR) -> i64 {
    match vor {
      VoR::Value(value) => *value,
      VoR::Register(value) => self.registers[*value],
    }
  }

  fn execute_once(&mut self) -> bool {
    if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
      return false;
    }
    let instruction = &self.instructions[self.pc as usize];
    match instruction {
      Instruction::Sound(vor) => self.outputs.push(self.value_of(vor)),
      Instruction::Set(reg, vor) => self.registers[*reg] = self.value_of(vor),
      Instruction::Add(reg, vor) => self.registers[*reg] += self.value_of(vor),
      Instruction::Mul(reg, vor) => self.registers[*reg] *= self.value_of(vor),
      Instruction::Mod(reg, vor) => self.registers[*reg] %= self.value_of(vor),
      Instruction::Recover(reg) => match (self.program_id, self.registers[*reg]) {
        (-1, 0) => return false,
        (-1, _) => (),
        (_, _) => {
          if self.inputs.is_empty() {
            return false;
          } else {
            self.registers[*reg] = self.inputs.remove(0);
          }
        }
      },
      Instruction::Jump(vor, jmp) => {
        if self.value_of(vor) > 0 {
          self.pc = self.pc + self.value_of(jmp) - 1;
        }
      }
    }
    self.pc += 1;
    true
  }

  pub fn get_first_sound(&mut self) -> i64 {
    while self.execute_once() {}
    *self.outputs.iter().last().unwrap()
  }
}

pub fn day_18_v1(input: impl Into<String>) -> i64 {
  let mut sound_card = SoundCard::new(&input.into(), 0);
  sound_card.get_first_sound()
}

pub fn day_18_v2(input: impl Into<String>) -> i64 {
  let input = input.into();
  let mut sc0 = SoundCard::new(&input, 0);
  let mut sc1 = SoundCard::new(&input, 1);
  sc1.registers[(b'p' - b'a') as usize] = 1;
  let mut sound_count = 0;
  while sc0.execute_once() || sc1.execute_once() {
    let mut output_1 = sc1.outputs.drain(..).collect_vec();
    sound_count += output_1.len() as i64;
    sc0.inputs.append(&mut output_1);

    sc1.inputs.append(&mut sc0.outputs.drain(..).collect_vec());
  }
  sound_count
  // sc1.sent_count
}
solvable!(day_18, day_18_v1, day_18_v2, i64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: &str = "set a 1\n\
      add a 2\n\
      mul a a\n\
      mod a 5\n\
      snd a\n\
      set a 0\n\
      rcv a\n\
      jgz a -1\n\
      set a 1\n\
      jgz a -2";
    assert_eq!(day_18_v1(sample_one), 4);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "snd 1\n\
      snd 2\n\
      snd p\n\
      rcv a\n\
      rcv b\n\
      rcv c\n\
      rcv d";
    assert_eq!(day_18_v2(sample_two), 3);
  }
}
