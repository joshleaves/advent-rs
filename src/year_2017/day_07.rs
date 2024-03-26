use std::collections::HashMap;

use itertools::Itertools;

pub fn day_07_v1(input: impl Into<String>) -> String {
  let mut nodes: HashMap<&str, bool> = HashMap::new();
  let binding = input.into();
  for line in binding.lines() {
    let (left, rest) = line.split_once(' ').unwrap();
    nodes.entry(left).and_modify(|e| *e = !*e).or_default();
    if let Some((_, rest)) = rest.split_once(" -> ") {
      for path in rest.split(", ") {
        nodes.entry(path).and_modify(|e| *e = !*e).or_default();
      }
    }
  }
  nodes
    .iter()
    .filter_map(|(node, val)| if *val { None } else { Some(node) })
    .next()
    .unwrap()
    .to_string()
}

struct TowerNode {
  name: String,
  size: u32,
  children: Vec<String>,
}

struct Tower {
  nodes: HashMap<String, TowerNode>,
  values: HashMap<String, u32>,
}

impl TowerNode {
  fn new(input: &str) -> Self {
    let (name, rest) = input.split_once(' ').unwrap();
    let (num, rest) = rest.split_once(')').unwrap();
    let size: u32 = num.strip_prefix('(').unwrap().parse::<u32>().unwrap();
    let mut node = TowerNode {
      name: name.to_string(),
      size,
      children: vec![],
    };

    if let Some((_, rest)) = rest.split_once(" -> ") {
      for path in rest.split(", ") {
        node.children.push(path.to_string());
      }
    }
    node
  }
}

impl Tower {
  // pub fn to_s(&self) {
  //   let starter_node = self.bottom_node();
  //   self.print_node(0, starter_node);
  // }

  // fn print_node(&self, depth: usize, node: &TowerNode) {
  //   let prefix = "  ".repeat(depth);
  //   println!("{} -> {:5}: {}", prefix, self.get_value(&node.name), node.name);
  //   for child_name in node.children.iter() {
  //     let child_node = self.get(&child_name);
  //     self.print_node(depth + 1, child_node);
  //   }
  // }

  // fn bottom_node(&self) -> &TowerNode {
  //   let mut name = "";
  //   let mut size = 0;
  //   for (node_name, node_size) in self.values.iter() {
  //     if size < *node_size {
  //       name = node_name;
  //       size = *node_size;
  //     }
  //   }

  //   self.nodes.get(name).unwrap()
  // }

  fn get(&self, name: &str) -> &TowerNode {
    self.nodes.get(name).unwrap()
  }

  fn get_value(&self, name: &str) -> u32 {
    *self.values.get(name).unwrap()
  }

  fn size_of(&self, name: &str) -> u32 {
    let target_node = &self.nodes.get(name).unwrap();
    let mut size = target_node.size;
    for child in &target_node.children {
      size += self.size_of(child);
    }

    size
  }

  fn new(input: &str) -> Self {
    let mut tower = Tower {
      nodes: HashMap::new(),
      values: HashMap::new(),
    };
    for line in input.lines() {
      let tower_node = TowerNode::new(line);
      tower.nodes.insert(tower_node.name.clone(), tower_node);
    }
    for node_name in tower.nodes.keys() {
      let value = tower.size_of(node_name);
      tower.values.insert(node_name.clone(), value);
    }

    tower
  }
}

pub fn day_07_v2(input: impl Into<String>) -> String {
  let tower = Tower::new(&input.into());
  let bad_parent = tower
    .nodes
    .iter()
    .filter(|(_, node)| {
      if node.children.is_empty() {
        return false;
      }
      node
        .children
        .iter()
        .map(|name| tower.get_value(name))
        .sorted()
        .unique()
        .count()
        == 2
    })
    .sorted_by_key(|(name, _)| tower.get_value(name))
    .next()
    .unwrap()
    .1;

  let bad_weights: (u32, u32) = bad_parent
    .children
    .iter()
    .map(|n| tower.size_of(n))
    .sorted()
    .group_by(|v| *v)
    .into_iter()
    .map(|(key, group)| (key, group.count()))
    .sorted_by_key(|elt| elt.1)
    .map(|elt| elt.0)
    .collect_tuple()
    .unwrap();
  let bad_node = bad_parent
    .children
    .iter()
    .filter_map(|name| {
      if tower.get_value(name) == bad_weights.0 {
        Some(tower.get(name))
      } else {
        None
      }
    })
    .next()
    .unwrap();
  let weight_difference = bad_weights.0.abs_diff(bad_weights.1);
  (bad_node.size - weight_difference).to_string()

  // println!("WEIGHTS: {:?}_", bad_weights);
  //   // let bad_weight_node = bottom.children.iter()
  //   //   .find(|&name| tower.size_of(&name) == bad_weight_size)
  //   //   .unwrap();
  //   return "foo".to_string();
  // }

  // "foo".to_string()
}
solvable!(day_07, day_07_v1, day_07_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "pbga (66)\n\
    xhth (57)\n\
    ebii (61)\n\
    havc (66)\n\
    ktlj (57)\n\
    fwft (72) -> ktlj, cntj, xhth\n\
    qoyq (66)\n\
    padx (45) -> pbga, havc, qoyq\n\
    tknk (41) -> ugml, padx, fwft\n\
    jptl (61)\n\
    ugml (68) -> gyxo, ebii, jptl\n\
    gyxo (61)\n\
    cntj (57)";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_07_v1(SAMPLE), "tknk");
  }

  // #[test]
  // fn works_with_samples_v2() {
  //   assert_eq!(day_07_v2(""), 0);
  // }
}
