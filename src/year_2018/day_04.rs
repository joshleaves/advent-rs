use itertools::Itertools;
use std::collections::HashMap;

struct Sleep {
  from: (u8, u8),
  to: (u8, u8),
  duration: u16,
}

impl Sleep {
  #[inline]
  fn parse_line(input: &str) -> (u8, u8) {
    input
      .split([' ', '-', ':', ']'])
      .skip(3)
      .take(2)
      .map(|number| number.parse::<u8>().unwrap())
      .collect_tuple()
      .unwrap()
  }

  fn new(input: &mut Vec<&str>) -> Self {
    let input_asleep = Sleep::parse_line(input.remove(0));
    let input_awake = Sleep::parse_line(input.remove(0));

    // Funny: It seems we don't have sleeps overlapping into another hour!
    assert_eq!(input_asleep.0, input_awake.0);
    let duration = input_awake.1 - input_asleep.1;

    Sleep {
      from: (input_asleep.0, input_asleep.1),
      to: (input_awake.0, input_awake.1),
      duration: duration as u16,
    }
  }
}

#[inline]
fn parse_guard(input: &str) -> u16 {
  input
    .split(['#', ' '])
    .nth(4)
    .unwrap()
    .parse::<u16>()
    .unwrap()
}

fn parse_input(input: &str) -> HashMap<u16, Vec<Sleep>> {
  let mut input: Vec<&str> = input.lines().sorted().collect();
  let mut guards: HashMap<u16, Vec<Sleep>> = HashMap::new();
  let mut current_guard_id = 0;
  while !input.is_empty() {
    if input[0].contains('#') {
      current_guard_id = parse_guard(input.remove(0));
      continue;
    }
    let sleep = Sleep::new(&mut input);
    guards.entry(current_guard_id).or_default().push(sleep);
  }

  guards
}

pub fn day_04_v1(input: impl Into<String>) -> u16 {
  let guards = parse_input(&input.into());
  let worst_guard = guards
    .iter()
    .max_by_key(|(_k, v)| v.iter().map(|s| s.duration).sum::<u16>())
    .unwrap();

  let mut minutes: HashMap<u8, u8> = HashMap::new();
  for sleep in worst_guard.1 {
    for minute in sleep.from.1..sleep.to.1 {
      *minutes.entry(minute).or_default() += 1;
    }
  }

  let worst_guard_id = *worst_guard.0 as u16;
  let worst_minute = *minutes.iter().max_by_key(|(_k, v)| *v).unwrap().0 as u16;

  worst_guard_id * worst_minute
}

pub fn day_04_v2(input: impl Into<String>) -> u16 {
  let guards = parse_input(&input.into());
  let mut minutes_guards: HashMap<(u8, u16), u8> = HashMap::new();
  for (guard_id, sleeps) in guards {
    for sleep in sleeps {
      for minute in sleep.from.1..sleep.to.1 {
        *minutes_guards.entry((minute, guard_id)).or_default() += 1;
      }
    }
  }
  let worst_minute_guard = minutes_guards.iter().max_by_key(|(_k, v)| *v).unwrap().0;

  worst_minute_guard.0 as u16 * worst_minute_guard.1
}

solvable!(day_04, day_04_v1, day_04_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "[1518-11-01 00:00] Guard #10 begins shift\n\
    [1518-11-01 00:05] falls asleep\n\
    [1518-11-01 00:25] wakes up\n\
    [1518-11-01 00:30] falls asleep\n\
    [1518-11-01 00:55] wakes up\n\
    [1518-11-01 23:58] Guard #99 begins shift\n\
    [1518-11-02 00:40] falls asleep\n\
    [1518-11-02 00:50] wakes up\n\
    [1518-11-03 00:05] Guard #10 begins shift\n\
    [1518-11-03 00:24] falls asleep\n\
    [1518-11-03 00:29] wakes up\n\
    [1518-11-04 00:02] Guard #99 begins shift\n\
    [1518-11-04 00:36] falls asleep\n\
    [1518-11-04 00:46] wakes up\n\
    [1518-11-05 00:03] Guard #99 begins shift\n\
    [1518-11-05 00:45] falls asleep\n\
    [1518-11-05 00:55] wakes up";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_04_v1(SAMPLE), 240);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_04_v2(SAMPLE), 4_455);
  }
}
