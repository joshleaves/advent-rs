use md5::{Digest, Md5};

const HEX_TABLE: [char; 16] = [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn day_05_v1(input: impl Into<String>) -> String {
  let mut hasher = Md5::new();
  hasher.update(input.into().lines().next().unwrap());
  let mut starter: u32 = 0;
  let mut result: String = String::new();
  loop {
    let mut hasher_num = hasher.clone();
    hasher_num.update(starter.to_string());
    let hash = hasher_num.finalize();
    if hash[..2] == [0, 0] && hash[2] <= 15 {
      result.push(HEX_TABLE[hash[2] as usize]);
      if result.len() == 8 {
        return result;
      }
    }
    starter += 1;
  }
}

pub fn day_05_v2(input: impl Into<String>) -> String {
  let mut hasher = Md5::new();
  hasher.update(input.into().lines().next().unwrap());
  let mut starter: u32 = 0;
  let mut result: [char; 8] = [
    '_', '_', '_', '_', '_', '_', '_', '_',
  ];
  let mut added: u16 = 0;
  loop {
    let mut hasher_num = hasher.clone();
    hasher_num.update(starter.to_string());
    let hash = hasher_num.finalize();
    if hash[..2] == [0, 0] && hash[2] <= 15 {
      let pos = hash[2] as usize;
      let value = (hash[3] / 16) as usize;
      if pos < 8 && result[pos as usize] == '_' {
        result[pos as usize] = HEX_TABLE[value as usize];
        added += 1;
        if added == 8 {
          return result.iter().collect::<String>();
        }
      }
    }
    starter += 1;
  }
}

solvable!(day_05, day_05_v1, day_05_v2, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    assert_eq!(day_05_v1("abc"), "18f47a30");
  }

  #[test]
  fn works_with_samples_v2() {
    assert_eq!(day_05_v2("abc"), "05ace8e3");
  }
}
