use itertools::Itertools;

fn count_safe_tiles(mut input: Vec<bool>, rows: usize) -> usize {
  let linelen = input.len() - 1;
  let mut results = input.iter().filter(|b| **b).count();
  let mut new_rows = vec![false; linelen + 1];
  for _ in 1..rows {
    for (idx, _) in input.iter().enumerate() {
      if idx == 0 {
        if input[1] {
          results += 1;
        }
        new_rows[idx] = input[1];
      } else if idx == linelen {
        if input[linelen - 1] {
          results += 1;
        }
        new_rows[idx] = input[linelen - 1];
      } else {
        let value = input[idx - 1] == input[idx + 1];
        if value {
          results += 1;
        }
        new_rows[idx] = value;
      }
    }
    std::mem::swap(&mut input, &mut new_rows);
  }
  results
}

#[inline]
fn string2bools(input: &str) -> Vec<bool> {
  input.trim_end().chars().map(|c| c == '.').collect_vec()
}

pub fn day_18_v1(input: impl Into<String>) -> usize {
  count_safe_tiles(string2bools(&input.into()), 40)
}

pub fn day_18_v2(input: impl Into<String>) -> usize {
  count_safe_tiles(string2bools(&input.into()), 400_000)
}

solvable!(day_18, day_18_v1, day_18_v2, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_count_safe_tiles() {
    let sample: [(&str, usize, usize); 2] = [
      ("..^^.", 2, 6),
      (".^^.^.^^^^", 9, 38),
    ];
    for (input, rows, result) in sample {
      assert_eq!(count_safe_tiles(string2bools(input), rows + 1), result);
    }
  }
}
