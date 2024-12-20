#[derive(Default)]
struct ParseCounter {
  input_values: Vec<u8>,
  counter: usize,
}

impl ParseCounter {
  fn new(input: &str) -> Self {
    let values = input
      .split_whitespace()
      .map(|f| f.parse::<u8>().unwrap())
      .collect();
    ParseCounter {
      input_values: values,
      counter: 0,
    }
  }

  fn next(&mut self) -> Option<u8> {
    let retval = self.input_values.get(self.counter);
    self.counter += 1;
    retval.copied()
  }
}

#[derive(Debug)]
struct Node {
  child_nodes: Vec<Node>,
  metadatas: Vec<u8>,
}

impl Node {
  fn new(pc: &mut ParseCounter) -> Self {
    let child_nodes_size = pc.next().expect("- Child nodes size");
    let metadatas_size = pc.next().expect("- Metadatas size");

    let mut child_nodes = Vec::<Node>::new();
    for _ in 0..child_nodes_size {
      child_nodes.push(Node::new(pc));
    }

    let mut metadatas = Vec::<u8>::new();
    for _ in 0..metadatas_size {
      metadatas.push(pc.next().expect("Should exist"));
    }

    Node {
      child_nodes,
      metadatas,
    }
  }

  fn metadatas_v1(self) -> u32 {
    let mut metadatas_total = 0;
    for child_node in self.child_nodes {
      metadatas_total += child_node.metadatas_v1();
    }
    for metadata in self.metadatas {
      metadatas_total += metadata as u32;
    }
    metadatas_total
  }

  fn metadatas_v2(&self) -> u32 {
    if self.child_nodes.is_empty() {
      return self.metadatas.iter().fold(0u32, |acc, x| acc + *x as u32);
    }
    let mut metadatas_total = 0;
    for metadata in self.metadatas.clone().into_iter() {
      if let Some(child_node) = self.child_nodes.get(metadata as usize - 1) {
        metadatas_total += child_node.metadatas_v2();
      }
    }
    metadatas_total
  }
}

pub fn day_08_v1(input: impl Into<String>) -> u32 {
  let mut parser = ParseCounter::new(&input.into());
  let root_node = Node::new(&mut parser);
  root_node.metadatas_v1()
}

pub fn day_08_v2(input: impl Into<String>) -> u32 {
  let mut parser = ParseCounter::new(&input.into());
  let root_node = Node::new(&mut parser);
  root_node.metadatas_v2()
}

solvable!(day_08, day_08_v1, day_08_v2, u32);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_08_v1(SAMPLE), 138);
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_08_v2(SAMPLE), 66);
  }
}
