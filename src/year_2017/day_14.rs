use super::knot_hash::knot_hash;
use itertools::Itertools;
use std::collections::VecDeque;

fn generate_rows(input: &str, rows: usize) -> Vec<Vec<u32>> {
  let input = input.as_bytes().to_vec();
  (0..rows)
    .map(|row| {
      let mut value = input.to_owned();
      value.push(b'-');
      value.append(&mut row.to_string().as_bytes().to_owned());
      value.iter().map(|c| *c as u32).collect()
    })
    .collect_vec()
}

#[inline]
fn fill_map(map: &mut [Vec<i32>], starter: (usize, usize), value: i32) {
  let mut positions = VecDeque::from([starter]);
  while let Some(position) = positions.pop_front() {
    if map[position.0][position.1] != -1 {
      continue;
    }
    map[position.0][position.1] = value;
    if position.0 > 0 {
      let up = (position.0 - 1, position.1);
      positions.push_back(up)
    }
    if position.0 + 1 < 128 {
      let down = (position.0 + 1, position.1);
      positions.push_back(down)
    }
    if position.1 > 0 {
      let left = (position.0, position.1 - 1);
      positions.push_back(left)
    }
    if position.1 + 1 < 128 {
      let right = (position.0, position.1 + 1);
      positions.push_back(right)
    }
  }
}

pub fn day_14_v1(input: impl Into<String>) -> u32 {
  let mut rows = generate_rows(input.into().trim_end(), 128);
  rows
    .iter_mut()
    .map(|row| {
      knot_hash(&mut *row)
        .iter()
        .map(|hash| hash.count_ones())
        .sum::<u32>()
    })
    .sum()
}

pub fn day_14_v2(input: impl Into<String>) -> u32 {
  let mut rows = generate_rows(input.into().trim_end(), 128);
  let mut map: Vec<_> = rows
    .iter_mut()
    .map(|row| {
      knot_hash(&mut *row)
        .iter()
        .flat_map(|hash| {
          (0..=7)
            .map(|x| -(((*hash >> x) & 1) as i32))
            .rev()
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
    })
    .collect();
  let mut cnt_groups = 0;
  for y in 0..128 {
    for x in 0..128 {
      if map[y][x] == -1 {
        cnt_groups += 1;
        fill_map(&mut map, (y, x), cnt_groups);
      }
    }
  }
  cnt_groups as u32
}

solvable!(day_14, day_14_v1, day_14_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "flqrgnkx";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_14_v1(SAMPLE), 8108);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_14_v2(SAMPLE), 1242);
  }
}
