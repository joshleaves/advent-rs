use itertools::Itertools;

pub fn day_02_v1(input: impl Into<String>) -> String {
  let chk = input
    .into()
    .lines()
    .map(|input| {
      let mut two = false;
      let mut tre = false;
      for (_chr, cnt) in &input.bytes().sorted().chunk_by(|chr| *chr) {
        match cnt.count() {
          2 => two = true,
          3 => tre = true,
          _ => (),
        }
      }
      (two, tre)
    })
    .fold((0, 0), |mut acc, elt| {
      if elt.0 {
        acc.0 += 1;
      }
      if elt.1 {
        acc.1 += 1;
      }
      acc
    });
  format!("{}", chk.0 * chk.1)
}

fn common_characters(lhs: &str, rhs: &str) -> Option<String> {
  let mut one_diff = false;
  lhs
    .chars()
    .zip(rhs.chars())
    .try_fold("".to_string(), |mut acc, (lhc, rhc)| {
      if lhc != rhc && one_diff {
        None
      } else if lhc != rhc {
        one_diff = true;
        Some(acc)
      } else {
        acc.push(lhc);
        Some(acc)
      }
    })
}

pub fn day_02_v2(input: impl Into<String>) -> String {
  let input = input.into();
  let input: Vec<_> = input.lines().collect();
  for i in 0..input.len() {
    for j in i + 1..input.len() {
      if let Some(common) = common_characters(input[i], input[j]) {
        return common;
      }
    }
  }
  "".to_string()
}
solvable!(day_02, day_02_v1, day_02_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "abcdef\n\
      bababc\n\
      abbcde\n\
      abcccd\n\
      aabcdd\n\
      abcdee\n\
      ababab";
    assert_eq!(day_02_v1(sample_one), "12");
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "abcde\n\
      fghij\n\
      klmno\n\
      pqrst\n\
      fguij\n\
      axcye\n\
      wvxyz";
    assert_eq!(day_02_v2(sample_two), "fgij");
  }
}
