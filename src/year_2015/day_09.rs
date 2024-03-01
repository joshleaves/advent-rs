use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn id_for_city<'a>(cities: &mut HashMap<&'a str, u8>, city: &'a str) -> u8 {
  return match cities.get(city) {
    Some(city_id) => *city_id,
    None => {
      let new_id = cities.len() as u8;
      cities.insert(city, new_id);
      new_id
    }
  };
}

fn parse_input<'a>(input: &str) -> (Vec<u8>, HashMap<(u8, u8), u16>) {
  let mut cities: HashMap<&str, u8> = HashMap::new();
  let mut paths: HashMap<(u8, u8), u16> = HashMap::new();

  let re = Regex::new(r#"(\w+) to (\w+) = (\d+)"#).unwrap();
  for (_, [from, to, path]) in re.captures_iter(input).map(|c| c.extract()) {
    let from_id = id_for_city(&mut cities, from);
    let to_id = id_for_city(&mut cities, to);
    let Ok(path_size) = path.parse::<u16>() else {
      panic!("Invalid number: {}", path);
    };
    paths.insert((from_id, to_id), path_size);
    paths.insert((to_id, from_id), path_size);
  }

  let mut cities_ids = cities.values().cloned().collect::<Vec<u8>>();
  cities_ids.sort();
  (cities_ids, paths)
}

fn next_cities_ids<'a>(input: &Vec<u8>, needle: &u8) -> Vec<u8> {
  let mut new_vec = input.clone();
  new_vec.retain(|&elt| elt != *needle);
  return new_vec;
}

fn try_best_path<'a, STP, CMP>(
  paths: &HashMap<(u8, u8), u16>,
  current_city: &u8,
  cities: Vec<u8>,
  stopper_func: &STP,
  better_path: &CMP,
  current_path: &'a u16,
  current_best: &'a u16,
) -> u16
where
  STP: Fn(u16, u16, usize) -> bool,
  CMP: Fn(u16, u16) -> u16,
{
  if stopper_func(*current_best, *current_path, cities.len()) {
    return *current_path;
  }
  let mut best_path = *current_best;
  for city_id in cities.iter() {
    let new_path = match paths.get(&(*current_city, *city_id)) {
      Some(distance) => distance,
      None => panic!("Invalid tuple: ({},{})", current_city, city_id),
    };
    let next_path = current_path + new_path;
    let new_best = try_best_path(
      &paths,
      city_id,
      next_cities_ids(&cities, city_id),
      stopper_func,
      better_path,
      &next_path,
      current_best,
    );
    best_path = better_path(best_path, new_best);
  }
  best_path
}

pub fn day_09_v1<'a>(input: impl Into<String>) -> u16 {
  let (cities_ids, paths) = parse_input(&input.into());
  let mut best_path = u16::MAX;
  for city_id in cities_ids.iter() {
    let new_best = try_best_path(
      &paths,
      city_id,
      next_cities_ids(&cities_ids, city_id),
      &|best, cur, cities_len| best < cur || cities_len == 0,
      &cmp::min,
      &0,
      &best_path,
    );
    best_path = cmp::min(best_path, new_best);
  }
  best_path
}

pub fn day_09_v2<'a>(input: impl Into<String>) -> u16 {
  let (cities_ids, paths) = parse_input(&input.into());
  let mut best_path = 0;
  for city_id in cities_ids.iter() {
    let new_best = try_best_path(
      &paths,
      city_id,
      next_cities_ids(&cities_ids, city_id),
      &|_best, _cur, cities_len| cities_len == 0,
      &cmp::max,
      &0,
      &best_path,
    );
    best_path = cmp::max(best_path, new_best);
  }
  best_path
}

solvable!(day_09, day_09_v1, day_09_v2, u16);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one = "London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141";
    assert_eq!(day_09_v1(sample_one), 605);
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two = "London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141";
    assert_eq!(day_09_v2(sample_two), 982);
  }
}
