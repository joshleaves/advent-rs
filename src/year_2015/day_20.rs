fn parse_input(input: &str) -> usize {
  let Some(first_line) = input.lines().next() else {
    panic!("Invalid input: {}", input)
  };
  let Ok(input_value) = first_line.parse::<usize>() else {
    panic!("Invalid input: {}", input)
  };

  input_value
}

fn divisors_of_v1(number: usize) -> usize {
  let mut value = 0;
  let target_iter = f64::sqrt(number as f64) as usize;
  for num in 1..=target_iter {
    if number % num != 0 {
      continue;
    }
    let num2 = number / num;
    value += num;
    if num2 != num {
      value += num2
    }
  }

  value
}

fn divisors_of_v2(number: usize) -> usize {
  let mut value = 0;
  let target_iter = f64::sqrt(number as f64) as usize;
  for num in 1..=target_iter {
    if number % num != 0 {
      continue;
    }
    let num2 = number / num;
    if num2 < 50 {
      value += num;
    }
    if num2 != num && num < 50 {
      value += num2
    }
  }

  value
}

pub fn day_20_v1(input: impl Into<String>) -> usize {
  let target: usize = parse_input(&input.into());
  let mut houses: usize = 0;
  loop {
    houses += 1;
    let elves = divisors_of_v1(houses) * 10;
    if elves >= target {
      return houses;
    }
  }
}

pub fn day_20_v2(input: impl Into<String>) -> usize {
  let target: usize = parse_input(&input.into());
  let mut houses: usize = 0;
  loop {
    houses += 1;
    let elves = divisors_of_v2(houses) * 11;
    if elves >= target {
      return houses;
    }
  }
}

solvable!(day_20, day_20_v1, day_20_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "150";
    assert_eq!(day_20_v1(sample_one), 8);
  }
}
