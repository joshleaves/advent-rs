use std::collections::HashMap;

// Thanks https://stackoverflow.com/a/61253346
fn spiral(input: usize) -> (i64, i64) {
  let n = input as f64;
  let k = ((n.sqrt() - 1f64) / 2f64).ceil();
  let mut t = (2f64 * k) + 1f64;
  let mut m = t.powf(2f64);
  t -= 1f64;
  if n >= (m - t) {
    return ((k - (m - n)) as i64, -k as i64);
  } else {
    m -= t;
  }
  if n >= (m - t) {
    return (-k as i64, (-k + (m - n)) as i64);
  } else {
    m -= t;
  }
  if n >= (m - t) {
    return ((-k + (m - n)) as i64, k as i64);
  }

  (k as i64, (k - (m - n - t)) as i64)
}

pub fn day_03_v1(input: impl Into<String>) -> usize {
  let number = input.into().trim_end().parse::<usize>().unwrap();
  let position = spiral(number);

  (position.0.unsigned_abs() + position.1.unsigned_abs()) as usize
}

#[inline]
fn get_neighbors(values: &HashMap<(i64, i64), usize>, position: (i64, i64)) -> usize {
  let mut result: usize = 0;
  for position_x in position.0 - 1i64..=position.0 + 1i64 {
    for position_y in position.1 - 1i64..=position.1 + 1i64 {
      if let Some(position_value) = values.get(&(position_x, position_y)) {
        result += *position_value;
      }
    }
  }
  result
}

pub fn day_03_v2(input: impl Into<String>) -> usize {
  let target = input.into().trim_end().parse::<usize>().unwrap();
  let mut values: HashMap<(i64, i64), usize> = HashMap::new();
  values.insert((0i64, 0i64), 1);
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

solvable!(day_03, day_03_v1, day_03_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, usize); 4] = [
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
