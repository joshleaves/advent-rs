use itertools::Itertools;

const DIVIDERS: [usize; 15] = [
  65536,
  32768,
  16384,
  8192,
  4096,
  2048,
  1024,
  512,
  256,
  128,
  64,
  32,
  16,
  8,
  4,
];

#[inline]
fn string2bools(input: &str) -> Vec<bool> {
  input
    .trim_end()
    .chars()
    .map(|c| if c == '1' { true } else { false })
    .collect_vec()
}

#[inline]
fn bools2string(input: Vec<bool>) -> String {
  input
    .into_iter()
    .map(|b| if b { '1' } else { '0' })
    .collect()
}

fn fill_until(mut input: Vec<bool>, disk_size: usize) -> Vec<bool> {
  while input.len() < disk_size {
    let mut reverse = input.iter().rev().map(|b| !b).collect_vec();
    input.push(false);
    input.append(&mut reverse);
  }
  input.truncate(disk_size);

  input
}

#[inline]
fn fold16(input: &[bool]) -> bool {
  fold8(&input[0..8]) == fold8(&input[8..16])
}

#[inline]
fn fold8(input: &[bool]) -> bool {
  fold4(&input[0..4]) == fold4(&input[4..8])
}

#[inline]
fn fold4(input: &[bool]) -> bool {
  (input[0] == input[1]) == (input[2] == input[3])
}

#[inline]
fn fold(mut input: Vec<bool>) -> bool {
  while input.len() % 16 == 0 {
    input = input.chunks(16).map(|chunk| fold16(chunk)).collect_vec();
  }
  while input.len() % 8 == 0 {
    input = input.chunks(8).map(|chunk| fold8(chunk)).collect_vec();
  }
  while input.len() % 4 == 0 {
    input = input.chunks(4).map(|chunk| fold4(chunk)).collect_vec();
  }
  if input.len() == 2 {
    return input[0] == input[1];
  }
  input[0]
}

fn checksum(mut input: Vec<bool>) -> Vec<bool> {
  for divider in DIVIDERS {
    while input.len() % divider == 0 {
      input = input
        .chunks(divider)
        .map(|chunk| fold(chunk.to_vec()))
        .collect_vec()
    }
  }

  while input.len() % 2 == 0 {
    input = input.chunks(2).map(|pair| pair[0] == pair[1]).collect_vec()
  }

  input
}

pub fn calculate_checksum(input: &str, disk_size: usize) -> String {
  let mut values = string2bools(&input);
  values = fill_until(values, disk_size);
  values = checksum(values);

  bools2string(values)
}

const DISK_SIZE_V1: usize = 272;

pub fn day_16_v1(input: impl Into<String>) -> String {
  calculate_checksum(&input.into(), DISK_SIZE_V1)
}

const DISK_SIZE_V2: usize = 35_651_584;

pub fn day_16_v2(input: impl Into<String>) -> String {
  calculate_checksum(&input.into(), DISK_SIZE_V2)
}

solvable!(day_16, day_16_v1, day_16_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples() {
    let sample_one = "10000";
    assert_eq!(calculate_checksum(sample_one, 20), "01100");
  }
}
