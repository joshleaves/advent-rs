use super::knot_hash;

fn parse_jumps(input: &str) -> Vec<u32> {
  input
    .trim_end()
    .split(',')
    .map(|number| number.trim().parse::<u32>().unwrap())
    .collect()
}

fn parse_input(input: &str) -> Vec<u32> {
  input
    .trim_end()
    .bytes()
    .map(|number| number as u32)
    .collect()
}

pub fn day_10_v1(input: impl Into<String>) -> String {
  let numbers = knot_hash::_knot_hash(256, &parse_jumps(&input.into()), 1);
  (numbers[0] * numbers[1]).to_string()
}

pub fn day_10_v2(input: impl Into<String>) -> String {
  let mut input = parse_input(&input.into());
  knot_hash::knot_hash(&mut input)
    .iter()
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
    let jumps = parse_jumps("3, 4, 1, 5");
    let numbers = knot_hash::_knot_hash(5, &jumps, 1);
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
