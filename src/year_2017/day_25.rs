use core::fmt;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Copy)]
struct Execution {
  write_value: bool,
  direction: bool,
  next_state: u8,
}

impl Execution {
  fn new(input: Vec<&str>) -> Self {
    assert_eq!(input.len(), 3);
    let mut write_value = input[0].split_whitespace();
    assert_eq!(write_value.nth(1).unwrap(), "Write");
    let mut direction = input[1].split_whitespace();
    assert_eq!(direction.nth(1).unwrap(), "Move");
    let mut next_state = input[2].split_whitespace();
    assert_eq!(next_state.nth(1).unwrap(), "Continue");

    let write_value = matches!(write_value.nth(2).unwrap(), "1.");
    let direction = matches!(direction.nth(4).unwrap(), "right.");
    let next_state = next_state.nth(2).unwrap().as_bytes()[0] - b'A';

    Execution {
      write_value,
      direction,
      next_state,
    }
  }
}

struct State {
  false_exec: Execution,
  true_exec: Execution,
}

impl State {
  fn new(input: Vec<&str>) -> Self {
    assert!(input.len() >= 9);
    let in_state = input[0];
    assert_eq!(in_state.split_whitespace().count(), 3);
    let false_state = input[1];
    assert_eq!(false_state.split_whitespace().count(), 6);
    let true_state = input[5];
    assert_eq!(true_state.split_whitespace().count(), 6);

    let false_exec = Execution::new(input[2..=4].to_vec());
    let true_exec = Execution::new(input[6..=8].to_vec());

    State {
      false_exec,
      true_exec,
    }
  }

  fn exec(&self, value: bool) -> Execution {
    match value {
      true => self.true_exec,
      false => self.false_exec,
    }
  }
}

struct StateMachine {
  stopper: u32,
  current_state: u8,
  states: Vec<State>,
  tape: HashMap<i16, bool>,
  index: i16,
}

impl StateMachine {
  fn new(input: &str) -> Self {
    let mut lines = input.lines();
    let current_state: Vec<_> = lines
      .next()
      .unwrap()
      .strip_suffix('.')
      .unwrap()
      .split_whitespace()
      .collect();
    assert_eq!(current_state.len(), 4);
    let current_state = current_state[3].as_bytes()[0] - b'A';
    let stopper: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    assert_eq!(stopper.len(), 7);
    let stopper = stopper[5].parse::<u32>().unwrap();
    lines.next();
    let states = lines
      .chunks(10)
      .into_iter()
      .map(|chunk| State::new(chunk.collect_vec()))
      .collect();

    StateMachine {
      stopper,
      states,
      current_state,
      tape: HashMap::new(),
      index: 0i16,
    }
  }

  fn _execute(&mut self) {
    let entry = self.tape.entry(self.index).or_insert(false);
    let exec = self.states[self.current_state as usize].exec(*entry);
    *entry = exec.write_value;
    self.index = match exec.direction {
      true => self.index + 1,
      false => self.index - 1,
    };
    self.current_state = exec.next_state;
    self.stopper -= 1;
  }

  fn run(&mut self) {
    while self.stopper > 0 {
      self._execute();
    }
  }
}

impl fmt::Display for StateMachine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (_idx, value) in self.tape.iter().sorted() {
      match value {
        true => write!(f, "1 ")?,
        false => write!(f, "0 ")?,
      }
    }
    Ok(())
  }
}

pub fn day_25(input: impl Into<String>) -> u64 {
  let mut state_machine = StateMachine::new(&input.into());
  state_machine.run();
  state_machine.tape.values().filter(|v| **v).count() as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "Begin in state A.\n\
    Perform a diagnostic checksum after 6 steps.\n\
    \n\
    In state A:\n\
      If the current value is 0:\n\
        - Write the value 1.\n\
        - Move one slot to the right.\n\
        - Continue with state B.\n\
      If the current value is 1:\n\
        - Write the value 0.\n\
        - Move one slot to the left.\n\
        - Continue with state B.\n\
    \n\
    In state B:\n\
      If the current value is 0:\n\
        - Write the value 1.\n\
        - Move one slot to the left.\n\
        - Continue with state A.\n\
      If the current value is 1:\n\
        - Write the value 1.\n\
        - Move one slot to the right.\n\
        - Continue with state A.";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_25(SAMPLE), 3);
  }
}
