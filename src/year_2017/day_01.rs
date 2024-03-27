pub fn day_01_v1(input: impl Into<String>) -> u16 {
  let mut characters = input
    .into()
    .trim_end()
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<_>>();
  characters.push(characters[0]);
  characters
    .windows(2)
    .filter_map(|pair| {
      if pair[0] == pair[1] {
        Some((pair[0] - 48) as u16)
      } else {
        None
      }
    })
    .sum()
}

pub fn day_01_v2(input: impl Into<String>) -> u16 {
  let characters = input
    .into()
    .trim_end()
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<_>>();
  let mut characters_rot = characters.clone();
  characters_rot.rotate_right(characters.len() / 2);
  characters
    .iter()
    .zip(characters_rot.iter())
    .filter_map(|(lhs, rhs)| {
      if lhs == rhs {
        Some((lhs - 48) as u16)
      } else {
        None
      }
    })
    .sum()
}

solvable!(day_01, day_01_v1, day_01_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u16); 4] = [
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
    let sample_two: [(&str, u16); 5] = [
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
