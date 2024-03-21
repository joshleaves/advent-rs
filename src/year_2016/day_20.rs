use itertools::Itertools;

fn parse_ranges(input: &str) -> Vec<(u32, u32)> {
  let mut results = input
    .lines()
    .map(|line| {
      line
        .split("-")
        .map(|part| part.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
    })
    .collect_vec();
  results.sort();

  results
}
fn calculate_available_addresses(ranges: Vec<(u32, u32)>, total: u32) -> u32 {
  let mut in_gaps = ranges[0].0;
  let mut highest_blocked = ranges[0].1;
  for (low, high) in ranges {
    if low > (highest_blocked) {
      in_gaps += low - highest_blocked - 1;
    }
    highest_blocked = std::cmp::max(highest_blocked, high);
  }

  total - highest_blocked + in_gaps
}

pub fn day_20_v1(input: impl Into<String>) -> u32 {
  let ranges = parse_ranges(&input.into());
  let mut low_ip = 0;
  let mut idx = 0;
  while idx < ranges.len() && ranges[idx].0 <= low_ip {
    let (_range_starting, range_ending) = ranges[idx];
    low_ip = std::cmp::max(low_ip, range_ending + 1);
    idx += 1;
  }
  low_ip
}

pub fn day_20_v2(input: impl Into<String>) -> u32 {
  let ranges = parse_ranges(&input.into());
  calculate_available_addresses(ranges, 4_294_967_295)
}

solvable!(day_20, day_20_v1, day_20_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "5-8\n0-2\n4-7";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_20_v1(SAMPLE), 3);
  }

  #[test]
  fn works_with_samples_v2() {
    let ranges = parse_ranges(SAMPLE);
    assert_eq!(calculate_available_addresses(ranges, 9), 2);
  }
}
