use itertools::Itertools;

#[inline]
fn parse_jumps_v1(input: &str) -> Vec<u32> {
  input
    .trim_end()
    .split(',')
    .map(|number| number.trim().parse::<u32>().unwrap())
    .collect()
}

#[inline]
fn parse_jumps_v2(input: &str) -> Vec<u32> {
  input
    .trim_end()
    .bytes()
    .map(|number| number as u32)
    .collect()
}

fn knot_hash(size: u32, jumps: &[u32], rounds: u32) -> Vec<u32> {
  let mut numbers: Vec<_> = (0..size).collect_vec();
  let num_lens = numbers.len();
  let mut jumps_done = 0;
  let mut skip_size = 0;

  for _ in 0..rounds {
    for jump in jumps.iter() {
      let extract = numbers[0..*jump as usize]
        .iter()
        .cloned()
        .rev()
        .collect_vec();
      numbers.splice(0..*jump as usize, extract);
      numbers.rotate_left(((jump + skip_size) % num_lens as u32) as usize);
      jumps_done += (jump + skip_size) % num_lens as u32;
      skip_size += 1;
    }
  }
  numbers.rotate_right(jumps_done as usize % num_lens);

  numbers
}

pub fn day_10_v1(input: impl Into<String>) -> String {
  let numbers = knot_hash(256, &parse_jumps_v1(&input.into()), 1);
  (numbers[0] * numbers[1]).to_string()
}

pub fn day_10_v2(input: impl Into<String>) -> String {
  let mut jumps = parse_jumps_v2(&input.into());
  jumps.append(&mut vec![17, 31, 73, 47, 23]);

  knot_hash(256, &jumps, 64)
    .chunks(16)
    .map(|chunk| chunk.iter().fold(0, |acc, &val| acc ^ val) as u8)
    .map(|xor| format!("{:02x}", xor))
    .collect::<Vec<_>>()
    .join("")
}

solvable!(day_10, day_10_v1, day_10_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let jumps = parse_jumps_v1("3, 4, 1, 5");
    let numbers = knot_hash(5, &jumps, 1);
    assert_eq!(numbers[0] * numbers[1], 12);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, &str); 4] = [
      ("", "a2582a3a0e66e6e86e3812dcb672a272"),
      ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"),
      ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"),
      ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_10_v2(sample), result);
    }
  }
}
