//! Advent of Code 2015: Day 13: Knights of the Dinner Table

use regex::Regex;
use std::collections::HashMap;

fn sort_tuple(lhp: u8, rhp: u8) -> (u8, u8) {
  if lhp < rhp {
    return (lhp, rhp);
  }
  (rhp, lhp)
}

fn parse_line(input: &str) -> ((String, String), i16) {
  let parser: Regex =
    Regex::new(r#"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)\."#).unwrap();
  let Some(captures) = parser.captures(input) else {
    panic!("Invalid input: {}", input);
  };
  let duet = (captures[1].to_string(), captures[4].to_string());
  let mut value: i16 = captures[3].parse::<i16>().unwrap();
  if captures[2] == *"lose" {
    value *= -1;
  }
  (duet, value)
}

fn parse_input(input: &str) -> (Vec<u8>, HashMap<(u8, u8), i16>) {
  let mut users: HashMap<String, u8> = HashMap::new();
  let mut paths: HashMap<(u8, u8), i16> = HashMap::new();

  fn get_id_for_user(users: &mut HashMap<String, u8>, user: String) -> u8 {
    match users.get(&user) {
      Some(user_id) => *user_id,
      None => {
        let new_id = users.len() as u8;
        users.insert(user, new_id);
        new_id
      }
    }
  }

  fn insert_happiness(paths: &mut HashMap<(u8, u8), i16>, users: (u8, u8), happiness: i16) {
    let new_happiness = match paths.get(&users) {
      Some(value) => value + happiness,
      None => happiness,
    };
    paths.insert(users, new_happiness);
  }

  for line in input.lines() {
    let ((lhp, rhp), happiness) = parse_line(line);
    let lhp_id = get_id_for_user(&mut users, lhp);
    let rhp_id = get_id_for_user(&mut users, rhp);
    let people = sort_tuple(lhp_id, rhp_id);
    insert_happiness(&mut paths, people, happiness);
  }
  let mut users_ids = users.values().cloned().collect::<Vec<u8>>();
  users_ids.sort();

  (users_ids, paths)
}

fn remove_user_id_from_vec(users: &[u8], needle: &u8) -> Vec<u8> {
  let mut new_vec = users.to_owned();
  new_vec.retain(|elt| elt != needle);
  new_vec
}

fn calculate_happiness(
  paths: &HashMap<(u8, u8), i16>,
  current_friend: &u8,
  friends_left: Vec<u8>,
  cur_hap: i16,
) -> i16 {
  fn get_happiness(paths: &HashMap<(u8, u8), i16>, lhp: &u8, rhp: &u8) -> i16 {
    let users = sort_tuple(*lhp, *rhp);
    match paths.get(&users) {
      Some(happiness) => *happiness,
      None => {
        panic!("No friendship for {}-{}", lhp, rhp);
      }
    }
  }

  if friends_left.is_empty() {
    return cur_hap + get_happiness(paths, current_friend, &0);
  }
  let mut best_happiness = 0;
  for user_id in friends_left.iter() {
    let friends_left = remove_user_id_from_vec(&friends_left, user_id);
    let add_hap = cur_hap + get_happiness(paths, current_friend, user_id);
    let nex_hap = calculate_happiness(paths, user_id, friends_left, add_hap);
    best_happiness = std::cmp::max(best_happiness, nex_hap);
  }
  best_happiness
}

pub fn day_13_v1(input: impl Into<String>) -> i16 {
  let (users_ids, paths) = parse_input(&input.into());
  let friends_left = remove_user_id_from_vec(&users_ids, &0);
  calculate_happiness(&paths, &0, friends_left, 0)
}

pub fn day_13_v2(input: impl Into<String>) -> i16 {
  let (mut users_ids, mut paths) = parse_input(&input.into());
  let my_user_id = users_ids.len() as u8;
  for lhp in 0..my_user_id {
    paths.insert((lhp, my_user_id), 0);
  }
  users_ids.push(my_user_id);
  let friends_left = remove_user_id_from_vec(&users_ids, &0);

  calculate_happiness(&paths, &0, friends_left, 0)
}

solvable!(day_13, day_13_v1, day_13_v2, i16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_input_lines() {
    let sample: [(&str, ((&str, &str), i16)); 12] = [
      (
        "Alice would gain 54 happiness units by sitting next to Bob.",
        (("Alice", "Bob"), 54),
      ),
      (
        "Alice would lose 79 happiness units by sitting next to Carol.",
        (("Alice", "Carol"), -79),
      ),
      (
        "Alice would lose 2 happiness units by sitting next to David.",
        (("Alice", "David"), -2),
      ),
      (
        "Bob would gain 83 happiness units by sitting next to Alice.",
        (("Bob", "Alice"), 83),
      ),
      (
        "Bob would lose 7 happiness units by sitting next to Carol.",
        (("Bob", "Carol"), -7),
      ),
      (
        "Bob would lose 63 happiness units by sitting next to David.",
        (("Bob", "David"), -63),
      ),
      (
        "Carol would lose 62 happiness units by sitting next to Alice.",
        (("Carol", "Alice"), -62),
      ),
      (
        "Carol would gain 60 happiness units by sitting next to Bob.",
        (("Carol", "Bob"), 60),
      ),
      (
        "Carol would gain 55 happiness units by sitting next to David.",
        (("Carol", "David"), 55),
      ),
      (
        "David would gain 46 happiness units by sitting next to Alice.",
        (("David", "Alice"), 46),
      ),
      (
        "David would lose 7 happiness units by sitting next to Bob.",
        (("David", "Bob"), -7),
      ),
      (
        "David would gain 41 happiness units by sitting next to Carol.",
        (("David", "Carol"), 41),
      ),
    ];
    for (input, ((lhp, rhp), happiness)) in sample {
      assert_eq!(
        parse_line(input),
        ((lhp.to_string(), rhp.to_string()), happiness)
      );
    }
  }

  #[test]
  fn works_with_samples_v1() {
    let sample = r#"Alice would gain 54 happiness units by sitting next to Bob.\n\
      Alice would lose 79 happiness units by sitting next to Carol.\n\
      Alice would lose 2 happiness units by sitting next to David.\n\
      Bob would gain 83 happiness units by sitting next to Alice.\n\
      Bob would lose 7 happiness units by sitting next to Carol.\n\
      Bob would lose 63 happiness units by sitting next to David.\n\
      Carol would lose 62 happiness units by sitting next to Alice.\n\
      Carol would gain 60 happiness units by sitting next to Bob.\n\
      Carol would gain 55 happiness units by sitting next to David.\n\
      David would gain 46 happiness units by sitting next to Alice.\n\
      David would lose 7 happiness units by sitting next to Bob.\n\
      David would gain 41 happiness units by sitting next to Carol.\n\"#;
    assert_eq!(day_13_v1(sample), 330);
  }
}
