use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

type Coordinate = (i32, i32);

struct ChronalCoordinates {
  points: HashMap<u8, Coordinate>,
  left: i32,
  top: i32,
  right: i32,
  bottom: i32,
}

impl ChronalCoordinates {
  fn new(input: &str) -> Self {
    let mut left = i32::MAX;
    let mut top = i32::MAX;
    let mut right = i32::MIN;
    let mut bottom = i32::MIN;

    let points = input
      .lines()
      .enumerate()
      .map(|(line_id, line)| {
        let line_id = line_id as u8 + 1u8;
        let coord: Coordinate = line
          .split(", ")
          .map(|v| v.parse::<i32>().unwrap())
          .collect_tuple()
          .unwrap();
        left = std::cmp::min(left, coord.0);
        top = std::cmp::min(top, coord.1);
        right = std::cmp::max(right, coord.0);
        bottom = std::cmp::max(bottom, coord.1);
        (line_id, coord)
      })
      .collect::<HashMap<u8, Coordinate>>();

    ChronalCoordinates {
      points,
      left,
      top,
      right,
      bottom,
    }
  }

  fn solve_part_one(&self) -> u32 {
    let mut countable_ids: HashMap<u8, u32> = HashMap::new();
    let mut infinite_ids: HashSet<u8> = HashSet::new();

    for x in self.left..=self.right {
      for y in self.top..=self.bottom {
        let mut closest_distance = i32::MAX;
        let mut closest_id = 0;

        for (id, point) in self.points.iter() {
          let distance = (x - point.0).abs() + (y - point.1).abs();

          match distance.cmp(&closest_distance) {
            Ordering::Greater => (),
            Ordering::Equal => closest_id = 0,
            Ordering::Less => {
              closest_distance = distance;
              closest_id = *id;
            }
          }

          if closest_id == 0 {
            continue;
          }
        }

        if x == self.left || x == self.right || y == self.top || y == self.bottom {
          infinite_ids.insert(closest_id);
          countable_ids.remove(&closest_id);
        } else if !infinite_ids.contains(&closest_id) {
          *countable_ids.entry(closest_id).or_insert(0) += 1;
        }
      }
    }

    *countable_ids.iter().max_by_key(|(_, v)| *v).unwrap().1
  }

  fn solve_part_two(&self, max_size: i32) -> u32 {
    let mut area = 0;

    for x in self.left..=self.right {
      for y in self.top..=self.bottom {
        let distance = self.points.iter().fold(0, |acc, (_, point)| {
          acc + (x - point.0).abs() + (y - point.1).abs()
        });
        if distance < max_size {
          area += 1;
        }
      }
    }

    area
  }
}

pub fn day_06_v1(input: impl Into<String>) -> u32 {
  ChronalCoordinates::new(&input.into()).solve_part_one()
}
pub fn day_06_v2(input: impl Into<String>) -> u32 {
  ChronalCoordinates::new(&input.into()).solve_part_two(10_000)
}

solvable!(day_06, day_06_v1, day_06_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "1, 1\n\
    1, 6\n\
    8, 3\n\
    3, 4\n\
    5, 5\n\
    8, 9";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_06_v1(SAMPLE), 17);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(ChronalCoordinates::new(SAMPLE).solve_part_two(32), 16);
  }
}
