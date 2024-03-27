use itertools::Itertools;
use std::collections::HashSet;

#[derive(Default)]
struct Village {
  groups: Vec<HashSet<usize>>,
}

impl Village {
  fn group_with(&self, needle: usize) -> Option<usize> {
    self.groups.iter().position(|group| group.contains(&needle))
  }

  fn parse_node(input: &str) -> Vec<usize> {
    let (root, links) = input.split_once(" <-> ").unwrap();
    let mut new_node = Vec::from([root.parse::<usize>().unwrap()]);
    for link in links.split(", ") {
      new_node.push(link.parse::<usize>().unwrap());
    }
    new_node.sort();
    new_node
  }

  pub fn add_node(&mut self, input: &str) {
    let new_node = Village::parse_node(input);
    let mut final_node = HashSet::from_iter(new_node.iter().cloned());
    let groups: Vec<_> = new_node
      .iter()
      .filter_map(|add_node| self.group_with(*add_node))
      .sorted()
      .unique()
      .rev()
      .collect();

    groups.iter().for_each(|node_idx| {
      let extract = &self.groups.remove(*node_idx);
      final_node.extend(&mut extract.iter());
    });
    self.groups.push(final_node);
  }

  pub fn group_zero(&mut self) -> &HashSet<usize> {
    let idx = self.group_with(0).unwrap();
    &self.groups[idx]
  }
}

pub fn day_12_v1(input: impl Into<String>) -> usize {
  let mut village = Village::default();
  for line in input.into().lines() {
    village.add_node(line);
  }
  village.group_zero().len()
}

pub fn day_12_v2(input: impl Into<String>) -> usize {
  let mut village = Village::default();
  for line in input.into().lines() {
    village.add_node(line);
  }
  village.groups.len()
}

solvable!(day_12, day_12_v1, day_12_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "0 <-> 2\n\
    1 <-> 1\n\
    2 <-> 0, 3, 4\n\
    3 <-> 2, 4\n\
    4 <-> 2, 3, 6\n\
    5 <-> 6\n\
    6 <-> 4, 5";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_12_v1(SAMPLE), 6);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_12_v2(SAMPLE), 2);
  }
}
