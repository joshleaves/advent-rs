use std::collections::HashMap;

#[derive(Debug, Default)]
struct Instructions {
  todo: Vec<char>,
  done: Vec<char>,
  steps: HashMap<char, Vec<char>>,
}

impl Instructions {
  pub fn new(input: String) -> Self {
    let mut newself = Self::default();
    for line in input.lines() {
      let tokens: Vec<&str> = line.split_whitespace().collect();
      assert!(tokens.len() == 10);
      let mut step = tokens[7].chars();
      assert!(tokens[7].len() == 1);
      let mut deps = tokens[1].chars();
      assert!(tokens[1].len() == 1);

      let char_step = step.next().unwrap();
      let char_deps = deps.next().unwrap();

      if !newself.todo.contains(&char_step) {
        newself.todo.push(char_step);
      }
      if !newself.todo.contains(&char_deps) {
        newself.todo.push(char_deps);
      }
      newself.steps.entry(char_step).or_default().push(char_deps);
    }
    newself.todo.sort();
    newself.todo.dedup();
    newself
  }

  fn get_next_step(&self) -> Option<char> {
    if self.todo.is_empty() {
      return None;
    }

    for step in &self.todo {
      if self.can_step_be_done(*step) {
        return Some(*step);
      }
    }
    None
  }

  fn can_step_be_done(&self, c: char) -> bool {
    if let Some(result) = self.steps.get(&c) {
      if !result.is_empty() {
        return result.iter().all(|todo| self.done.contains(todo));
      }
    }
    true
  }
}

fn solve_v1(mut steps: Instructions) -> String {
  while !steps.todo.is_empty() {
    let Some(next_step) = steps.get_next_step() else {
      panic!("No next step!?");
    };
    steps.todo.retain(|todo| todo != &next_step);
    steps.done.push(next_step);
  }

  steps.done.iter().collect()
}

fn solve_v2(mut steps: Instructions, workers: u8, duration: u8) -> u32 {
  let length = steps.todo.len();
  let mut timer: u32 = 0;
  let mut tasks: Vec<(u32, char)> = Vec::new();

  while steps.done.len() < length {
    tasks.retain(|(timer_up, step_doing)| {
      if timer < *timer_up {
        return true;
      }
      steps.done.push(*step_doing);
      false
    });
    while let Some(next_step) = steps.get_next_step() {
      if tasks.len() as u8 >= workers {
        break;
      }
      steps.todo.retain(|todo| todo != &next_step);
      let next_timer = timer + duration as u32 + (next_step as u32) - 64u32;
      tasks.push((next_timer, next_step));
    }

    timer += 1;
  }
  timer - 1
}

pub fn day_07_v1(input: impl Into<String>) -> String {
  let steps = Instructions::new(input.into());
  solve_v1(steps)
}

pub fn day_07_v2(input: impl Into<String>) -> String {
  let steps = Instructions::new(input.into());
  format!("{:?}", solve_v2(steps, 5, 60))
}

solvable!(day_07, day_07_v1, day_07_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "Step C must be finished before step A can begin.\n\
    Step C must be finished before step F can begin.\n\
    Step A must be finished before step B can begin.\n\
    Step A must be finished before step D can begin.\n\
    Step B must be finished before step E can begin.\n\
    Step D must be finished before step E can begin.\n\
    Step F must be finished before step E can begin.";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_07_v1(SAMPLE), "CABDFE");
  }

  #[test]
  fn works_with_samples_v2() {
    let steps = Instructions::new(SAMPLE.into());
    assert_eq!(solve_v2(steps, 2, 0), 15);
  }
}
