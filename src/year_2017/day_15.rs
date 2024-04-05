struct Generator {
  factor: u64,
  value: u64,
  multiples: u64,
}

const FACTOR_A: u64 = 16_807;
const FACTOR_B: u64 = 48_271;

impl Generator {
  fn new(value: u64, factor: u64, multiples: u64) -> Self {
    Generator {
      value,
      factor,
      multiples,
    }
  }
}

impl Iterator for Generator {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let prod = self.value * self.factor;
      let g = (prod & 0x7fff_ffff) + (prod >> 31);
      self.value = if g >> 31 == 0 { g } else { g - 0x7fff_ffff };
      if self.value & (self.multiples - 1) == 0 {
        return Some(self.value);
      }
    }
  }
}

fn parse_input(input: &str) -> (u64, u64) {
  let mut input = input.lines();
  let line_a = input.next().unwrap().split_whitespace().last().unwrap();
  let line_b = input.next().unwrap().split_whitespace().last().unwrap();
  (
    line_a.parse::<u64>().unwrap(),
    line_b.parse::<u64>().unwrap(),
  )
}

pub fn day_15_v1(input: impl Into<String>) -> u64 {
  let inputs = parse_input(&input.into());
  let gen_a = Generator::new(inputs.0, FACTOR_A, 1);
  let gen_b = Generator::new(inputs.1, FACTOR_B, 1);
  gen_a
    .zip(gen_b)
    .take(40_000_000)
    .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
    .count() as u64
}

pub fn day_15_v2(input: impl Into<String>) -> u64 {
  let inputs = parse_input(&input.into());
  let gen_a = Generator::new(inputs.0, FACTOR_A, 4);
  let gen_b = Generator::new(inputs.1, FACTOR_B, 8);
  gen_a
    .zip(gen_b)
    .take(5_000_000)
    .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
    .count() as u64
}

solvable!(day_15, day_15_v1, day_15_v2, u64);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "Generator A starts with 65\n\
    Generator B starts with 8921";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_15_v1(SAMPLE), 588);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_15_v2(SAMPLE), 309);
  }
}
