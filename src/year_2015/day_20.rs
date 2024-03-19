//! Advent of Code 2015: Day 20: Infinite Elves and Infinite Houses

const MAX_HOUSES: usize = 900_000;

fn count_gifts(
  target_gifts: usize,
  starter: usize,
  ender: usize,
  gifts: usize,
  limiter: usize,
) -> usize {
  let mut houses: Vec<usize> = vec![0; ender];
  for elf_idx in starter..=ender {
    let mut house_idx = elf_idx;
    for _ in 1..=limiter {
      if house_idx >= ender {
        break;
      }
      houses[house_idx] += gifts * elf_idx;
      if houses[house_idx] >= target_gifts {
        return elf_idx;
      }
      house_idx += elf_idx;
    }
  }

  0
}

pub fn day_20_v1(input: impl Into<String>) -> usize {
  let target_gifts = input.into().trim_end().parse::<usize>().unwrap();
  count_gifts(target_gifts, 1, MAX_HOUSES, 10, MAX_HOUSES)
}

pub fn day_20_v2(input: impl Into<String>) -> usize {
  let target_gifts = input.into().trim_end().parse::<usize>().unwrap();
  count_gifts(target_gifts, 19_000, MAX_HOUSES, 11, 50)
}

solvable!(day_20, day_20_v1, day_20_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(count_gifts(150, 1, 10, 10, 10), 8);
  }
}
