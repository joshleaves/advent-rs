use std::collections::HashSet;

pub fn day_01_v1(input: impl Into<String>) -> i32 {
  input
    .into()
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .sum()
}

pub fn day_01_v2(input: impl Into<String>) -> i32 {
  let mut numset: HashSet<i32> = HashSet::new();
  input
    .into()
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .cycle()
    .try_fold(0, |cursor, number| match numset.insert(cursor) {
      true => Ok(cursor + number),
      false => Err(cursor),
    })
    .unwrap_err()

  // let mut numset: HashSet<i32> = HashSet::from([0]);
  // let mut cursor: i32 = 0;
  // input
  //   .into()
  //   .lines()
  //   .map(|line| line.parse::<i32>().unwrap())
  //   .cycle()
  //   .try_for_each(|number| {
  //     cursor += number;
  //     match numset.insert(cursor) {
  //       true => Some(()),
  //       false => None,
  //     }
  //   });
  // cursor
}

solvable!(day_01, day_01_v1, day_01_v2, i32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, i32); 4] = [
      ("+1\n-2\n+3\n+1", 3),
      ("+1\n+1\n+1", 3),
      ("+1\n+1\n-2", 0),
      ("-1\n-2\n-3", -6),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_01_v1(sample), result);
    }
  }
  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, i32); 5] = [
      ("+1\n-2\n+3\n+1", 2),
      ("+1\n-1", 0),
      ("+3\n+3\n+4\n-2\n-4", 10),
      ("-6\n+3\n+8\n+5\n-6", 5),
      ("+7\n+7\n-2\n-7\n-4", 14),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_01_v2(sample), result);
    }
  }
}
