use std::collections::HashMap;

// Thanks https://stackoverflow.com/a/61253346
fn spiral(input: u32) -> (i32, i32) {
  let n = input as f64;
  let k = ((n.sqrt() - 1f64) / 2f64).ceil();
  let mut t = (2f64 * k) + 1f64;
  let mut m = t.powf(2f64);
  t -= 1f64;
  if n >= (m - t) {
    return ((k - (m - n)) as i32, -k as i32);
  } else {
    m -= t;
  }
  if n >= (m - t) {
    return (-k as i32, (-k + (m - n)) as i32);
  } else {
    m -= t;
  }
  if n >= (m - t) {
    return ((-k + (m - n)) as i32, k as i32);
  }

  (k as i32, (k - (m - n - t)) as i32)
}

pub fn day_03_v1(input: impl Into<String>) -> u32 {
  let number = input.into().trim_end().parse::<u32>().unwrap();
  let position = spiral(number);

  (position.0.unsigned_abs() + position.1.unsigned_abs()) as u32
}

#[inline]
fn get_neighbors(values: &HashMap<(i32, i32), u32>, position: (i32, i32)) -> u32 {
  let mut result: u32 = 0;
  for position_x in position.0 - 1i32..=position.0 + 1i32 {
    for position_y in position.1 - 1i32..=position.1 + 1i32 {
      if let Some(position_value) = values.get(&(position_x, position_y)) {
        result += *position_value;
      }
    }
  }
  result
}

pub fn day_03_v2(input: impl Into<String>) -> u32 {
  let target = input.into().trim_end().parse::<u32>().unwrap();
  let mut values: HashMap<(i32, i32), u32> = HashMap::new();
  values.insert((0i32, 0i32), 1);
  let mut counter = 1;
  loop {
    counter += 1;
    let position = spiral(counter);
    let position_value = get_neighbors(&values, position);
    if position_value > target {
      return position_value;
    }
    values.insert(position, position_value);
  }
}

solvable!(day_03, day_03_v1, day_03_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, u32); 4] = [
      ("1", 0),
      ("12", 3),
      ("23", 2),
      ("1024", 31),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_03_v1(sample), result);
    }
  }
}
