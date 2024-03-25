pub fn day_01_v1(input: impl Into<String>) -> usize {
  let mut characters = input
    .into()
    .trim_end()
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<_>>();
  characters.push(characters[0]);
  characters
    .windows(2)
    .filter(|pair| pair[0] == pair[1])
    .map(|pair| (pair[0] - 48) as usize)
    .sum()
}

pub fn day_01_v2(input: impl Into<String>) -> usize {
  let characters = input
    .into()
    .trim_end()
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<_>>();
  let length = characters.len();
  characters
    .iter()
    .enumerate()
    .filter(|(idx, chr)| **chr == characters[(idx + length / 2) % length])
    .map(|(_idx, chr)| (*chr - 48) as usize)
    .sum()
}

solvable!(day_01, day_01_v1, day_01_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, usize); 4] = [
      ("1122", 3),
      ("1111", 4),
      ("1234", 0),
      ("91212129", 9),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_01_v1(sample), result);
    }
  }
  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, usize); 5] = [
      ("1212", 6),
      ("1221", 0),
      ("123425", 4),
      ("123123", 12),
      ("12131415", 4),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_01_v2(sample), result);
    }
  }
}
