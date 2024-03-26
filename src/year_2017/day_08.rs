use std::collections::HashMap;

#[inline]
fn check_conditional(register: i16, conditions: &[&str]) -> bool {
  let value = conditions[1].parse::<i16>().unwrap();
  match conditions[0] {
    ">" => register > value,
    "<" => register < value,
    ">=" => register >= value,
    "<=" => register <= value,
    "==" => register == value,
    "!=" => register != value,
    _ => panic!("Invalid condition: {}", conditions[0]),
  }
}

fn follow_instructions(input: &str) -> (i16, i16) {
  let mut registers: HashMap<&str, i16> = HashMap::new();
  let mut best_max = 0;
  for line in input.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let reg_cond = registers.get(parts[4]).unwrap_or(&0);
    if check_conditional(*reg_cond, &parts[5..]) {
      let reg_target = registers.entry(parts[0]).or_default();
      let value = parts[2].parse::<i16>().unwrap();
      match parts[1] {
        "inc" => *reg_target += value,
        "dec" => *reg_target -= value,
        _ => panic!("Invalid instruction: {}", parts[1]),
      }
      best_max = std::cmp::max(best_max, *reg_target);
    }
  }

  (*registers.values().max().unwrap(), best_max)
}

pub fn day_08_v1(input: impl Into<String>) -> i16 {
  follow_instructions(&input.into()).0
}

pub fn day_08_v2(input: impl Into<String>) -> i16 {
  follow_instructions(&input.into()).1
}

solvable!(day_08, day_08_v1, day_08_v2, i16);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "b inc 5 if a > 1\n\
    a inc 1 if b < 5\n\
    c dec -10 if a >= 1\n\
    c inc -20 if c == 10";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_08_v1(SAMPLE), 1);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_08_v2(SAMPLE), 10);
  }
}
