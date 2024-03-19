use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_world(input: &str) -> Vec<u8> {
  let mut result: Vec<u8> = vec![0];
  let mut chips: HashMap<&str, u8> = HashMap::new();
  let mut gens: HashMap<&str, u8> = HashMap::new();
  let mut alltypes: BTreeSet<&str> = BTreeSet::new();
  let re_chips: Regex = Regex::new(r"(\w+)-compatible").unwrap();
  let re_gens: Regex = Regex::new(r"(\w+) generator").unwrap();

  for (floor, line) in input.lines().enumerate() {
    for (_, [capture]) in re_chips.captures_iter(line).map(|c| c.extract()) {
      alltypes.insert(capture);
      chips.insert(capture, floor as u8);
    }
    for (_, [capture]) in re_gens.captures_iter(line).map(|c| c.extract()) {
      alltypes.insert(capture);
      gens.insert(capture, floor as u8);
    }
  }
  // alltypes.sort()
  for element in alltypes {
    result.push(*chips.get(element).unwrap());
    result.push(*gens.get(element).unwrap());
  }

  result
}

fn state_is_safe(state: &Vec<u8>) -> bool {
  let pairs: Vec<Vec<u8>> = state[1..].chunks(2).map(|s| s.into()).collect();
  for (idx_out, pair_out) in pairs.iter().enumerate() {
    if pair_out[0] == pair_out[1] {
      continue;
    }

    for (idx_in, pair_in) in pairs.iter().enumerate() {
      if idx_in == idx_out {
        continue;
      }
      if pair_out[0] == pair_in[1] {
        return false;
      }
    }
  }
  true
}

fn next_moves_from(current_state: &Vec<u8>, idx: usize, go_up: bool) -> Vec<Vec<u8>> {
  let mut new_moves: Vec<Vec<u8>> = vec![];
  let mut new_state: Vec<u8> = current_state.to_owned();
  if go_up {
    new_state[0] += 1;
    new_state[idx] += 1;
  } else {
    new_state[0] -= 1;
    new_state[idx] -= 1;
  }
  if state_is_safe(&new_state) {
    new_moves.push(new_state.to_vec());
  }
  for (idx_in, item_in) in current_state[idx + 1..].iter().enumerate() {
    if *item_in != current_state[0] {
      continue;
    }
    let mut new_state_in: Vec<u8> = current_state.to_owned();
    if go_up {
      new_state_in[0] += 1;
      new_state_in[idx] += 1;
      new_state_in[idx_in + idx + 1] += 1;
    } else {
      new_state_in[0] -= 1;
      new_state_in[idx] -= 1;
      new_state_in[idx_in + idx + 1] -= 1;
    }
    if state_is_safe(&new_state_in) {
      new_moves.push(new_state_in.to_vec());
    }
  }

  new_moves
}

fn next_moves(current_state: Vec<u8>) -> Vec<Vec<u8>> {
  let mut new_moves: Vec<Vec<u8>> = vec![];
  for (idx, item) in current_state.iter().enumerate() {
    if idx == 0 || *item != current_state[0] {
      continue;
    }
    if current_state[0] < 3 {
      let mut up_moves: Vec<Vec<u8>> = next_moves_from(&current_state, idx, true);
      new_moves.append(&mut up_moves);
    }
    if current_state[0] > 0 {
      let mut down_moves: Vec<Vec<u8>> = next_moves_from(&current_state, idx, false);
      new_moves.append(&mut down_moves);
    }
  }

  new_moves
}

fn search(states: Vec<Vec<u8>>, mut moves_tried: HashSet<Vec<u8>>, depth: u16) -> u16 {
  // println!("NEXT MOVES ({depth}) => {}", states.len());
  if states
    .iter()
    .any(|state| state.iter().all(|floor| *floor == 3))
  {
    return depth;
  }
  let mut next_states: Vec<Vec<u8>> = vec![];
  for state in states.iter().sorted().rev() {
    if moves_tried.contains(state) {
      continue;
    }
    moves_tried.insert(state.to_owned());
    for next_state in next_moves(state.to_vec()).iter() {
      if moves_tried.contains(next_state) {
        continue;
      }
      next_states.push(next_state.to_vec());
    }
  }

  search(next_states, moves_tried, depth + 1)
}

pub fn day_11_v1(input: impl Into<String>) -> u16 {
  let floors: Vec<u8> = parse_world(&input.into());
  let moves_tried: HashSet<Vec<u8>> = HashSet::new();
  search(vec![floors], moves_tried, 0)
}

pub fn day_11_v2(input: impl Into<String>) -> u16 {
  let mut floors: Vec<u8> = parse_world(&input.into());
  floors.append(&mut vec![0, 0, 0, 0]);
  let moves_tried: HashSet<Vec<u8>> = HashSet::new();
  search(vec![floors], moves_tried, 0)
}

solvable!(day_11, day_11_v1, day_11_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_ONE: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.\n\
    The second floor contains a hydrogen generator.\n\
    The third floor contains a lithium generator.\n\
    The fourth floor contains nothing relevant.";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_11_v1(SAMPLE_ONE), 11);
  }
}
