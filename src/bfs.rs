use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

pub struct BreadthFirstSearch<POS, NM>
where
  POS: Eq + Hash + Clone + std::fmt::Debug,
  NM: Fn(POS) -> Vec<POS>,
{
  starting: POS,
  pub ending: Option<POS>,
  queue: VecDeque<(POS, usize)>,
  pub visited: HashSet<POS>,
  pub depth: usize,
  next_moves: NM,
}

impl<POS, NM> BreadthFirstSearch<POS, NM>
where
  POS: Eq + Hash + Clone + std::fmt::Debug,
  NM: Fn(POS) -> Vec<POS>,
{
  pub fn new(starting: POS, next_moves: NM) -> Self {
    BreadthFirstSearch {
      starting: starting,
      ending: None,
      queue: VecDeque::new(),
      visited: HashSet::new(),
      depth: 0,
      next_moves: next_moves,
    }
  }

  fn traverse<TU>(&mut self, traverse_until: TU, stop_on_success: bool) -> &mut Self
  where
    TU: Fn(&POS, usize) -> bool,
  {
    self.queue.push_back((self.starting.clone(), 0));
    while let Some((position, depth)) = self.queue.pop_front() {
      if self.visited.contains(&position) {
        continue;
      }
      if traverse_until(&position, depth) {
        self.ending = Some(position);
        self.depth = depth;
        if stop_on_success {
          return self;
        } else {
          continue;
        }
      }
      self.visited.insert(position.clone());
      for next_move in (self.next_moves)(position) {
        if self.visited.contains(&next_move) {
          continue;
        }
        self.queue.push_back((next_move, depth + 1));
      }
    }

    self
  }

  pub fn shortest_path_to(&mut self, target_position: POS) -> usize {
    self.traverse_until_position(target_position.clone());
    self.depth
  }

  pub fn traverse_until_depth(&mut self, target_depth: usize) -> &mut Self {
    self.traverse(|_position, depth| depth > target_depth, true)
  }

  pub fn traverse_until_position(&mut self, target_position: POS) -> &mut Self {
    self.traverse(|position, _depth| *position == target_position, true)
  }

  pub fn traverse_until<TU>(&mut self, traverse_until: TU) -> &mut Self
  where
    TU: Fn(&POS) -> bool,
  {
    self.traverse(|position, _depth| traverse_until(position), true)
  }

  pub fn longest_path_until<TU>(&mut self, traverse_until: TU) -> &mut Self
  where
    TU: Fn(&POS) -> bool,
  {
    self.traverse(|position, _depth| traverse_until(position), false)
  }
}
