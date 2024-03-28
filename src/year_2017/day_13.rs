use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
  input
    .lines()
    .map(|line| {
      let tuple: (i32, i32) = line
        .split(": ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
      (tuple.0, tuple.1, (tuple.1 - 1) * 2)
    })
    .collect()
}

pub fn day_13_v1(input: impl Into<String>) -> u32 {
  let firewalls = parse_input(&input.into());
  let mut severity: u32 = 0;
  for (depth, range, position) in firewalls.iter() {
    if *depth == 0 || (depth % position) == 0 {
      severity += *range as u32 * *depth as u32;
    }
  }

  severity
}

#[inline]
fn traverse_firewall(firewalls: &[(i32, i32)], delay: u32) -> bool {
  for (depth, range) in firewalls.iter() {
    if ((*depth as u32 + delay) % *range as u32) == 0 {
      return false;
    }
  }
  true
}

pub fn day_13_v2(input: impl Into<String>) -> u32 {
  let firewalls: Vec<_> = input
    .into()
    .lines()
    .map(|line| {
      let mut tuple: (i32, i32) = line
        .split(": ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
      tuple.1 = (tuple.1 - 1) * 2;
      tuple
    })
    .collect();
  let mut delay = 0;
  while !traverse_firewall(&firewalls, delay) {
    delay += 1;
  }
  delay
}

solvable!(day_13, day_13_v1, day_13_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "0: 3\n\
    1: 2\n\
    4: 4\n\
    6: 4";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_13_v1(SAMPLE), 24);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_13_v2(SAMPLE), 10);
  }
}
