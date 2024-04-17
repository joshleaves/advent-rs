use itertools::Itertools;

#[inline]
fn parse_line(input: &str) -> (u16, u16, u16, u16, u16) {
  let (id, input) = input.split_once(" @ ").unwrap();
  let id = id.strip_prefix('#').unwrap().parse::<u16>().unwrap();
  let (position, size) = input.split_once(": ").unwrap();
  let (x, y) = position
    .split(',')
    .map(|num| num.parse::<u16>().unwrap())
    .collect_tuple()
    .unwrap();
  let (width, height) = size
    .split('x')
    .map(|num| num.parse::<u16>().unwrap())
    .collect_tuple()
    .unwrap();
  (id, x, y, width, height)
}

pub fn day_03_v1(input: impl Into<String>) -> u32 {
  let mut fabric = vec![0; 1_000_000];
  input
    .into()
    .lines()
    .map(parse_line)
    .for_each(|(_id, x, y, width, height)| {
      for i in x..x + width {
        for j in y..y + height {
          fabric[(j as usize * 1000usize) + i as usize] += 1;
        }
      }
    });

  fabric.iter().filter(|number| number >= &&2).count() as u32
}

pub fn day_03_v2(input: impl Into<String>) -> u32 {
  let mut fabric = vec![0; 1_000_000];
  let lines: Vec<_> = input.into().lines().map(parse_line).collect();
  lines.iter().for_each(|(_id, x, y, width, height)| {
    for i in *x..*x + *width {
      for j in *y..*y + *height {
        fabric[(j as usize * 1000usize) + i as usize] += 1;
      }
    }
  });
  lines
    .iter()
    .find_map(|(id, x, y, width, height)| {
      for i in *x..*x + *width {
        for j in *y..*y + *height {
          if fabric[(j as usize * 1000usize) + i as usize] > 1 {
            return None;
          }
        }
      }
      Some(*id as u32)
    })
    .unwrap()
}

solvable!(day_03, day_03_v1, day_03_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "#1 @ 1,3: 4x4\n\
    #2 @ 3,1: 4x4\n\
    #3 @ 5,5: 2x2";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_03_v1(SAMPLE), 4);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_03_v2(SAMPLE), 3);
  }
}
