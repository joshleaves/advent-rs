pub fn day_05_v1(input: impl Into<String>) -> u32 {
  let mut instructions: Vec<i16> = input
    .into()
    .lines()
    .map(|line| line.parse::<i16>().unwrap())
    .collect();
  let mut pc: i16 = 0;
  let mut moves: u32 = 0;

  while let Some(instruction) = instructions.get_mut(pc as usize) {
    pc += *instruction;
    *instruction += 1;
    moves += 1;
  }
  moves
}

pub fn day_05_v2(input: impl Into<String>) -> u32 {
  let mut instructions: Vec<i16> = input
    .into()
    .lines()
    .map(|line| line.parse::<i16>().unwrap())
    .collect();
  let mut pc: i16 = 0;
  let mut moves: u32 = 0;

  while let Some(instruction) = instructions.get_mut(pc as usize) {
    pc += *instruction;
    *instruction += if *instruction < 3 { 1 } else { -1 };
    moves += 1;
  }
  moves
}

solvable!(day_05, day_05_v1, day_05_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "0\n\
    3\n\
    0\n\
    1\n\
    -3";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_05_v1(SAMPLE), 5);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_05_v2(SAMPLE), 10);
  }
}
