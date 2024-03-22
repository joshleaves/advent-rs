use std::fmt;
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone, Copy, Debug)]
enum BotOutput {
  Bot(usize),
  Output(usize),
}

impl fmt::Display for BotOutput {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BotOutput::Bot(id) => write!(f, "Bot({})", id),
      BotOutput::Output(id) => write!(f, "Output({})", id),
    }
  }
}

#[derive(Clone, Debug)]
struct Bot {
  values: Vec<usize>,
  low: BotOutput,
  high: BotOutput,
}

impl Bot {
  fn new() -> Self {
    Bot {
      values: vec![],
      low: BotOutput::Output(0),
      high: BotOutput::Output(0),
    }
  }

  fn add_chip(&mut self, chip: usize) {
    if self.values.len() == 2 {
      panic!("Cannot add chips here!");
    }
    self.values.push(chip);
    self.values.sort();
  }

  fn set_low_high(&mut self, low: BotOutput, high: BotOutput) {
    self.low = low;
    self.high = high;
  }

  fn extract_chips(&mut self) -> [(usize, BotOutput); 2] {
    if self.values.len() != 2 {
      panic!("No chips to extract");
    }

    let ex_low = (self.values[0], self.low);
    let ex_high = (self.values[1], self.high);
    self.values = vec![];
    [ex_low, ex_high]
  }
}

type BotHM = HashMap<usize, RefCell<Bot>>;

fn parse_input(input: impl Into<String>) -> BotHM {
  let mut bots = BotHM::new();

  for line in input.into().lines() {
    let parts: Vec<_> = line.split_whitespace().collect();
    match parts[0] {
      "value" => {
        let bot_id = parts[5].parse::<usize>().unwrap();
        let bot = bots.entry(bot_id).or_insert_with(|| Bot::new().into());

        let value = parts[1].parse::<usize>().unwrap();
        bot.borrow_mut().add_chip(value);
      }
      "bot" => {
        let bot_id = parts[1].parse::<usize>().unwrap();
        let bot = bots.entry(bot_id).or_insert_with(|| Bot::new().into());

        let low_id = parts[6].parse::<usize>().unwrap();
        let low_bo = match parts[5] {
          "bot" => BotOutput::Bot(low_id),
          "output" => BotOutput::Output(low_id),
          _ => panic!("Invalid instruction: {}", line),
        };
        let high_id = parts[11].parse::<usize>().unwrap();
        let high_bo = match parts[10] {
          "bot" => BotOutput::Bot(high_id),
          "output" => BotOutput::Output(high_id),
          _ => panic!("Invalid instruction: {}", line),
        };

        bot.borrow_mut().set_low_high(low_bo, high_bo)
      }
      _ => panic!("Invalid instruction: {}", line),
    }
  }

  bots
}

fn solve(input: impl Into<String>, chipstop: Option<Vec<usize>>) -> usize {
  let mut outs: HashMap<usize, Vec<usize>> = HashMap::new();
  let bots = parse_input(input);

  let mut full_bots: Vec<_> = bots
    .iter()
    .filter(|(_, bot)| bot.borrow_mut().values.len() == 2)
    .collect();
  while !full_bots.is_empty() {
    for (id, bot) in full_bots.iter() {
      if bot.borrow_mut().values.len() < 2 {
        continue;
      }
      if let Some(chipstopper) = &chipstop {
        if chipstopper == &bot.borrow_mut().values {
          return **id;
        }
      }
      for (chip, direction) in bot.borrow_mut().extract_chips().iter() {
        match direction {
          BotOutput::Bot(bot_id) => {
            let bot_out = bots.get(bot_id).unwrap();
            bot_out.borrow_mut().add_chip(*chip);
          }
          BotOutput::Output(out_id) => {
            let output = outs.entry(*out_id).or_default();
            output.push(*chip);
          }
        }
      }
    }
    full_bots = bots
      .iter()
      .filter(|(_, bot)| bot.borrow_mut().values.len() == 2)
      .collect();
  }

  outs
    .iter()
    .filter(|(id, _)| **id <= 2)
    .map(|(_, output)| output[0])
    .product()
}

const LOW_CHIP: usize = 17;
const HIGH_CHIP: usize = 61;

pub fn day_10_v1(input: impl Into<String>) -> usize {
  solve(input, Some(vec![LOW_CHIP, HIGH_CHIP]))
}

pub fn day_10_v2(input: impl Into<String>) -> usize {
  solve(input, None)
}

solvable!(day_10, day_10_v1, day_10_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "value 5 goes to bot 2\n\
      bot 2 gives low to bot 1 and high to bot 0\n\
      value 3 goes to bot 1\n\
      bot 1 gives low to output 1 and high to bot 0\n\
      bot 0 gives low to output 2 and high to output 0\n\
      value 2 goes to bot 2";
    assert_eq!(solve(sample_one, Some(vec![2, 5])), 2);
  }
}
