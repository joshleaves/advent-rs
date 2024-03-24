//! Advent of Code 2015: Day 12: JSAbacusFramework.io

fn traverse_node_value(input: &[u8], idx: usize) -> (i32, usize) {
  let mut total: i32 = 0;
  let mut min: i32 = 1;
  let mut length: usize = 0;

  loop {
    match input[idx + length] {
      b'-' => {
        min = -1;
      }
      b'0'..=b'9' => {
        total *= 10;
        total += input[idx + length] as i32 - 48;
      }
      _ => {
        if length == 0 {
          return (0, 1);
        }
        return (total * min, length - 1);
      }
    };
    length += 1;
  }
}

fn traverse_node_array(input: &[u8], idx: usize) -> (i32, usize) {
  let mut total: i32 = 0;
  let mut length: usize = 1;
  loop {
    match input[idx + length] {
      b']' => {
        return (total, length);
      }
      b'-'..=b'9' => {
        let (add_total, add_length) = traverse_node_value(input, idx + length);
        total += add_total;
        length += add_length;
      }
      b'{' => {
        let (add_total, add_length) = traverse_node_hash(input, idx + length);
        total += add_total;
        length += add_length;
      }
      b'[' => {
        let (add_total, add_length) = traverse_node_array(input, idx + length);
        total += add_total;
        length += add_length;
      }
      _ => {}
    }
    length += 1;
  }
}

fn traverse_node_hash(input: &[u8], idx: usize) -> (i32, usize) {
  let mut total: i32 = 0;
  let mut length: usize = 1;
  let mut no_red = true;
  loop {
    match input[idx + length] {
      b'}' => {
        if !no_red {
          total = 0;
        }
        return (total, length);
      }
      b'-'..=b'9' => {
        let (add_total, add_length) = traverse_node_value(input, idx + length);
        total += add_total;
        length += add_length;
      }
      b'{' => {
        let (add_total, add_length) = traverse_node_hash(input, idx + length);
        total += add_total;
        length += add_length;
      }
      b'[' => {
        let (add_total, add_length) = traverse_node_array(input, idx + length);
        total += add_total;
        length += add_length;
      }
      b'r' => {
        if no_red && input[idx + length - 1..=idx + length + 3] == *r#""red""#.as_bytes() {
          length += 3;
          no_red = false;
        }
      }
      _ => {}
    }
    length += 1;
  }
}

pub fn day_12_v1(input: impl Into<String>) -> i32 {
  let mut total: i32 = 0;
  let mut idx = 0;

  let letters: Vec<u8> = input.into().into_bytes();
  while idx < letters.len() - 1 {
    if let b'-'..=b'9' = letters[idx] {
      let (add_total, add_length) = traverse_node_value(&letters, idx);
      total += add_total;
      idx += add_length;
    }
    idx += 1;
  }

  total
}

pub fn day_12_v2(input: impl Into<String>) -> i32 {
  let bytes = input.into().as_bytes().to_vec();
  match bytes[0] {
    b'{' => traverse_node_hash(&bytes, 0).0,
    b'[' => traverse_node_array(&bytes, 0).0,
    _ => traverse_node_value(&bytes, 0).0,
  }
}

solvable!(day_12, day_12_v1, day_12_v2, i32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_samples_v1() {
    let sample_one: [(&str, i32); 8] = [
      (r#"[1,2,3]"#, 6),
      (r#"{"a":2,"b":4}"#, 6),
      (r#"[[[3]]]"#, 3),
      (r#"{"a":{"b":4},"c":-1}"#, 3),
      (r#"{"a":[-1,1]}"#, 0),
      (r#"[-1,{"a":1}]"#, 0),
      (r#"[]"#, 0),
      (r#"{}"#, 0),
    ];
    for (sample, result) in sample_one {
      assert_eq!(day_12_v1(sample), result);
    }
  }

  #[test]
  fn works_with_samples_v2() {
    let sample_two: [(&str, i32); 4] = [
      (r#"[1,2,3]"#, 6),
      (r#"[1,{"c":"red","b":2},3]"#, 4),
      (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
      (r#"[1,"red",5]"#, 6),
    ];
    for (sample, result) in sample_two {
      assert_eq!(day_12_v2(sample), result);
    }
  }
}
