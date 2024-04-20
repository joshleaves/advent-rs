fn react_polymer(input: &str) -> u16 {
  input
    .bytes()
    .fold(vec![], |mut acc, chr| {
      if !acc.is_empty() && *acc.last().unwrap() == (chr ^ 32) {
        acc.pop();
      } else {
        acc.push(chr);
      }
      acc
    })
    .len() as u16
}

pub fn day_05_v1(input: impl Into<String>) -> u16 {
  react_polymer(&input.into())
}

pub fn day_05_v2(input: impl Into<String>) -> u16 {
  let input = input.into();
  let maj = 'A'..='Z';
  ('a'..='z')
    .zip(maj)
    .map(|(min, maj)| react_polymer(&input.replace([min, maj], "")))
    .min()
    .unwrap()
}

solvable!(day_05, day_05_v1, day_05_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "dabAcCaCBAcCcaDA";

  #[test]
  fn works_with_samples_v1() {
    let sample_one = [
      ("aA", 0),
      ("abBA", 0),
      ("abAB", 4),
      ("aabAAB", 6),
      (SAMPLE, 10),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_05_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_05_v2(SAMPLE), 4);
  }
}
