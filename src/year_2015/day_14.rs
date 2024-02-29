use regex::Regex;
const RACE_DURATION: u16 = 2503;

#[derive(PartialEq)]
enum ReindeerState {
  Running,
  Resting,
}

struct Reindeer {
  speed: u16,
  length: u16,
  rest: u16,
  curpos: u16,
  state: ReindeerState,
  progress: u16,
  points: u16,
}

impl Reindeer {
  fn switch_state(&mut self, new_state: ReindeerState) {
    self.state = new_state;
    self.progress = 0;
  }

  fn advance(&mut self) {
    self.progress += 1;
    match &self.state {
      ReindeerState::Running => {
        self.curpos += self.speed;
        if self.progress == self.length {
          self.switch_state(ReindeerState::Resting);
        }
      }
      ReindeerState::Resting => {
        if self.progress == self.rest {
          self.switch_state(ReindeerState::Running);
        }
      }
    }
  }
}

fn parse_line(input: &str) -> Reindeer {
  let parser: Regex = Regex::new(
    r#"\w+ can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds\."#,
  )
  .unwrap();
  let Some(captures) = parser.captures(input) else {
    panic!("Invalid input: {}", input);
  };
  let speed: u16 = captures[1].parse::<u16>().unwrap();
  let length: u16 = captures[2].parse::<u16>().unwrap();
  let rest: u16 = captures[3].parse::<u16>().unwrap();

  Reindeer {
    speed: speed,
    length: length,
    rest: rest,
    curpos: 0,
    state: ReindeerState::Running,
    progress: 0,
    points: 0,
  }
}

fn race(reindeers: &mut Vec<Reindeer>, duration: u16) {
  for _i in 0..=duration {
    let mut pole_position = 0;
    for deer in reindeers.iter_mut() {
      deer.advance();
      pole_position = std::cmp::max(pole_position, deer.curpos);
    }
    for deer in reindeers.iter_mut() {
      if deer.curpos == pole_position {
        deer.points += 1;
      }
    }
  }
}

pub fn day_14_v1(input: impl Into<String>) -> u16 {
  let mut reindeers: Vec<Reindeer> = vec![];
  for line in input.into().lines() {
    reindeers.push(parse_line(line));
  }
  race(&mut reindeers, RACE_DURATION);

  reindeers.iter().map(|deer| deer.curpos).max().unwrap()
}

pub fn day_14_v2(input: impl Into<String>) -> u16 {
  let mut reindeers: Vec<Reindeer> = vec![];
  for line in input.into().lines() {
    reindeers.push(parse_line(line));
  }
  race(&mut reindeers, RACE_DURATION);
  reindeers.iter().map(|deer| deer.points).max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let mut reindeers: Vec<Reindeer> = vec![
      parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
      parse_line("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."),
    ];
    race(&mut reindeers, 1000);
    assert_eq!(
      reindeers.iter().map(|deer| deer.curpos).max().unwrap(),
      1120
    );
  }

  #[test]
  fn works_with_samples_v2() {
    let mut reindeers: Vec<Reindeer> = vec![
      parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
      parse_line("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."),
    ];
    race(&mut reindeers, 1000);
    assert_eq!(reindeers.iter().map(|deer| deer.points).max().unwrap(), 689);
  }
}
