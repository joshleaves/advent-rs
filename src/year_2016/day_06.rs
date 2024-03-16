use itertools::Itertools;
use std::cmp::Ordering;

fn build_result<F>(input: impl Into<String>, sort_closure: F) -> String
where
  F: Fn(&(&char, usize), &(&char, usize)) -> Ordering,
{
  let mut chars: Vec<Vec<char>> = vec![];

  for line in input.into().lines() {
    for (idx, chr) in line.chars().enumerate() {
      if chars.len() == idx {
        chars.push(vec![]);
      }
      chars[idx].push(chr);
    }
  }
  chars
    .iter()
    .map(|row| {
      let (next_char, _) = row
        .iter()
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .max_by(&sort_closure)
        .unwrap();
      next_char
    })
    .join("")
}

pub fn day_06_v1(input: impl Into<String>) -> String {
  build_result(input, |&lhs, &rhs| lhs.1.cmp(&rhs.1))
}

pub fn day_06_v2(input: impl Into<String>) -> String {
  build_result(input, |&lhs, &rhs| rhs.1.cmp(&lhs.1))
}

solvable!(day_06, day_06_v1, day_06_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "eedadn\n\
    drvtee\n\
    eandsr\n\
    raavrd\n\
    atevrs\n\
    tsrnev\n\
    sdttsa\n\
    rasrtv\n\
    nssdts\n\
    ntnada\n\
    svetve\n\
    tesnvt\n\
    vntsnd\n\
    vrdear\n\
    dvrsen\n\
    enarar";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_06_v1(SAMPLE), "easter");
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_06_v2(SAMPLE), "advent");
  }
}
