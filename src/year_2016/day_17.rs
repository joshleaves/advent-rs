use crate::bfs::BreadthFirstSearch;
use md5::{digest::core_api::CoreWrapper, Digest, Md5, Md5Core};
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone)]
struct Day17Position {
  position: (usize, usize),
  path: String,
}

impl Day17Position {
  fn new(position: (usize, usize)) -> Self {
    Self {
      position,
      path: String::new(),
    }
  }

  pub fn next_moves_for_hasher(&self, mut hasher: CoreWrapper<Md5Core>) -> Vec<Self> {
    hasher.update(self.path.clone());
    let positions: &[u8] = &hasher.finalize()[0..=1];
    let mut next_moves = vec![];
    if ((positions[0] & 0xF0) >> 4) >= 11 {
      if let Some(move_up) = self.move_up() {
        next_moves.push(move_up);
      }
    }
    if (positions[0] & 0x0F) >= 11 {
      if let Some(move_down) = self.move_down() {
        next_moves.push(move_down);
      }
    }
    if ((positions[1] & 0xF0) >> 4) >= 11 {
      if let Some(move_left) = self.move_left() {
        next_moves.push(move_left);
      }
    }
    if (positions[1] & 0x0F) >= 11 {
      if let Some(move_right) = self.move_right() {
        next_moves.push(move_right);
      }
    }
    next_moves
  }

  fn move_up(&self) -> Option<Day17Position> {
    if self.position.1 == 0 {
      return None;
    }
    Some(Day17Position {
      position: (self.position.0, self.position.1 - 1),
      path: self.path.clone() + "U",
    })
  }

  fn move_down(&self) -> Option<Day17Position> {
    if self.position.1 == 3 {
      return None;
    }
    Some(Day17Position {
      position: (self.position.0, self.position.1 + 1),
      path: self.path.clone() + "D",
    })
  }

  fn move_left(&self) -> Option<Day17Position> {
    if self.position.0 == 0 {
      return None;
    }
    Some(Day17Position {
      position: (self.position.0 - 1, self.position.1),
      path: self.path.clone() + "L",
    })
  }

  fn move_right(&self) -> Option<Day17Position> {
    if self.position.0 == 3 {
      return None;
    }
    Some(Day17Position {
      position: (self.position.0 + 1, self.position.1),
      path: self.path.clone() + "R",
    })
  }
}

impl PartialEq for Day17Position {
  fn eq(&self, _other: &Self) -> bool {
    // self.position == other.position && sel
    false
  }
}

impl Hash for Day17Position {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.position.hash(state);
  }
}

pub fn day_17_v1(input: impl Into<String>) -> String {
  let mut hasher = Md5::new();
  hasher.update(input.into().trim_end());
  let starter = Day17Position::new((0, 0));
  let mut bfs = BreadthFirstSearch::new(starter, |curpos| {
    curpos.next_moves_for_hasher(hasher.clone())
  });
  bfs.traverse_until(|position| position.position == (3, 3));

  bfs.ending.unwrap().path
}

pub fn day_17_v2(input: impl Into<String>) -> String {
  let mut hasher = Md5::new();
  hasher.update(input.into().trim_end());
  let starter = Day17Position::new((0, 0));
  let mut bfs = BreadthFirstSearch::new(starter, |curpos| {
    curpos.next_moves_for_hasher(hasher.clone())
  });
  bfs.longest_path_until(|position| position.position == (3, 3));

  bfs.depth.to_string()
}

solvable!(day_17, day_17_v1, day_17_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, &str); 3] = [
      ("ihgpwlah", "DDRRRD"),
      ("kglvqrro", "DDUDRLRRUDRD"),
      ("ulqzkmiv", "DRURDRUDDLLDLUURRDULRLDUUDDDRR"),
    ];
    for (sample, result) in sample_one {
      println!("TEST => {}", sample);
      assert_eq!(day_17_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, &str); 3] = [
      ("ihgpwlah", "370"),
      ("kglvqrro", "492"),
      ("ulqzkmiv", "830"),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_17_v2(sample), result);
    }
  }
}
