use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u16, u16)> {
  input
    .lines()
    .map(|line| {
      line
        .split('/')
        .map(|n| n.parse::<u16>().unwrap())
        .collect_tuple::<(u16, u16)>()
        .unwrap()
    })
    .collect::<Vec<(u16, u16)>>()
}

fn sub_best_path_v1(need: u16, current: u16, parts: &[(u16, u16)]) -> u16 {
  let mut best_path = current;
  for (pidx, part) in parts.iter().enumerate() {
    let next_need = if need == part.0 {
      part.1
    } else if need == part.1 {
      part.0
    } else {
      continue;
    };
    let mut next_parts = parts.to_owned();
    next_parts.remove(pidx);
    let new_best = sub_best_path_v1(next_need, current + part.0 + part.1, &next_parts);
    best_path = std::cmp::max(best_path, new_best);
  }
  best_path
}

fn sub_best_path_v2(need: u16, len: u16, current: u16, parts: &[(u16, u16)]) -> (u16, u16) {
  let mut best_path = (len, current);
  for (pidx, part) in parts.iter().enumerate() {
    let next_need = if need == part.0 {
      part.1
    } else if need == part.1 {
      part.0
    } else {
      continue;
    };
    let mut next_parts = parts.to_owned();
    next_parts.remove(pidx);
    let new_best = sub_best_path_v2(next_need, len + 1, current + part.0 + part.1, &next_parts);
    if new_best.0 > best_path.0 || new_best.0 == best_path.0 && new_best.1 > best_path.1 {
      best_path = new_best;
    }
  }
  best_path
}

pub fn day_24_v1(input: impl Into<String>) -> u16 {
  let parts = parse_input(&input.into());
  let mut best_path = 0;
  for (pidx, part) in parts.iter().enumerate() {
    let next_needed = if part.0 == 0 {
      part.1
    } else if part.1 == 0 {
      part.0
    } else {
      continue;
    };
    let mut next_parts = parts.clone();
    next_parts.remove(pidx);
    let next_best = sub_best_path_v1(next_needed, part.0 + part.1, &next_parts);
    best_path = std::cmp::max(best_path, next_best);
  }
  best_path
}

pub fn day_24_v2(input: impl Into<String>) -> u16 {
  let parts = parse_input(&input.into());
  let mut best_path = (0, 0);
  for (pidx, part) in parts.iter().enumerate() {
    let next_needed = if part.0 == 0 {
      part.1
    } else if part.1 == 0 {
      part.0
    } else {
      continue;
    };
    let mut next_parts = parts.clone();
    next_parts.remove(pidx);
    let (new_len, new_best) = sub_best_path_v2(next_needed, 1, part.0 + part.1, &next_parts);
    if new_len > best_path.0 || new_len == best_path.0 && new_best > best_path.1 {
      best_path = (new_len, new_best);
    }
  }
  best_path.1
}
solvable!(day_24, day_24_v1, day_24_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "0/2\n\
    2/2\n\
    2/3\n\
    3/4\n\
    3/5\n\
    0/1\n\
    10/1\n\
    9/10";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_24_v1(SAMPLE), 31);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_24_v2(SAMPLE), 19);
  }
}
