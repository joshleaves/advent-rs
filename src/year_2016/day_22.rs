use crate::bfs::BreadthFirstSearch;
use itertools::Itertools;

struct Node {
  position: (u8, u8),
  size: usize,
  used: usize,
  available: usize,
}

impl Node {
  fn new(mut input: &str) -> Self {
    input = input.strip_prefix("/dev/grid/node-").unwrap();
    let parts: Vec<_> = input.split_whitespace().collect();
    let id_parts: Vec<_> = parts[0].split('-').collect();
    let x = id_parts[0][1..].parse::<u8>().unwrap();
    let y = id_parts[1][1..].parse::<u8>().unwrap();
    let size = parts[1]
      .strip_suffix('T')
      .unwrap()
      .parse::<usize>()
      .unwrap();
    let used = parts[2]
      .strip_suffix('T')
      .unwrap()
      .parse::<usize>()
      .unwrap();
    let available = parts[3]
      .strip_suffix('T')
      .unwrap()
      .parse::<usize>()
      .unwrap();

    Node {
      position: (x, y),
      size,
      used,
      available,
    }
  }
}

#[inline]
fn parse_nodes(input: &str) -> Vec<Node> {
  input
    .lines()
    .filter(|line| line.starts_with('/'))
    .map(Node::new)
    .collect_vec()
}

pub fn day_22_v1(input: impl Into<String>) -> usize {
  let nodes = parse_nodes(&input.into());
  let mut result: usize = 0;
  for combo in nodes.iter().combinations(2) {
    let lhn = combo[0];
    let rhn = combo[1];
    if lhn.used > 0 && lhn.used < rhn.available {
      result += 1;
    }
    if rhn.used > 0 && rhn.used < lhn.available {
      result += 1;
    }
  }

  result
}

pub fn day_22_v2(input: impl Into<String>) -> usize {
  let nodes = parse_nodes(&input.into());
  let max_x = nodes.iter().map(|node| node.position.0).max().unwrap();
  let max_y = nodes.iter().map(|node| node.position.1).max().unwrap();
  let open_node = nodes.iter().find(|node| node.used == 0).unwrap().position;
  let blacknodes = nodes
    .iter()
    .filter(|node| node.size > 100)
    .map(|node| node.position)
    .collect_vec();
  let mut bfs = BreadthFirstSearch::new(open_node, |position| {
    let mut results: Vec<(u8, u8)> = vec![];
    if position.1 > 0 && !blacknodes.contains(&(position.0, position.1 - 1)) {
      results.push((position.0, position.1 - 1));
    }
    if position.1 < max_y && !blacknodes.contains(&(position.0, position.1 + 1)) {
      results.push((position.0, position.1 + 1));
    }
    if position.0 > 0 && !blacknodes.contains(&(position.0 - 1, position.1)) {
      results.push((position.0 - 1, position.1));
    }
    if position.0 < max_x && !blacknodes.contains(&(position.0 + 1, position.1)) {
      results.push((position.0 + 1, position.1));
    }
    results
  });
  bfs.traverse_until_position((max_x - 1, 0));

  let mut answer = bfs.depth;
  answer += 1;
  answer += 5 * (max_x as usize - 1);
  answer
}

solvable!(day_22, day_22_v1, day_22_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE: &str = "root@ebhq-gridcenter# df -h\n\
    Filesystem            Size  Used  Avail  Use%\n\
    /dev/grid/node-x0-y0   10T    8T     2T   80%\n\
    /dev/grid/node-x0-y1   11T    6T     5T   54%\n\
    /dev/grid/node-x0-y2   32T   28T     4T   87%\n\
    /dev/grid/node-x1-y0    9T    7T     2T   77%\n\
    /dev/grid/node-x1-y1    8T    0T     8T    0%\n\
    /dev/grid/node-x1-y2   11T    7T     4T   63%\n\
    /dev/grid/node-x2-y0   10T    6T     4T   60%\n\
    /dev/grid/node-x2-y1    9T    8T     1T   88%\n\
    /dev/grid/node-x2-y2    9T    6T     3T   66%";

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_22_v2(SAMPLE), 7);
  }
}
