use itertools::Itertools;

pub fn day_02_v1(input: impl Into<String>) -> usize {
  input
    .into()
    .lines()
    .map(|line| {
      let mut max = 0;
      let mut min = usize::MAX;
      line.split_whitespace().for_each(|number| {
        let number = number.parse::<usize>().unwrap();
        max = std::cmp::max(max, number);
        min = std::cmp::min(min, number);
      });
      max - min
    })
    .sum()
}

pub fn day_02_v2(input: impl Into<String>) -> usize {
  input
    .into()
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .combinations(2)
        .filter_map(|pair| {
          if pair[0] % pair[1] == 0 {
            Some(pair[0] / pair[1])
          } else if pair[1] % pair[0] == 0 {
            Some(pair[1] / pair[0])
          } else {
            None
          }
        })
        .next()
        .unwrap()
    })
    .sum()
}

solvable!(day_02, day_02_v1, day_02_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "5 1 9 5\n\
      7 5 3\n\
      2 4 6 8";
    assert_eq!(day_02_v1(sample_one), 18);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_one = "5 9 2 8\n\
      9 4 7 3\n\
      3 8 6 5";
    assert_eq!(day_02_v2(sample_one), 9);
  }
}
