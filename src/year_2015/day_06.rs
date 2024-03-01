use regex::Regex;

#[inline]
fn parse_number(number: &str) -> u16 {
  let Ok(parsed) = number.parse::<u16>() else {
    panic!("Invalid number: {}", number);
  };
  return parsed;
}

fn parse_coordinates(coordinates: &str) -> (u16, u16) {
  let Some(numbers) = coordinates.split_once(',') else {
    panic!("Invalid coordinates: {}", coordinates);
  };
  return (parse_number(numbers.0), parse_number(numbers.1));
}

fn modify_grid<F>(light_grid: &mut Vec<u8>, from: &str, to: &str, modifier: F)
where
  F: Fn(&[u8], usize) -> Vec<u8>,
{
  let (from_y, from_x) = parse_coordinates(from);
  let (to_y, to_x) = parse_coordinates(to);
  for y in from_y..=to_y {
    let idx_from = y as usize * 1000 + from_x as usize;
    let idx_to = y as usize * 1000 + to_x as usize;
    let oldval = &light_grid[idx_from..=idx_to];
    let newval = modifier(&oldval, oldval.len());

    light_grid.splice(idx_from..=idx_to, newval.iter().cloned());
  }
}

pub fn day_06_v1(input: impl Into<String>) -> u32 {
  let re = Regex::new(r"(?<inst>on|off|toggle) (?<from>\d+,\d+) through (?<to>\d+,\d+)").unwrap();
  let mut light_grid: Vec<u8> = vec![0_u8; 1_000_000];
  for line in input.into().lines() {
    let Some(caps) = re.captures(line) else {
      panic!("Invalid instruction: {}", line)
    };

    match &caps["inst"] {
      "on" => modify_grid(
        &mut light_grid,
        &caps["from"],
        &caps["to"],
        |_lights: &[u8], length: usize| vec![1u8; length],
      ),
      "off" => modify_grid(
        &mut light_grid,
        &caps["from"],
        &caps["to"],
        |_lights: &[u8], length: usize| vec![0u8; length],
      ),
      "toggle" => modify_grid(
        &mut light_grid,
        &caps["from"],
        &caps["to"],
        |lights: &[u8], _length: usize| {
          lights
            .iter()
            .map(|c| if *c == 1 as u8 { 0 } else { 1 })
            .collect()
        },
      ),
      _ => panic!("Invalid instruction: {}", line),
    }
  }

  return light_grid.iter().map(|&i| i as u32).sum();
}

pub fn day_06_v2(input: impl Into<String>) -> u32 {
  let re = Regex::new(r"(?<inst>on|off|toggle) (?<from>\d+,\d+) through (?<to>\d+,\d+)").unwrap();
  let mut light_grid: Vec<u8> = vec![0_u8; 1_000_000];
  for line in input.into().lines() {
    let Some(caps) = re.captures(line) else {
      panic!("Invalid instruction: {}", line)
    };

    match &caps["inst"] {
      "on" => modify_grid(
        &mut light_grid,
        &caps["from"],
        &caps["to"],
        |lights: &[u8], _length: usize| lights.iter().map(|c| *c + 1).collect(),
      ),
      "off" => modify_grid(
        &mut light_grid,
        &caps["from"],
        &caps["to"],
        |lights: &[u8], _length: usize| {
          lights
            .iter()
            .map(|c| {
              if let Some(res) = c.checked_sub(1) {
                res
              } else {
                0
              }
            })
            .collect()
        },
      ),
      "toggle" => modify_grid(
        &mut light_grid,
        &caps["from"],
        &caps["to"],
        |lights: &[u8], _length: usize| lights.iter().map(|c| *c + 2).collect(),
      ),
      _ => panic!("Invalid instruction: {}", line),
    }
  }

  return light_grid.iter().map(|&i| i as u32).sum();
}

solvable!(day_06, day_06_v1, day_06_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = r#"turn on 0,0 through 999,999
    toggle 0,0 through 999,0
    turn off 499,499 through 500,500"#;
    assert_eq!(day_06_v1(sample_one), 998_996);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = r#"turn on 0,0 through 0,0
    toggle 0,0 through 999,999"#;
    assert_eq!(day_06_v2(sample_two), 2_000_001);
  }
}
